mod sprite;
mod tile;

use sprite::{OAM_BYTE_SIZE, Sprite};
use tile::{Tile, TILE_BYTES};
use crate::cpu::clock::ModeTypes;
use crate::utils::*;
use std::ops::Range;

// =============
// = Constants =
// =============
const MAP_SIZE: usize = 32; // In tiles
const MAP_PIXELS: usize = MAP_SIZE * TILESIZE; // In pixels
const VRAM_SIZE: usize = 0x8000;
const VRAM_OFFSET: usize = 0x8000;
const TILE_NUM: usize = 384;
const OAM_SPR_NUM: usize = 40;
const SPR_PER_LINE: usize = 10;

// VRAM registers
const LCDC: usize                    = 0xFF40 - VRAM_OFFSET;
const STAT: usize                    = 0xFF41 - VRAM_OFFSET;
const SCY: usize                     = 0xFF42 - VRAM_OFFSET;
const SCX: usize                     = 0xFF43 - VRAM_OFFSET;
const LY: usize                      = 0xFF44 - VRAM_OFFSET;
const LYC: usize                     = 0xFF45 - VRAM_OFFSET;
// 0xFF46 is DMA transfer, handled by Bus
const BGP: usize                     = 0xFF47 - VRAM_OFFSET;
const OBP0: usize                    = 0xFF48 - VRAM_OFFSET;
const OBP1: usize                    = 0xFF49 - VRAM_OFFSET;
const WY: usize                      = 0xFF4A - VRAM_OFFSET;
const WX: usize                      = 0xFF4B - VRAM_OFFSET;

// VRAM ranges
const DISPLAY_RAM_RANGE: Range<usize> = (0x8000 - VRAM_OFFSET)..(0xA000 - VRAM_OFFSET);
const OAM_MEM: u16                    = 0xFE00 - (VRAM_OFFSET as u16);
const OAM_MEM_END: u16                = 0xFE9F - (VRAM_OFFSET as u16); // Inclusive
const TILE_SET: u16                   = 0x8000 - (VRAM_OFFSET as u16);
const TILE_SET_END: u16               = 0x97FF - (VRAM_OFFSET as u16);

const TILE_MAP_0_RANGE: Range<usize> = (0x9800 - VRAM_OFFSET)..(0x9C00 - VRAM_OFFSET);
const TILE_MAP_1_RANGE: Range<usize> = (0x9C00 - VRAM_OFFSET)..(0xA000 - VRAM_OFFSET);

// Register bit constants
const BG_DISP_BIT: u8           = 0;
const SPR_DISP_BIT: u8          = 1;
const SPR_SIZE_BIT: u8          = 2;
const BG_TILE_MAP_BIT: u8       = 3;
const BG_WNDW_TILE_DATA_BIT: u8 = 4;
const WNDW_DISP_BIT: u8         = 5;
const WNDW_TILE_MAP_BIT: u8     = 6;
const LCD_DISP_BIT: u8          = 7;

const LYC_LY_FLAG_BIT: u8 =         2;
// const HBLANK_INTERRUPT_BIT: u8 =    3;
// const VBLANK_INTERRUPT_BIT: u8 =    4;
// const OAM_INTERRUPT_BIT: u8 =       5;
const LYC_LY_INTERRUPT_BIT: u8 =    6;

// Colors
const BLACK: [u8; COLOR_CHANNELS]            = [0,   0,   0,   255];
const LIGHT_GRAY: [u8; COLOR_CHANNELS]       = [148, 148, 165, 255];
const DARK_GRAY: [u8; COLOR_CHANNELS]        = [107, 107, 90,  255];
const WHITE: [u8; COLOR_CHANNELS]            = [255, 255, 255, 255];

const COLORS: [[u8; COLOR_CHANNELS]; 4] = [
    WHITE,
    LIGHT_GRAY,
    DARK_GRAY,
    BLACK,
];

pub struct PPU {
    vram: [u8; VRAM_SIZE],
    screen_buffer: [u8; SCREEN_HEIGHT * SCREEN_WIDTH],
    tiles: [Tile; TILE_NUM],
    oam: [Sprite; OAM_SPR_NUM],
}

impl PPU {
    // ==================
    // = Public methods =
    // ==================
    pub fn new() -> PPU {
        PPU {
            vram: [0; VRAM_SIZE],
            screen_buffer: [0; SCREEN_HEIGHT * SCREEN_WIDTH],
            tiles: [Tile::new(); TILE_NUM],
            oam: [Sprite::new(); OAM_SPR_NUM],
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
    /// ```
    pub fn write_vram(&mut self, raw_addr: u16, val: u8) {
        let addr = raw_addr - VRAM_OFFSET as u16;

        if self.is_valid_status(raw_addr) {
            // Update OAM objects if needed
            if is_in_oam(addr) {
                let relative_addr = addr - OAM_MEM;
                let spr_num = relative_addr / OAM_BYTE_SIZE;
                let byte_num = relative_addr % OAM_BYTE_SIZE;
                self.oam[spr_num as usize].update_byte(byte_num, val);
            } else if is_in_tile_set(addr) {
                let offset = addr - TILE_SET;
                let tile_num = offset / TILE_BYTES;
                let byte_num = offset % TILE_BYTES;
                self.tiles[tile_num as usize].update_byte(byte_num, val);
            }

            self.vram[addr as usize] = val;
        }
    }

    /// ```
    /// Read VRAM
    ///
    /// Read value from given address in VRAM
    ///
    /// Input:
    ///     Address to read from (u16)
    ///
    /// Output:
    ///     Value at given address (u8)
    /// ```
    pub fn read_vram(&self, raw_addr: u16) -> u8 {
        let addr = raw_addr - VRAM_OFFSET as u16;
        self.vram[addr as usize]
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
        self.vram[LY] = line;

        if self.vram[LY] == self.vram[LYC] {
            // If LY and LYC are equal:
            // - Set coincidence bit in STAT register
            // - Trigger LCDC status interrupt if enabled
            self.vram[STAT].set_bit(LYC_LY_FLAG_BIT);
            self.vram[STAT].get_bit(LYC_LY_INTERRUPT_BIT)
        } else {
            false
        }
    }

    /// ```
    /// Render scanline
    ///
    /// Renders specified scanline to buffer
    /// ```
    pub fn render_scanline(&mut self) {
        // Render current scanline
        let line = self.vram[LY];
        let mut pixel_row = [0; SCREEN_WIDTH];

        if self.is_bkgd_dspl() {
            self.render_background_line(&mut pixel_row, line);
        }

        if self.is_wndw_dspl() {
            self.render_wndw_line(&mut pixel_row, line);
        }

        if self.is_sprt_dspl() {
            self.render_sprite_line(&mut pixel_row, line);
        }

        // Copy this line of pixels into overall screen buffer
        let start_index = line as usize * SCREEN_WIDTH;
        let end_index = (line + 1) as usize * SCREEN_WIDTH;
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
        self.vram[STAT] &= 0b1111_1100;
        self.vram[STAT] |= mode;
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
        let mut map_array = [0; SCREEN_HEIGHT * SCREEN_WIDTH];
        if self.is_lcd_dspl() {
            map_array.copy_from_slice(&self.screen_buffer);
        }
        self.get_color(&map_array)
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
    /// ```
    fn render_background_line(&self, pixel_row: &mut [u8], line: u8) {
        let tile_map = self.get_bkgd_tile_map();
        let palette = self.get_bkgd_palette();
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
            let tile_index = if self.get_bkgd_wndw_tile_set_index() == 0 {
                (256 + (tile_map[index] as i8 as isize)) as usize
            } else {
                tile_map[index] as usize
            };
            let tile = &self.tiles[tile_index];
            let col = (start_x + x) % TILESIZE;
            let pixel = tile.get_row(row)[col];
            let corrected_pixel = palette[pixel as usize];
            pixel_row[x] = corrected_pixel;
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
    fn render_wndw_line(&self, pixel_row: &mut [u8], line: u8) {
        let wndw_coords = self.get_wndw_coords();
        // If window isn't drawn on this scanline, return
        if wndw_coords.y > line {
            return;
        }

        let wndw_map = self.get_wndw_tile_map();
        let palette = self.get_bkgd_palette();

        // Get the row of tiles containing our scanline
        let y = (line - wndw_coords.y) as usize;
        let row = y % TILESIZE;
        let map_y = y / TILESIZE;
        let start_x = wndw_coords.x as usize;
        for x in start_x..SCREEN_WIDTH {
            // Get coords for current tile
            let map_x = (x % MAP_PIXELS) / TILESIZE;
            let index = map_y * MAP_SIZE + map_x;
            // The tile indexes in the second tile pattern table ($8800-97ff) are signed
            let tile_index = if self.get_bkgd_wndw_tile_set_index() == 0 {
                (256 + (wndw_map[index] as i8 as isize)) as usize
            } else {
                wndw_map[index] as usize
            };
            let tile = &self.tiles[tile_index];
            let col = x % TILESIZE;
            let pixel = tile.get_row(row)[col];
            let corrected_pixel = palette[pixel as usize];
            pixel_row[x] = corrected_pixel;
        }
    }

    /// ```
    /// Render Sprite Line
    ///
    /// Renders the given scanline of the sprite layer
    ///
    /// Inputs:
    ///     Array to load pixel data into (&[u8])
    ///     Scanline to render (u8)
    /// ```
    fn render_sprite_line(&self, pixel_row: &mut [u8], line: u8) {
        // Iterate through every sprite
        let sorted_sprites = self.sort_sprites();
        let is_8x16 = self.spr_are_8x16();
        let mut sprites_drawn = 10;
        for i in 0..sorted_sprites.len() {
            let spr = sorted_sprites[i];
            if !spr.contains_scanline(line, is_8x16) || !spr.is_onscreen() {
                continue;
            }

            sprites_drawn += 1;
            // System only allows finite number of sprites drawn per line
            // If we hit threshold, no more sprites can be drawn on this line
            if sprites_drawn == SPR_PER_LINE {
                break;
            }

            let palette = self.get_spr_palette(spr.is_pal_0());
            let above_bg = spr.is_above_bkgd();

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
            // If we are 8x16 and Y-flipped, bottom tile is drawn on top
            let spr_num = if is_8x16 && spr.is_y_flip() {
                if row >= TILESIZE {
                    spr.get_tile_num()
                } else {
                    spr.get_tile_num() + 1
                }
            } else {
                if row >= TILESIZE {
                    spr.get_tile_num() + 1
                } else {
                    spr.get_tile_num()
                }
            };

            let tile = &self.tiles[spr_num as usize];
            let pixels = tile.get_row(row % TILESIZE);
            let spr_x = top_x as usize;
            for col in 0..TILESIZE {
                let pixel = pixels[col as usize];
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

                let corrected_pixel = palette[pixel as usize];

                // Only draw pixel if
                // - Sprite is above background, and the pixel being drawn isn't transparent
                // - Sprite is below background, and background has transparent color here
                let should_draw = (above_bg && pixel != 0) || (!above_bg && pixel_row[pixel_x] == 0);
                if should_draw {
                    pixel_row[pixel_x] = corrected_pixel;
                }
            }
        }
    }

    /// ```
    /// Get color
    ///
    /// Gets the pixel values for the pixels currently on screen
    ///
    /// Input:
    ///     160x144 screen pixel array (&[u8])
    ///
    /// Output:
    ///     RGB values for on-screen pixels ([u8])
    /// ```
    fn get_color(&self, pixel_array: &[u8]) -> [u8; DISP_SIZE] {
        let mut rgb_screen = [0; DISP_SIZE];
        // Iterate through every visible pixel
        for y in 0..SCREEN_HEIGHT {
            for x in 0..SCREEN_WIDTH {
                let index = y * SCREEN_WIDTH + x;
                let pixel = pixel_array[index];

                let view_index = index * COLOR_CHANNELS;
                let color = COLORS[pixel as usize];
                for i in 0..color.len() {
                    rgb_screen[view_index + i] = color[i];
                }
            }
        }

        rgb_screen
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
        let tile_map = if self.get_bkgd_tile_map_index() == 0 {
            &self.vram[TILE_MAP_0_RANGE]
        } else {
            &self.vram[TILE_MAP_1_RANGE]
        };

        tile_map
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
        let wndw_map = if self.get_wndw_tile_map_index() == 0 {
            &self.vram[TILE_MAP_0_RANGE]
        } else {
            &self.vram[TILE_MAP_1_RANGE]
        };

        wndw_map
    }

    /// ```
    /// Get background palette
    ///
    /// Gets the palette indices from the BGP register ($FF47)
    ///
    /// Output:
    ///     Palette indices ([u8])
    /// ```
    fn get_bkgd_palette(&self) -> [u8; 4] {
        unpack_u8(self.vram[BGP])
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
    fn get_spr_palette(&self, pal_0: bool) -> [u8; 4] {
        let pal = if pal_0 {
            unpack_u8(self.vram[OBP0])
        } else {
            unpack_u8(self.vram[OBP1])
        };

        pal
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
        let lcd_control = self.vram[LCDC];
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
        let lcd_control = self.vram[LCDC];
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
    fn is_wndw_dspl(&self) -> bool {
        let lcd_control = self.vram[LCDC];
        lcd_control.get_bit(WNDW_DISP_BIT)
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
        let lcd_control = self.vram[LCDC];
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
        let lcd_control = self.vram[LCDC];
        if lcd_control.get_bit(BG_WNDW_TILE_DATA_BIT) { return 1 } else { return 0 }
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
        let lcd_control = self.vram[LCDC];
        if lcd_control.get_bit(BG_TILE_MAP_BIT) { return 1 } else { return 0 }
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
        let lcd_control = self.vram[LCDC];
        if lcd_control.get_bit(WNDW_TILE_MAP_BIT) { return 1 } else { return 0 }
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
        self.vram[LCDC].get_bit(SPR_SIZE_BIT)
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
        let scroll_x = self.vram[SCX];
        let scroll_y = self.vram[SCY];

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
        let wndw_x = self.vram[WX].saturating_sub(7);
        let wndw_y = self.vram[WY];

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
        let lcd_stat = self.vram[STAT];
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
                !is_in_oam(addr) && !DISPLAY_RAM_RANGE.contains(&(addr as usize))
            },
            _ => {
                true
            }
        }
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
    addr >= OAM_MEM && addr <= OAM_MEM_END
}

/// ```
/// Is in tile set?
///
/// Helper function to determine if address is in tile set memory
///
/// Input:
///     Address in question (u16)
///
/// Output:
///     Whether address is in tile set memory (bool)
/// ```
fn is_in_tile_set(addr: u16) -> bool {
    addr >= TILE_SET && addr <= TILE_SET_END
}
