extern crate sdl2;

mod bkgd;

use bkgd::Tile;
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
const SCREEN_WIDTH: usize = 160;
const SCREEN_HEIGHT: usize = 144;
const MAP_WIDTH: usize = SCREEN_WIDTH / TILESIZE;
const MAP_HEIGHT: usize = SCREEN_HEIGHT / TILESIZE;

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
        let draw_color = Color::RGB(255, 255, 255);
        canvas.set_draw_color(draw_color);
        canvas.clear();

        self.draw_background(canvas);
        // if self.is_bkgd_dspl() {
        //     self.draw_background(canvas);
        // }

        canvas.present();
    }

    // ===================
    // = Private methods =
    // ===================
    fn draw_background(&self, canvas: &mut Canvas<Window>) {
        // let scroll_x = self.vram[SCX] as usize;
        // let scroll_y = self.vram[SCY] as usize;
        let bkgd = self.get_background();
        let dim = canvas.output_size().unwrap();
        let scale = (dim.0 as usize) / SCREEN_HEIGHT;

        let tile_map = self.get_bkgd_tile_map();

        // TODO: Only draw window at (SCX, SCY)
        for y in 0..MAP_HEIGHT {
            for x in 0..MAP_WIDTH {
                let index = y * MAP_HEIGHT + x;
                let tile_index = tile_map[index];
                let tile = &bkgd[tile_index as usize];
                tile.draw(x, y, scale, canvas);
            }
        }
    }

    // Tile set is the tile pixel data
    // Tile map are the tile indices that make up the current background image
    // TODO: This 100% can and should be cached
    fn get_background(&self) -> Vec<Tile> {
        let mut map = Vec::new();
        let tile_set = self.get_bkgd_tile_set();
        let num_tiles = tile_set.len() / (2 * TILESIZE);

        for i in 0..num_tiles {
            let tile_data = &tile_set[(2 * TILESIZE * i)..(2 * TILESIZE * (i + 1))];
            let tile = Tile::new(tile_data);
            map.push(tile);
        }

        map
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
