mod tile;

use tile::Tile;
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

// VRAM registers
const LCD_DISP_REG: usize            = 0xFF40 - VRAM_OFFSET;
const LCD_STAT_REG: usize            = 0xFF41 - VRAM_OFFSET;
const SCY: usize                     = 0xFF42 - VRAM_OFFSET;
const SCX: usize                     = 0xFF43 - VRAM_OFFSET;
const LY: usize                      = 0xFF44 - VRAM_OFFSET;
const LYC: usize                     = 0xFF45 - VRAM_OFFSET;
const DMA: usize                     = 0xFF46 - VRAM_OFFSET;
const BGP: usize                     = 0xFF47 - VRAM_OFFSET;
const OBP0: usize                    = 0xFF48 - VRAM_OFFSET;
const OBP1: usize                    = 0xFF49 - VRAM_OFFSET;
const WY: usize                      = 0xFF4A - VRAM_OFFSET;
const WX: usize                      = 0xFF4B - VRAM_OFFSET;

// VRAM ranges
const DISPLAY_RAM_RANGE: Range<usize> = (0x8000 - VRAM_OFFSET)..(0xA000 - VRAM_SIZE);
const OAM_MEM_RANGE: Range<usize> = (0xFE00 - VRAM_OFFSET)..(0xFEA0 - VRAM_OFFSET);

const TILE_SET_0_RANGE: Range<usize> = (0x8000 - VRAM_OFFSET)..(0x9000 - VRAM_OFFSET);
const TILE_SET_1_RANGE: Range<usize> = (0x8800 - VRAM_OFFSET)..(0x9800 - VRAM_OFFSET);
const TILE_MAP_0_RANGE: Range<usize> = (0x9800 - VRAM_OFFSET)..(0x9C00 - VRAM_OFFSET);
const TILE_MAP_1_RANGE: Range<usize> = (0x9C00 - VRAM_OFFSET)..(0xA000 - VRAM_OFFSET);
const SAM:              Range<usize> = (0xFE00 - VRAM_OFFSET)..(0xFEA0 - VRAM_OFFSET);

pub struct PPU {
    vram: [u8; VRAM_SIZE],
}

impl PPU {
    // ==================
    // = Public methods =
    // ==================
    pub fn new() -> PPU {
        PPU {
            vram: [0; VRAM_SIZE],
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
    pub fn write_vram(&mut self, addr: u16, val: u8) {
        let adjusted_addr = addr - VRAM_OFFSET as u16;
        let lcdc_mode = self.get_LCDC_status();

        if self.is_valid_status(addr) {
            self.vram[adjusted_addr as usize] = val;
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
    pub fn read_vram(&self, addr: u16) -> u8 {
        let adjusted_addr = addr - VRAM_OFFSET as u16;
        self.vram[adjusted_addr as usize]
    }

    /// ```
    /// Set LY register
    ///
    /// Sets the value at the LY RAM address
    ///
    /// Input:
    ///     Value to write (u8)
    /// ```
    pub fn set_ly(&mut self, line: u8) {
        self.vram[LY] = line;
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
        self.vram[LCD_STAT_REG] &= 0b1111_1100;
        self.vram[LCD_STAT_REG] |= mode;
    }

    /// ```
    /// Get palette
    ///
    /// Gets the palette indices from the BGP register ($FF47)
    ///
    /// Output:
    ///     Palette indices ([u8])
    /// ```
    pub fn get_palette(&self) -> [u8; 4] {
        unpack_u8(self.vram[BGP])
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
        let tiles = self.get_background_tiles();

        if self.is_bkgd_dspl() {
            self.render_background(&mut map_array, &tiles);
        }

        if self.is_wndw_dspl() {
            self.render_window(&mut map_array, &tiles);
        }

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
    ///     Background tile data (&Vec<Tile>)
    /// ```
    fn render_background(&self, pixel_array: &mut [u8], bkgd: &Vec<Tile>) {
        let tile_map = self.get_bkgd_tile_map();

        // The tile indexes in the second tile pattern table ($8800-97ff) are signed
        let signed_offset = if self.get_bkgd_tile_set_index() == 0 { 128 } else { 0 };

        // Iterate through every tile in map
        for y in 0..MAP_SIZE {
            for x in 0..MAP_SIZE {
                let index = y * MAP_SIZE + x;
                let tile_index = tile_map[index];
                let tile = &bkgd[(tile_index + signed_offset) as usize];

                // Iterate through row in tile
                for row in 0..TILESIZE {
                    let map_x = TILESIZE * x;
                    let map_y = (TILESIZE * y) + row;
                    let map_index = (map_y * MAP_SIZE * TILESIZE) + map_x;
                    // Copy row into pixel map
                    pixel_array[map_index..(map_index + TILESIZE)].copy_from_slice(tile.get_row(row));
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
    ///     Window tile data (&Vec<Tile>)
    /// ```
    fn render_window(&self, pixel_array: &mut [u8], wndw: &Vec<Tile>) {
        let coords = self.get_wndw_coords();
        let wndw_map = self.get_wndw_tile_map();

        // Iterate through all pixels in window
        for y in (coords.1)..(coords.1 + SCREEN_HEIGHT) {
            for x in (coords.0)..(coords.0 + SCREEN_WIDTH) {
                let index = y * MAP_PIXELS + x;
                let pixel = pixel_array[index];

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
        for y in (scroll.1)..(scroll.1 + SCREEN_HEIGHT) {
            for x in (scroll.0)..(scroll.0 + SCREEN_WIDTH) {
                let index = y * MAP_PIXELS + x;
                let pixel = pixel_array[index];

                let view_index = (y - scroll.1) * SCREEN_WIDTH + (x - scroll.0);
                viewport[view_index] = pixel;
            }
        }

        viewport
    }

    /// ```
    /// Get background tiles
    ///
    /// Fetches the indices of background tiles from VRAM
    ///
    /// Output:
    ///     A vector of tile objects (Vec<Tile>)
    /// ```
    fn get_background_tiles(&self) -> Vec<Tile> {
        // Tile set is the tile pixel data
        // Tile map are the tile indices that make up the current background image
        // TODO: This 100% can and should be cached
        let mut map = Vec::new();
        let tile_set = self.get_tile_set();
        let num_tiles = tile_set.len() / (2 * TILESIZE);

        for i in 0..num_tiles {
            let tile_data = &tile_set[(2 * TILESIZE * i)..(2 * TILESIZE * (i + 1))];
            let tile = Tile::new(tile_data);
            map.push(tile);
        }

        map
    }

    /// ```
    /// Get tile set
    ///
    /// Gets the tileset indices currently in use for background and window layers
    ///
    /// Output:
    ///     Slice of tileset indices (&[u8])
    /// ```
    fn get_tile_set(&self) -> &[u8] {
        // $01 for $8000-$8FFF
        // $00 for $8800-$97FF
        let tile_set = if self.get_bkgd_tile_set_index() == 1 {
            &self.vram[TILE_SET_0_RANGE]
        } else {
            &self.vram[TILE_SET_1_RANGE]
        };

        tile_set
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
    /// Is background displayed
    ///
    /// Is background layer currently visible
    ///
    /// Output:
    ///     Whether or not background is displayed (bool)
    /// ```
    fn is_bkgd_dspl(&self) -> bool {
        let lcd_control = self.vram[LCD_DISP_REG];
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
        let lcd_control = self.vram[LCD_DISP_REG];
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
        let lcd_control = self.vram[LCD_DISP_REG];
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
        let lcd_control = self.vram[LCD_DISP_REG];
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
        let lcd_control = self.vram[LCD_DISP_REG];
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
        let lcd_control = self.vram[LCD_DISP_REG];
        if lcd_control.get_bit(6) { return 1 } else { return 0 }
    }

    /// ```
    /// Get scroll coords
    ///
    /// Returns the values of the SCX and SCY registers
    ///
    /// Output:
    ///     Tuple of SCX, SCY ((usize, usize))
    /// ```
    fn get_scroll_coords(&self) -> (usize, usize) {
        let scroll_x = self.vram[SCX] as usize;
        let scroll_y = self.vram[SCY] as usize;

        (scroll_x, scroll_y)
    }

    /// ```
    /// Get window coords
    ///
    /// Returns the window position from the WX and WY registers
    ///
    /// Output:
    ///     Location of the window ((usize, usize))
    fn get_wndw_coords(&self) -> (usize, usize) {
        let wndw_x = (self.vram[WX] - 7) as usize;
        let wndw_y = self.vram[WY] as usize;

        (wndw_x, wndw_y)
    }

    fn get_LCDC_status(&self) -> ModeTypes {
        let lcd_stat = self.vram[LCD_STAT_REG];
        let mode = lcd_stat & 0b0000_0011;
        match mode {
            0 => { ModeTypes::HBLANK },
            1 => { ModeTypes::VBLANK },
            2 => { ModeTypes::OAMReadMode },
            3 => { ModeTypes::VRAMReadMode },
            _ => { panic!("Invalid mode") }
        }
    }

    fn is_valid_status(&self, addr: u16) -> bool {
        let lcdc_status = self.get_LCDC_status();

        match lcdc_status {
            ModeTypes::OAMReadMode => {
                !OAM_MEM_RANGE.contains(&(addr as usize))
            },
            ModeTypes::VRAMReadMode => {
                !OAM_MEM_RANGE.contains(&(addr as usize)) && !DISPLAY_RAM_RANGE.contains(&(addr as usize))
            },
            _ => {
                true
            }
        }
    }
}
