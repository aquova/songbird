extern crate sdl2;

use crate::utils::*;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::pixels::Color;
use sdl2::video::Window;
use std::ops::Range;

// =============
// = Constants =
// =============

const TILESIZE: usize = 8;
const MAPSIZE: usize = 32 * TILESIZE;
const SCREEN_WIDTH: usize = 160;
const SCREEN_HEIGHT: usize = 144;

const VRAM_SIZE: usize = 0x8000;
const VRAM_OFFSET: usize = 0x8000;

// VRAM registers
const LCD_DISP_REG: usize            = 0xFF40 - VRAM_OFFSET;
const LCD_STAT_REG: usize            = 0xFF41 - VRAM_OFFSET;
const SCY: usize                     = 0xFF42 - VRAM_OFFSET;
const SCX: usize                     = 0xFF43 - VRAM_OFFSET;
const LY: usize                      = 0xFF44 - VRAM_OFFSET;
const LYC: usize                     = 0xFF45 - VRAM_OFFSET;
const BGP: usize                     = 0xFF46 - VRAM_OFFSET;
const OBP0: usize                    = 0xFF47 - VRAM_OFFSET;
const OBP1: usize                    = 0xFF48 - VRAM_OFFSET;
const WY: usize                      = 0xFF49 - VRAM_OFFSET;
const WX: usize                      = 0xFF4A - VRAM_OFFSET;

// VRAM ranges
const TILE_SET_0_RANGE: Range<usize> = (0x8000 - VRAM_OFFSET)..(0x9000 - VRAM_OFFSET);
const TILE_SET_1_RANGE: Range<usize> = (0x8800 - VRAM_OFFSET)..(0x9800 - VRAM_OFFSET);
const TILE_MAP_0_RANGE: Range<usize> = (0x9800 - VRAM_OFFSET)..(0x9C00 - VRAM_OFFSET);
const TILE_MAP_1_RANGE: Range<usize> = (0x9C00 - VRAM_OFFSET)..(0xA000 - VRAM_OFFSET);
const SAM:              Range<usize> = (0xFE00 - VRAM_OFFSET)..(0xFEA0 - VRAM_OFFSET);

// Colors
const BLACK: (u8, u8, u8)            = (0,   0,   0);
const LIGHT_GRAY: (u8, u8, u8)       = (211, 211, 211);
const DARK_GRAY: (u8, u8, u8)        = (169, 169, 169);
const WHITE: (u8, u8, u8)            = (255, 255, 255);

const PALETTE: [(u8, u8, u8); 4] = [
    WHITE,
    DARK_GRAY,
    LIGHT_GRAY,
    BLACK,
];

pub struct PPU {
    vram: [u8; VRAM_SIZE]
}

impl PPU {
    // ==================
    // = Public methods =
    // ==================
    pub fn new() -> PPU {
        PPU {
            vram: [0xFF; VRAM_SIZE]
        }
    }

    /// ```
    /// Write VRAM
    ///
    /// Write value to specified address in VRAM
    ///
    /// Input:
    ///     Address to write to (u16)
    ///     Value to write (u8)
    /// ```
    pub fn write_vram(&mut self, addr: u16, val: u8) {
        let adjusted_addr = addr - VRAM_OFFSET as u16;
        self.vram[adjusted_addr as usize] = val;
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

    pub fn set_status(&mut self, mode: u8) {
        self.vram[LCD_STAT_REG] &= 0b1111_1100;
        self.vram[LCD_STAT_REG] |= mode;
    }

    pub fn draw_screen(&self, canvas: &mut Canvas<Window>) {
        // Clear window
        let draw_color = self.get_color(WHITE);
        canvas.set_draw_color(draw_color);
        canvas.clear();

        if self.is_bkgd_dspl() {
            self.draw_background(canvas);
        }

        canvas.present();
    }

    // ===================
    // = Private methods =
    // ===================
    fn draw_background(&self, canvas: &mut Canvas<Window>) {
        let scroll_x = self.vram[SCX] as usize;
        let scroll_y = self.vram[SCY] as usize;
        let bkgd = self.get_background();
        let dim = canvas.output_size().unwrap();
        let scale = (dim.0 as usize) / SCREEN_HEIGHT;

        for y in scroll_y..(scroll_y + SCREEN_WIDTH) {
            for x in scroll_x..(scroll_x + SCREEN_HEIGHT) {
                let i = y * MAPSIZE + x;
                let pixel = bkgd[i];
                if pixel != 0 {
                    canvas.set_draw_color(PALETTE[pixel as usize]);
                    let block = Rect::new(
                        (scale * x) as i32,
                        (scale * y) as i32,
                        scale as u32,
                        scale as u32,
                    );
                    canvas.fill_rect(block).unwrap();
                }
            }
        }
    }

    fn get_background(&self) -> [u8; MAPSIZE * MAPSIZE] {
        let mut map: [u8; MAPSIZE * MAPSIZE] = [0; MAPSIZE * MAPSIZE];
        let tile_set = self.get_bkgd_tile_set();
        let tile_map = self.get_bkgd_tile_map();

        // Iterate through tile_map, getting indices for tile_set. Store pixel values (0-3) into map
        for i in 0..tile_map.len() {
            let index = tile_map[i];
            // This is one row of a tile
            let row_low = tile_set[index as usize]; // May need to change to be RAM index, rather than offset
            let row_high = tile_set[(index + 1) as usize];
            let row = self.get_pixel_row(row_low, row_high);

            // Copy into map
            &map[TILESIZE * i..TILESIZE * (i + 1)].copy_from_slice(&row);
        }

        map
    }

    fn get_pixel_row(&self, low: u8, high: u8) -> [u8; 8] {
        let mut output = [0; 8];
        for i in 0..8 {
            let low_bit = low.get_bit(i);
            let high_bit = high.get_bit(i);
            let concat = self.concat_bits(low_bit, high_bit);
            output[7-i as usize] = concat;
        }

        output
    }

    fn concat_bits(&self, low: bool, high: bool) -> u8 {
        let low_bit = if low { 1 } else { 0 };
        let high_bit = if high { 1 } else { 0 };
        let concat = (high_bit << 1) | low_bit;
        concat
    }

    fn get_color(&self, color: (u8, u8, u8)) -> Color {
        Color::RGB(color.0, color.1, color.2)
    }

    fn get_bkgd_tile_set(&self) -> &[u8] {
        let tile_set = if self.get_bkgd_tile_set_index() == 0 {
            &self.vram[TILE_SET_0_RANGE]
        } else {
            &self.vram[TILE_SET_1_RANGE]
        };

        tile_set
    }

    fn get_bkgd_tile_map(&self) -> &[u8] {
        let tile_map = if self.get_bkgd_tile_map_index() == 0 {
            &self.vram[TILE_MAP_0_RANGE]
        } else {
            &self.vram[TILE_MAP_1_RANGE]
        };

        tile_map
    }

    fn is_bkgd_dspl(&self) -> bool {
        let lcd_control = self.vram[LCD_DISP_REG];
        lcd_control.get_bit(0)
    }

    fn get_bkgd_tile_set_index(&self) -> u8 {
        let lcd_control = self.vram[LCD_DISP_REG];
        if lcd_control.get_bit(4) { return 1 } else { return 0 }
    }

    fn get_bkgd_tile_map_index(&self) -> u8 {
        let lcd_control = self.vram[LCD_DISP_REG];
        if lcd_control.get_bit(3) { return 1 } else { return 0 }
    }
}
