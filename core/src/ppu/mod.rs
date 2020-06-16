mod sprite;
mod tile;

use sprite::Sprite;
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

pub struct PPU {
    vram: [u8; VRAM_SIZE],
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
            tiles: [Tile::new(); TILE_NUM],
            oam: [Sprite::new(); OAM_SPR_NUM],
        }
    }

    /// ```
    /// Write VRAM
    ///
    /// Write value to specified address in VRAM
    ///
    /// Can't access OAM memory during OAM Interrupt
    /// Can't access OAM or VRAM during LCD transfer
    ///
    /// Input:
    ///     Address to write to (u16)
    ///     Value to write (u8)
    /// ```
    pub fn write_vram(&mut self, raw_addr: u16, val: u8) {
        let addr = raw_addr - VRAM_OFFSET as u16;

        if self.is_valid_status(raw_addr) {
            // Update OAM objects if needed
            if is_in_oam(addr) {
                let relative_addr = addr - OAM_MEM;
                let spr_num = relative_addr / 4;
                let byte_num = relative_addr % 4;
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
    ///     Whether values in LY and LYC registers are equal
    /// ```
    pub fn set_ly(&mut self, line: u8) -> bool {
        self.vram[LY] = line;

        if self.vram[LY] == self.vram[LYC] {
            // If LY and LYC are equal:
            // - Set coincidence bit in STAT register
            // - Trigger LCDC status interrupt if enabled
            self.vram[STAT].set_bit(2);
            self.vram[STAT].get_bit(6)
        } else {
            false
        }
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
        let mut map_array = [0; MAP_PIXELS * MAP_PIXELS];
        if self.is_bkgd_dspl() {
            self.render_background(&mut map_array);
        }

        if self.is_wndw_dspl() {
            self.render_window(&mut map_array);
        }

        if self.is_sprt_dspl() {
            self.render_sprites(&mut map_array);
        }

        // TODO: Someday this all should be rewritten so that this function isn't needed
        let screen = self.get_view(&map_array);

        screen
    }

    // ===================
    // = Private methods =
    // ===================

    /// ```
    /// Render background
    ///
    /// Renders the background tiles onto the pixel array
    ///
    /// Input:
    ///     Array of pixels to modify (&[u8])
    /// ```
    fn render_background(&self, pixel_array: &mut [u8]) {
        let tile_map = self.get_bkgd_tile_map();
        let palette = self.get_bkgd_palette();

        // Iterate through every tile in map
        for y in 0..MAP_SIZE {
            for x in 0..MAP_SIZE {
                let index = y * MAP_SIZE + x;
                // The tile indexes in the second tile pattern table ($8800-97ff) are signed
                let tile_index = if self.get_bkgd_tile_set_index() == 0 {
                    (256 + (tile_map[index] as i8 as isize)) as usize
                } else {
                    tile_map[index] as usize
                };
                let tile = &self.tiles[tile_index];

                // Iterate through row in tile
                for row in 0..TILESIZE {
                    let map_x = TILESIZE * x;
                    let map_y = (TILESIZE * y) + row;
                    let map_index = (map_y * MAP_SIZE * TILESIZE) + map_x;
                    let pixels = tile.get_row(row);
                    // Iterate through each pixel in row, applying the palette
                    for i in 0..TILESIZE {
                        let corrected_pixel = palette[pixels[i as usize] as usize];
                        pixel_array[map_index + i] = corrected_pixel;
                    }
                }
            }
        }
    }

    /// ```
    /// Render window
    ///
    /// Renders the window tiles onto the pixel array
    ///
    /// Input:
    ///     Array of pixels to modify (&[u8])
    /// ```
    fn render_window(&self, pixel_array: &mut [u8]) {
        let coords = self.get_wndw_coords();
        let wndw_map = self.get_wndw_tile_map();
        let palette = self.get_bkgd_palette();

        // Iterate through all tiles in window
        'tile_y: for y in 0..MAP_SIZE {
            'tile_x: for x in 0..MAP_SIZE {
                let index = y * MAP_SIZE + x;
                let tile_index = wndw_map[index];
                let tile = &self.tiles[tile_index as usize];

                // Windows can only be drawn on bottom/right of screen
                // If tiles have gone off to the right, we are done with this row
                // If they've gone off the bottom, we're done period
                let map_x = x + coords.x as usize;
                let map_y = y + coords.y as usize;
                if map_y > SCREEN_HEIGHT {
                    break 'tile_y;
                } else if map_x > SCREEN_WIDTH {
                    break 'tile_x;
                }

                for row in 0..TILESIZE {
                    let map_index = (map_y + row) * MAP_SIZE + map_x;
                    let pixels = tile.get_row(row);
                    // Iterate through each pixel in row, applying the palette
                    for i in 0..TILESIZE {
                        let corrected_pixel = palette[pixels[i as usize] as usize];
                        pixel_array[map_index + i] = corrected_pixel;
                    }
                }
            }
        }
    }

    /// ```
    /// Render sprites
    ///
    /// Renders the sprites onto the graphics array
    ///
    /// Input:
    ///     [u8] - Graphics array to render upon
    /// ```
    fn render_sprites(&self, pixel_array: &mut [u8]) {
        // Iterate through every sprite
        for i in 0..OAM_SPR_NUM {
            let spr = self.oam[i];
            if !spr.is_onscreen() {
                continue;
            }

            let spr_coords = spr.get_coords();
            let spr_num = spr.get_tile_num();
            let tile = &self.tiles[spr_num as usize];
            self.draw_spr(pixel_array, tile, spr, spr_coords);

            // If sprites are 8x16, draw the bottom sprite
            if self.spr_are_8x16() {
                let top_coords = spr.get_coords();
                let spr_coords = Point::new(top_coords.x, top_coords.y + TILESIZE as u8);
                let spr_num = spr.get_tile_num() + 1;
                let tile = &self.tiles[spr_num as usize];
                self.draw_spr(pixel_array, tile, spr, spr_coords);
            }
        }
    }

    /// ```
    /// Draw sprite
    ///
    /// Draw sprite to screen
    ///
    /// Inputs:
    ///     Graphics array to render upon ([u8])
    ///     Tile to render (Tile)
    ///     Sprite metadata (Sprite)
    ///     Screen coordinates to draw to (Point)
    /// ```
    fn draw_spr(&self, pixel_array: &mut[u8], tile: &Tile, spr: Sprite, spr_coords: Point) {
        // TODO: This does not check if sprite should be drawn above/below background
        let screen_coords = self.get_scroll_coords();
        let palette = self.get_spr_palette(spr.is_pal_0());
        let flip_x = spr.is_x_flip();
        let flip_y = spr.is_y_flip();

        for row in 0..TILESIZE {
            let spr_x = (screen_coords.x as usize) + (spr_coords.x as usize);
            let spr_y = ((screen_coords.y as usize) + (spr_coords.y as usize) + row) % MAP_PIXELS;
            let arr_index = spr_x + spr_y * MAP_PIXELS;
            let pixels = if flip_x {
                tile.get_row(row)
            } else {
                tile.get_row(TILESIZE - row - 1)
            };

            // Iterate through each pixel in row, applying the palette
            for j in 0..TILESIZE {
                let pixel = pixels[j as usize];
                // Pixel value 0 is transparent
                if pixel != 0 {
                    let corrected_pixel = palette[pixel as usize];
                    let arr_offset = if flip_y {
                        j
                    } else {
                        TILESIZE - j - 1
                    };

                    pixel_array[arr_index + arr_offset] = corrected_pixel;
                }
            }
        }
    }

    /// ```
    /// Get view
    ///
    /// Gets the pixel values for the pixels currently on screen
    ///
    /// Input:
    ///     Entire 256x256 screen pixel array (&[u8])
    ///
    /// Output:
    ///     Index values for on-screen pixels ([u8])
    /// ```
    fn get_view(&self, pixel_array: &[u8]) -> [u8; DISP_SIZE] {
        let mut viewport = [0; DISP_SIZE];
        let scroll = self.get_scroll_coords();

        // Iterate through every visible pixel
        let start_x = scroll.x as usize;
        let start_y = scroll.y as usize;
        for y in start_y..(start_y + SCREEN_HEIGHT) {
            for x in start_x..(start_x + SCREEN_WIDTH) {
                // Wrap X/Y coord if needed
                let adj_x = x % MAP_PIXELS;
                let adj_y = y % MAP_PIXELS;

                let index = adj_y * MAP_PIXELS + adj_x;
                let pixel = pixel_array[index];

                // Index in output array uses un-wrapped x/y for indices
                let view_index = (y - start_y) * SCREEN_WIDTH + (x - start_x);
                viewport[view_index] = pixel;
            }
        }

        viewport
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
    /// Is background displayed
    ///
    /// Is background layer currently visible
    ///
    /// Output:
    ///     Whether or not background is displayed (bool)
    /// ```
    fn is_bkgd_dspl(&self) -> bool {
        let lcd_control = self.vram[LCDC];
        lcd_control.get_bit(0)
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
        lcd_control.get_bit(5)
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
        lcd_control.get_bit(1)
    }

    /// ```
    /// Get background tileset index
    ///
    /// Returns which tileset is being used (0/1)
    ///
    /// Output:
    ///     Tileset index (u8)
    /// ```
    fn get_bkgd_tile_set_index(&self) -> u8 {
        let lcd_control = self.vram[LCDC];
        if lcd_control.get_bit(4) { return 1 } else { return 0 }
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
        if lcd_control.get_bit(3) { return 1 } else { return 0 }
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
        if lcd_control.get_bit(6) { return 1 } else { return 0 }
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
        self.vram[LCDC].get_bit(2)
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
    fn get_wndw_coords(&self) -> Point {
        let wndw_x = self.vram[WX].wrapping_sub(7);
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

fn is_in_tile_set(addr: u16) -> bool {
    addr >= TILE_SET && addr <= TILE_SET_END
}
