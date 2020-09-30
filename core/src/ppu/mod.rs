pub mod palette;
mod sprite;
mod tile;

use palette::*;
use sprite::{OAM_BYTE_SIZE, Sprite};
use tile::{Tile, TILE_BYTES};
use crate::cpu::clock::ModeTypes;
use crate::utils::*;
use std::ops::Range;

// =============
// = Constants =
// =============

// VRAM registers
const LCDC: u16                    = 0xFF40;
const STAT: u16                    = 0xFF41;
const SCY: u16                     = 0xFF42;
const SCX: u16                     = 0xFF43;
const LY: u16                      = 0xFF44;
const LYC: u16                     = 0xFF45;
// 0xFF46 is DMA transfer, handled by Bus
const BGP: u16                     = 0xFF47;
const OBP0: u16                    = 0xFF48;
const OBP1: u16                    = 0xFF49;
const WY: u16                      = 0xFF4A;
const WX: u16                      = 0xFF4B;
const VBK: u16                     = 0xFF4F;

// CGB Palette registers
const BGPI: u16                    = 0xFF68;
const BGPD: u16                    = 0xFF69;
const OBPI: u16                    = 0xFF6A;
const OBPD: u16                    = 0xFF6B;

// VRAM ranges
const VRAM_START: u16              = 0x8000;
const VRAM_END: u16                = 0x9FFF;
const OAM_START: u16               = 0xFE00;
const OAM_END: u16                 = 0xFE9F;
const IO_START: u16                = 0xFF00;
const IO_END: u16                  = 0xFF7F;
const TILE_SET: u16                = VRAM_START;
const TILE_SET_END: u16            = 0x97FF;

const TILE_MAP_0_RANGE: Range<usize> = (0x9800 - VRAM_START as usize)..(0x9C00 - VRAM_START as usize);
const TILE_MAP_1_RANGE: Range<usize> = (0x9C00 - VRAM_START as usize)..(0xA000 - VRAM_START as usize);

// General constants
const MAP_SIZE: usize = 32; // In tiles
const MAP_PIXELS: usize = MAP_SIZE * TILESIZE; // In pixels
const VRAM_SIZE: usize = (VRAM_END - VRAM_START + 1) as usize;
const IO_SIZE: usize = (IO_END - IO_START + 1) as usize;
const TILE_NUM: usize = 384;
const OAM_SPR_NUM: usize = 40;
const SPR_PER_LINE: usize = 10;
const CGB_BG_PAL_DATA_SIZE: usize = 64; // 8 palettes, 4 colors per palette, 2 bytes per color
const CGB_SPR_PAL_DATA_SIZE: usize = 64;

// Register bit constants
const BG_DISP_BIT: u8           = 0;
const SPR_DISP_BIT: u8          = 1;
const SPR_SIZE_BIT: u8          = 2;
const BG_TILE_MAP_BIT: u8       = 3;
const BG_WNDW_TILE_DATA_BIT: u8 = 4;
const WNDW_DISP_BIT: u8         = 5;
const WNDW_TILE_MAP_BIT: u8     = 6;
const LCD_DISP_BIT: u8          = 7;

const LYC_LY_FLAG_BIT: u8       = 2;
// const HBLANK_INTERRUPT_BIT: u8 =    3;
// const VBLANK_INTERRUPT_BIT: u8 =    4;
// const OAM_INTERRUPT_BIT: u8 =       5;
const LYC_LY_INTERRUPT_BIT: u8  = 6;

pub struct PPU {
    vram: [u8; VRAM_SIZE],
    vram_bank: usize,
    io: [u8; IO_SIZE],
    screen_buffer: [u8; DISP_SIZE],
    tiles: [Tile; 2 * TILE_NUM], // CGB can have two banks of tiles
    oam: [Sprite; OAM_SPR_NUM],
    last_wndw_line: Option<u8>,
    cgb_bg_pal_data: [u8; CGB_BG_PAL_DATA_SIZE],
    cgb_spr_pal_data: [u8; CGB_SPR_PAL_DATA_SIZE],
    sys_pal: Palettes,
}

impl Default for PPU {
    fn default() -> Self {
        Self::new()
    }
}

impl PPU {
    // ==================
    // = Public methods =
    // ==================
    pub fn new() -> PPU {
        PPU {
            vram: [0; VRAM_SIZE],
            vram_bank: 0,
            io: [0; IO_SIZE],
            screen_buffer: [0; DISP_SIZE],
            tiles: [Tile::new(); 2 * TILE_NUM],
            oam: [Sprite::new(); OAM_SPR_NUM],
            last_wndw_line: None,
            cgb_bg_pal_data: [0; CGB_BG_PAL_DATA_SIZE],
            cgb_spr_pal_data: [0; CGB_SPR_PAL_DATA_SIZE],
            sys_pal: Palettes::GRAYSCALE,
        }
    }

    /// ```
    /// Write VRAM
    ///
    /// Write value to specified address in VRAM
    ///
    /// Inputs:
    ///     Address to write to (u16)
    ///     Value to write (u8)
    ///     System mode (GB)
    /// ```
    pub fn write_vram(&mut self, addr: u16, val: u8, mode: GB) {
        // TODO: Need to check for valid clock mode before writes
        // if !self.is_valid_status(addr) {
        //     return;
        // }

        match addr {
            OAM_START..=OAM_END => {
                let relative_addr = addr - OAM_START;
                let spr_num = relative_addr / OAM_BYTE_SIZE;
                let byte_num = relative_addr % OAM_BYTE_SIZE;
                self.oam[spr_num as usize].set_byte(byte_num, val);
            },
            TILE_SET..=TILE_SET_END => {
                let offset = addr - TILE_SET;
                let tile_num = offset / TILE_BYTES + (self.vram_bank * TILE_NUM) as u16;
                let byte_num = offset % TILE_BYTES;
                self.tiles[tile_num as usize].set_byte(byte_num, val);
            },
            VRAM_START..=VRAM_END => {
                let vram_addr = addr - VRAM_START;
                self.vram[vram_addr as usize] = val;
            },
            IO_START..=IO_END => {
                if mode == GB::CGB || mode == GB::CGB_DMG {
                    match addr {
                        BGPD => {
                            self.write_cgb_bg_color(val);
                        },
                        OBPD => {
                            self.write_cgb_spr_color(val);
                        },
                        VBK => {
                            self.set_vram_bank(val);
                        },
                        _ => {
                            self.write_io(addr, val);
                        }
                    }
                } else {
                    self.write_io(addr, val);
                }
            },
            _ => {
                // Unused, do nothing
            }
        }
    }

    /// ```
    /// Read VRAM
    ///
    /// Read value from given address in VRAM
    ///
    /// Input:
    ///     Address to read from (u16)
    ///     System mode (GB)
    ///
    /// Output:
    ///     Value at given address (u8)
    /// ```
    pub fn read_vram(&self, addr: u16, mode: GB) -> u8 {
        match addr {
            OAM_START..=OAM_END => {
                let relative_addr = addr - OAM_START;
                let spr_num = relative_addr / OAM_BYTE_SIZE;
                let byte_num = relative_addr % OAM_BYTE_SIZE;
                self.oam[spr_num as usize].get_byte(byte_num)
            },
            TILE_SET..=TILE_SET_END => {
                let offset = addr - TILE_SET;
                let tile_num = offset / TILE_BYTES + (self.vram_bank * TILE_NUM) as u16;
                let byte_num = offset % TILE_BYTES;
                self.tiles[tile_num as usize].get_byte(byte_num)
            },
            VRAM_START..=VRAM_END => {
                let vram_addr = addr - VRAM_START;
                self.vram[vram_addr as usize]
            },
            IO_START..=IO_END => {
                if mode == GB::CGB_DMG || mode == GB::CGB {
                    match addr {
                        BGPD => {
                            self.read_cgb_bg_color()
                        },
                        OBPD => {
                            self.read_cgb_spr_color()
                        },
                        _ => {
                            self.read_io(addr)
                        }
                    }
                } else {
                    self.read_io(addr)
                }
            },
            _ => {
                // Unused, do nothing
                0
            }
        }
    }

    /// ```
    /// Set LY register
    ///
    /// Sets the value at the LY RAM address
    ///
    /// Input:
    ///     Value to write (u8)
    ///
    /// Output:
    ///     Whether values in LY and LYC registers are equal (bool)
    /// ```
    pub fn set_ly(&mut self, line: u8) -> bool {
        let old_ly = self.read_io(LY);
        if old_ly != line {
            // If we are in a new frame, reset window layer line
            if line == 0 {
                self.last_wndw_line = None;
            }

            self.write_io(LY, line);

            let mut stat = self.read_io(STAT);
            if self.read_io(LY) == self.read_io(LYC) {
                // If LY and LYC are equal:
                // - Set coincidence bit in STAT register
                // - Trigger LCDC status interrupt if enabled
                stat.set_bit(LYC_LY_FLAG_BIT);
                self.write_io(STAT, stat);
                return stat.get_bit(LYC_LY_INTERRUPT_BIT);
            } else {
                stat.clear_bit(LYC_LY_FLAG_BIT);
                self.write_io(STAT, stat);
            }
        }

        false
    }

    /// ```
    /// Render scanline
    ///
    /// Renders specified scanline to buffer
    ///
    /// Input:
    ///     GB hardware type
    /// ```
    pub fn render_scanline(&mut self, mode: GB) {
        // Render current scanline
        let line = self.read_io(LY);
        let mut pixel_row = [0xFF; SCREEN_WIDTH * COLOR_CHANNELS];

        if self.is_bkgd_dspl() {
            self.render_background_line(&mut pixel_row, line, mode);
        }

        if self.is_wndw_dspl(mode) {
            self.render_wndw_line(&mut pixel_row, line, mode);
        }

        if self.is_sprt_dspl() {
            self.render_sprite_line(&mut pixel_row, line, mode);
        }

        // Copy this line of pixels into overall screen buffer
        let start_index = line as usize * (SCREEN_WIDTH * COLOR_CHANNELS);
        let end_index = (line + 1) as usize * (SCREEN_WIDTH * COLOR_CHANNELS);
        self.screen_buffer[start_index..end_index].copy_from_slice(&pixel_row);
    }

    /// ```
    /// Set status
    ///
    /// Sets the current value of the status register ($FF41)
    ///
    /// Input:
    ///     Current clock mode (u8)
    /// ```
    pub fn set_status(&mut self, mode: u8) {
        let mut stat = self.read_io(STAT);
        stat &= 0b1111_1100;
        stat |= mode;
        self.write_io(STAT, stat);
    }

    /// ```
    /// Render screen
    ///
    /// Renders the current screen
    ///
    /// Output:
    ///     Array of pixels to draw ([u8])
    /// ```
    pub fn render_screen(&self) -> [u8; DISP_SIZE] {
        let mut map_array = [0xFF; DISP_SIZE];
        if self.is_lcd_dspl() {
            map_array.copy_from_slice(&self.screen_buffer);
        }
        map_array
    }

    /// ```
    /// Set system palette
    ///
    /// Set which color palette we want to use
    ///
    /// Input:
    ///     Palette (Palettes)
    /// ```
    pub fn set_sys_pal(&mut self, pal: Palettes) {
        self.sys_pal = pal;
    }

    // ===================
    // = Private methods =
    // ===================

    /// ```
    /// Render Background Line
    ///
    /// Renders the given scanline of the background layer
    ///
    /// Inputs:
    ///     Array to load pixel data into (&[u8])
    ///     Scanline to render (u8)
    ///     Hardware type (GB)
    /// ```
    fn render_background_line(&self, pixel_row: &mut [u8], line: u8, mode: GB) {
        let tile_map = self.get_bkgd_tile_map();
        // TODO: This is not ideal. Someday, I'd like to not have this variable if we aren't DMG
        let dmg_pal = get_sys_pal(self.sys_pal);
        let pal_indices = self.get_dmg_bkgd_palette();
        let screen_coords = self.get_scroll_coords();

        // Get the row of tiles containing our scanline
        let y = ((screen_coords.y as usize) + (line as usize)) % MAP_PIXELS;
        let row = y % TILESIZE;
        let start_x = screen_coords.x as usize;
        for x in 0..SCREEN_WIDTH {
            // Get coords for current tile
            let map_x = ((start_x + x) % MAP_PIXELS) / TILESIZE;
            let map_y = y / TILESIZE;
            let index = map_y * MAP_SIZE + map_x;
            // The tile indexes in the second tile pattern table ($8800-97ff) are signed
            let mut tile_index = if self.get_bkgd_wndw_tile_set_index() == 0 {
                (256 + (tile_map[index] as i8 as isize)) as usize
            } else {
                tile_map[index] as usize
            };
            tile_index += self.vram_bank * TILE_NUM;

            let tile = &self.tiles[tile_index];
            let col = (start_x + x) % TILESIZE;
            let pixel = tile.get_row(row)[col] as usize;
            if mode == GB::CGB {
                let raw_color = merge_bytes(self.cgb_bg_pal_data[pixel + 1], self.cgb_bg_pal_data[pixel]);
                let color = gbc2rgba(raw_color);
                for i in 0..COLOR_CHANNELS {
                    pixel_row[COLOR_CHANNELS * x + i] = color[i];
                }
            } else {
                let color = dmg_pal[pal_indices[pixel] as usize];
                for i in 0..COLOR_CHANNELS {
                    pixel_row[COLOR_CHANNELS * x + i] = color[i];
                }
            }
        }
    }

    /// ```
    /// Render Window Line
    ///
    /// Renders the given scanline of the window layer
    ///
    /// Inputs:
    ///     Array to load pixel data into (&[u8])
    ///     Scanline to render (u8)
    /// ```
    fn render_wndw_line(&mut self, pixel_row: &mut [u8], line: u8, mode: GB) {
        let wndw_coords = self.get_wndw_coords();
        // See below for why this is needed
        let line = if self.last_wndw_line.is_none() { line } else { self.last_wndw_line.unwrap() + 1 };

        // If window isn't drawn on this scanline, return
        if (wndw_coords.y > line) || (wndw_coords.x > SCREEN_WIDTH as u8) {
            return;
        }

        let wndw_map = self.get_wndw_tile_map();
        let dmg_pal = get_sys_pal(self.sys_pal);
        let pal_indices = self.get_dmg_bkgd_palette();

        // Get the row of tiles containing our scanline
        let y = (line - wndw_coords.y) as usize;
        let row = y % TILESIZE;
        let map_y = y / TILESIZE;
        let start_x = wndw_coords.x as usize;
        for x in start_x..SCREEN_WIDTH {
            // Get coords for current tile
            let map_x = ((x - start_x) % MAP_PIXELS) / TILESIZE;
            let index = map_y * MAP_SIZE + map_x;
            // The tile indexes in the second tile pattern table ($8800-97ff) are signed
            let mut tile_index = if self.get_bkgd_wndw_tile_set_index() == 0 {
                (256 + (wndw_map[index] as i8 as isize)) as usize
            } else {
                wndw_map[index] as usize
            };
            tile_index += self.vram_bank * TILE_NUM;
            let tile = &self.tiles[tile_index];
            let col = (x - start_x) % TILESIZE;
            let pixel = tile.get_row(row)[col] as usize;
            if mode == GB::CGB {
                let raw_color = merge_bytes(self.cgb_bg_pal_data[pixel + 1], self.cgb_bg_pal_data[pixel]);
                let color = gbc2rgba(raw_color);
                for i in 0..COLOR_CHANNELS {
                    pixel_row[COLOR_CHANNELS * x + i] = color[i];
                }
            } else {
                let color = dmg_pal[pal_indices[pixel] as usize];
                for i in 0..COLOR_CHANNELS {
                    pixel_row[COLOR_CHANNELS * x + i] = color[i];
                }
            }
        }

        // The window layer has an odd edge case
        // If it is disabled mid-frame and then re-enabled, it continues rendering where it was
        // Thus, we need to keep track of what scanline we finished rendering in case we are disabled
        // And continue there if re-enabled this frame (and reset this value at start of next)
        self.last_wndw_line = Some(line);
    }

    /// ```
    /// Render Sprite Line
    ///
    /// Renders the given scanline of the sprite layer
    ///
    /// Inputs:
    ///     Array to load pixel data into (&[u8])
    ///     Scanline to render (u8)
    ///     GB hardware type
    /// ```
    fn render_sprite_line(&self, pixel_row: &mut [u8], line: u8, mode: GB) {
        // Iterate through every sprite
        let sorted_sprites = self.sort_sprites();
        let is_8x16 = self.spr_are_8x16();
        let mut sprites_drawn = 0;
        let dmg_pal = get_sys_pal(self.sys_pal);
        for spr in sorted_sprites {
            if !spr.contains_scanline(line, is_8x16) || !spr.is_onscreen() {
                continue;
            }

            sprites_drawn += 1;
            // System only allows finite number of sprites drawn per line
            // If we hit threshold, no more sprites can be drawn on this line
            if sprites_drawn > SPR_PER_LINE {
                break;
            }

            let pal_indices = self.get_dmg_spr_palette(spr.is_pal_0());
            let mut above_bg = spr.is_above_bkgd();
            if mode == GB::CGB {
                let lcd_control = self.read_io(LCDC);
                above_bg |= lcd_control.get_bit(BG_DISP_BIT);
            }

            let (top_x, top_y) = spr.get_coords();
            // Get which row in the sprite we're drawing
            let row = ((line as i16) - top_y) as usize;
            // If sprite is Y-flipped, adjust row
            let row = if spr.is_y_flip() {
                if is_8x16 {
                    (2 * TILESIZE) - row - 1
                } else {
                    TILESIZE - row - 1
                }
            } else {
                row
            };

            let spr_num = if is_8x16 {
                // In 8x16 mode, lower bit of tile number is ignored
                // Upper 8x8 tile is NN & $FE
                // Lower 8x8 tile is NN | $01
                if row < TILESIZE {
                    spr.get_tile_num() & 0xFE
                } else {
                    spr.get_tile_num() | 0x01
                }
            } else {
                // If 8x8 sprite, simply get tile num
                spr.get_tile_num()
            };
            let spr_bank = spr_num as usize + (self.vram_bank * TILE_NUM);

            let tile = &self.tiles[spr_bank];
            let pixels = tile.get_row(row % TILESIZE);
            let spr_x = top_x as usize;
            for col in 0..TILESIZE {
                let pixel = pixels[col as usize] as usize;
                let x_offset = if spr.is_x_flip() {
                    TILESIZE - col - 1
                } else {
                    col
                };

                let pixel_x = spr_x.wrapping_add(x_offset);
                // Move on if pixel is going to be drawn off-screen
                if pixel_x >= SCREEN_WIDTH {
                    continue;
                }

                // Only draw pixel if
                // - Sprite is above background, and the pixel being drawn isn't transparent
                // - Sprite is below background, and background has transparent color here
                if (above_bg && pixel != 0) || (!above_bg && (pixel_row[(COLOR_CHANNELS * pixel_x)..(COLOR_CHANNELS * (pixel_x + 1))] == dmg_pal[0])) {
                    if mode == GB::CGB {
                        let raw_color = merge_bytes(self.cgb_spr_pal_data[pixel + 1], self.cgb_spr_pal_data[pixel]);
                        let color = gbc2rgba(raw_color);
                        for i in 0..COLOR_CHANNELS {
                            pixel_row[COLOR_CHANNELS * pixel_x + i] = color[i];
                        }
                    } else {
                        let color = dmg_pal[pal_indices[pixel] as usize];
                        for i in 0..COLOR_CHANNELS {
                            pixel_row[COLOR_CHANNELS * pixel_x + i] = color[i];
                        }
                    }
                }
            }
        }
    }

    /// ```
    /// Write IO
    ///
    /// Writes byte to I/O register space ($FF00-$FF7F)
    ///
    /// Inputs:
    ///     Address to write to (u16)
    ///     Value to write (u8)
    /// ```
    fn write_io(&mut self, addr: u16, val: u8) {
        let io_addr = addr - IO_START;
        self.io[io_addr as usize] = val;
    }

    /// ```
    /// Read IO
    ///
    /// Reads byte from I/O register space ($FF00-$FF7F)
    ///
    /// Input:
    ///     Address to read from (u16)
    ///
    /// Output:
    ///     Value at address (u8)
    /// ```
    fn read_io(&self, addr: u16) -> u8 {
        let io_addr = addr - IO_START;
        self.io[io_addr as usize]
    }

    /// ```
    /// Get background tile map
    ///
    /// Gets the pixel data for the background tiles
    ///
    /// Output:
    ///     Slice of tilemap values (&[u8])
    /// ```
    fn get_bkgd_tile_map(&self) -> &[u8] {
        // $00 for $9800-$9BFF
        // $01 for $9C00-$9FFF
        if self.get_bkgd_tile_map_index() == 0 {
            &self.vram[TILE_MAP_0_RANGE]
        } else {
            &self.vram[TILE_MAP_1_RANGE]
        }
    }

    /// ```
    /// Get window tile map
    ///
    /// Gets the pixel data for the window tiles
    ///
    /// Output:
    ///     Slice of tilemap values (&[u8])
    /// ```
    fn get_wndw_tile_map(&self) -> &[u8] {
        // $00 for $9800-$9BFF
        // $01 for $9C00-$9FFF
        if self.get_wndw_tile_map_index() == 0 {
            &self.vram[TILE_MAP_0_RANGE]
        } else {
            &self.vram[TILE_MAP_1_RANGE]
        }
    }

    /// ```
    /// Get DMG background palette
    ///
    /// Gets the palette indices from the BGP register ($FF47)
    ///
    /// Output:
    ///     Palette indices ([u8])
    /// ```
    fn get_dmg_bkgd_palette(&self) -> [u8; DMG_PAL_SIZE] {
        unpack_u8(self.read_io(BGP))
    }

    /// ```
    /// Get sprite palette
    ///
    /// Gets the palette indices for the sprites
    ///
    /// Input:
    ///     Whether to use palette 0 or 1 (bool)
    ///
    /// Output:
    ///     Palette indices ([u8])
    /// ```
    fn get_dmg_spr_palette(&self, pal_0: bool) -> [u8; DMG_PAL_SIZE] {
        if pal_0 {
            unpack_u8(self.read_io(OBP0))
        } else {
            unpack_u8(self.read_io(OBP1))
        }
    }

    /// ```
    /// Sort sprites
    ///
    /// Sort sprites into correct drawing order
    ///
    /// Output:
    ///     Sorted sprites (Vec<Sprite>)
    /// ```
    fn sort_sprites(&self) -> Vec<Sprite> {
        // In event of overlap, sprites are drawn
        // (on DMG) with the lowest x-coordinate on top.
        // If tie, lowest sprite number goes on top
        let mut sprites = self.oam.to_vec();
        // Reverse the vector so that lower sprite number is earlier in a tie
        sprites.reverse();
        sprites.sort_by(|a, b| b.get_coords().0.cmp(&a.get_coords().0));
        sprites
    }

    /// ```
    /// Is the LCD displayed
    ///
    /// Is the LCD screen enabled
    ///
    /// Output:
    ///     Whether or not LCD screen is enabled (bool)
    /// ```
    fn is_lcd_dspl(&self) -> bool {
        let lcd_control = self.read_io(LCDC);
        lcd_control.get_bit(LCD_DISP_BIT)
    }

    /// ```
    /// Is background displayed
    ///
    /// Is background layer currently visible
    ///
    /// Output:
    ///     Whether or not background is displayed (bool)
    /// ```
    fn is_bkgd_dspl(&self) -> bool {
        let lcd_control = self.read_io(LCDC);
        lcd_control.get_bit(BG_DISP_BIT)
    }

    /// ```
    /// Is window displayed
    ///
    /// Is the window layer currently visible
    ///
    /// Output:
    ///     Whether window layer is visible (bool)
    /// ```
    fn is_wndw_dspl(&self, mode: GB) -> bool {
        let lcd_control = self.read_io(LCDC);
        let mut is_dspl = lcd_control.get_bit(WNDW_DISP_BIT);
        if mode == GB::CGB_DMG {
            // For CGB running in DMG mode, the BG bit can also disable the background
            is_dspl &= lcd_control.get_bit(BG_DISP_BIT);
        }

        is_dspl
    }

    /// ```
    /// Are sprites displayed
    ///
    /// Is the sprite layer visible
    ///
    /// Output:
    ///     Whether the sprite layer is visible (bool)
    /// ```
    fn is_sprt_dspl(&self) -> bool {
        let lcd_control = self.read_io(LCDC);
        lcd_control.get_bit(SPR_DISP_BIT)
    }

    /// ```
    /// Get background tileset index
    ///
    /// Returns which tileset is being used (0/1)
    ///
    /// Output:
    ///     Tileset index (u8)
    /// ```
    fn get_bkgd_wndw_tile_set_index(&self) -> u8 {
        let lcd_control = self.read_io(LCDC);
        if lcd_control.get_bit(BG_WNDW_TILE_DATA_BIT) { 1 } else { 0 }
    }

    /// ```
    /// Get background tilemap index
    ///
    /// Returns which tilemap set is being used (0/1)
    ///
    /// Output:
    ///     Tilemap index (u8)
    /// ```
    fn get_bkgd_tile_map_index(&self) -> u8 {
        let lcd_control = self.read_io(LCDC);
        if lcd_control.get_bit(BG_TILE_MAP_BIT) { 1 } else { 0 }
    }

    /// ```
    /// Get window tilemap index
    ///
    /// Returns which window tilemap set is being used (0/1)
    ///
    /// Output:
    ///     Tilemap index (u8)
    /// ```
    fn get_wndw_tile_map_index(&self) -> u8 {
        let lcd_control = self.read_io(LCDC);
        if lcd_control.get_bit(WNDW_TILE_MAP_BIT) { 1 } else { 0 }
    }

    /// ```
    /// Are sprites 8x16?
    ///
    /// Returns true if sprites are to be drawn 8x16
    ///
    /// Output:
    ///     Whether spries are 8x16 (vs 8x8) (bool)
    /// ```
    fn spr_are_8x16(&self) -> bool {
        self.read_io(LCDC).get_bit(SPR_SIZE_BIT)
    }

    /// ```
    /// Set VRAM bank
    ///
    /// Sets which VRAM tile bank should be used (either 0 or 1)
    ///
    /// Input:
    ///     Which bank to use (u8)
    /// ```
    fn set_vram_bank(&mut self, val: u8) {
        self.vram_bank = if val.get_bit(0) { 1 } else { 0 };
    }

    /// ```
    /// Get scroll coords
    ///
    /// Returns the values of the SCX and SCY registers
    ///
    /// Output:
    ///     SCX, SCY point (Point)
    /// ```
    fn get_scroll_coords(&self) -> Point {
        let scroll_x = self.read_io(SCX);
        let scroll_y = self.read_io(SCY);

        Point::new(scroll_x, scroll_y)
    }

    /// ```
    /// Get window coords
    ///
    /// Returns the window position from the WX and WY registers
    ///
    /// Output:
    ///     Location of the window (Point)
    /// ```
    fn get_wndw_coords(&self) -> Point {
        let wndw_x = self.read_io(WX).saturating_sub(7);
        let wndw_y = self.read_io(WY);

        Point::new(wndw_x, wndw_y)
    }

    /// ```
    /// Get LCDC Status
    ///
    /// Get the current clock mode from the LCD status register
    ///
    /// Output:
    ///     Current clock mode (ModeTypes)
    /// ```
    fn get_lcdc_status(&self) -> ModeTypes {
        let lcd_stat = self.read_io(STAT);
        let mode = lcd_stat & 0b0000_0011;
        match mode {
            0 => { ModeTypes::HBLANK },
            1 => { ModeTypes::VBLANK },
            2 => { ModeTypes::OAMReadMode },
            3 => { ModeTypes::VRAMReadMode },
            _ => { panic!("Invalid mode") }
        }
    }

    /// ```
    /// Is valid status
    ///
    /// Can we write to the given address, given the clock mode?
    ///
    /// Input:
    ///     Address to write to (u16)
    ///
    /// Output:
    ///     Write status (bool)
    /// ```
    fn is_valid_status(&self, addr: u16) -> bool {
        let lcdc_status = self.get_lcdc_status();

        match lcdc_status {
            ModeTypes::OAMReadMode => {
                !is_in_oam(addr)
            },
            ModeTypes::VRAMReadMode => {
                // TODO: This function should also not allow writes to VRAM
                // However, this blocks needed GFX writes, and causes corrupted graphics
                // Likely an issue with timing, probably fixed when mem_timing passes
                !is_in_oam(addr) // && !is_in_vram(addr)
            },
            _ => {
                true
            }
        }
    }

    /// ```
    /// Read CGB Background color data
    ///
    /// Gets the color data from the specified index
    ///
    /// Output:
    ///     Partial color data loaded into the palette data RAM register
    /// ```
    fn read_cgb_bg_color(&self) -> u8 {
        let ind = self.read_io(BGPI) & 0x3F;
        self.cgb_bg_pal_data[ind as usize]
    }

    /// ```
    /// Write CGB Background color data
    ///
    /// Sets the color data from the specified index
    ///
    /// Input:
    ///     New value for the index set in BGPI
    /// ```
    fn write_cgb_bg_color(&mut self, val: u8) {
        let ind = self.read_io(BGPI) & 0x3F;
        self.cgb_bg_pal_data[ind as usize] = val;
    }

    /// ```
    /// Read CGB sprite color data
    ///
    /// Gets the color data from the specified index
    ///
    /// Output:
    ///     Partial color data loaded into the palette data RAM register
    /// ```
    fn read_cgb_spr_color(&self) -> u8 {
        let ind = self.read_io(OBPI) & 0x7F;
        self.cgb_spr_pal_data[ind as usize]
    }

    /// ```
    /// Write CGB sprite color data
    ///
    /// Sets the color data from the specified index
    ///
    /// Input:
    ///     New value for the index set in OBPI
    /// ```
    fn write_cgb_spr_color(&mut self, val: u8) {
        let ind = self.read_io(OBPI) & 0x7F;
        self.cgb_spr_pal_data[ind as usize] = val;
    }
}

/// ```
/// Is in OAM?
///
/// Helper function to determine if address being written to is in OAM memory
///
/// Inputs:
///     Address to write to (u16)
///
/// Outputs:
///     Whether the address is in OAM memory (bool)
/// ```
fn is_in_oam(addr: u16) -> bool {
    addr >= OAM_START && addr <= OAM_END
}

/// ```
/// Is in VRAM?
///
/// Helper function to determine if address being written to is in VRAM memory
///
/// Inputs:
///     Address to write to (u16)
///
/// Outputs:
///     Whether the address is in VRAM memory (bool)
/// ```
fn is_in_vram(addr: u16) -> bool {
    addr >= VRAM_START && addr <= VRAM_END
}
