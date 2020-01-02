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

const VRAM_SIZE: usize = 0x2000;
const VRAM_OFFSET: u16 = 0x8000;

// VRAM registers
const LCD_DISP_REG: usize            = 0xFF40;
const LCD_STAT_REG: usize            = 0xFF41;
const SCY: usize                     = 0xFF42;
const SCX: usize                     = 0xFF43;
const LY: usize                      = 0xFF44;
const LYC: usize                     = 0xFF45;
const BGP: usize                     = 0xFF46;
const OBP0: usize                    = 0xFF47;
const OBP1: usize                    = 0xFF48;
const WY: usize                      = 0xFF49;
const WX: usize                      = 0xFF4A;

// VRAM ranges
const TILE_SET_0_RANGE: Range<usize> = 0x8000..0x9000;
const TILE_SET_1_RANGE: Range<usize> = 0x8800..0x9800;
const TILE_MAP_0_RANGE: Range<usize> = 0x9800..0x9C00;
const TILE_MAP_1_RANGE: Range<usize> = 0x9C00..0xA000;
const SAM:              Range<usize> = 0xFE00..0xFEA0;

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
            vram: [0; VRAM_SIZE]
        }
    }

    pub fn write_vram(&mut self, addr: u16, val: u8) {
        let adjusted_addr = addr - VRAM_OFFSET;
        self.vram[adjusted_addr as usize] = val;
    }

    pub fn read_vram(&self, addr: u16) -> u8 {
        let adjusted_addr = addr - VRAM_OFFSET;
        self.vram[adjusted_addr as usize]
    }

    // pub fn draw_screen(&self, ram: &[u8], canvas: &mut Canvas<Window>, scale: usize) {
    //     // Clear window
    //     let draw_color = self.get_color(WHITE);
    //     canvas.set_draw_color(draw_color);
    //     canvas.clear();

    //     let lcd_reg = ram[LCD_DISP_REG];
    //     // if self.is_bkgd_dspl(lcd_reg) {
    //     //     self.draw_background(ram, canvas, scale);
    //     // }

    //     canvas.present();
    // }

    // pub fn draw_tile_set(&self, tile_set: &[u8], canvas: &mut Canvas<Window>) {
    //     let draw_color = self.get_color(WHITE);
    //     canvas.set_draw_color(draw_color);
    //     canvas.clear();

    //     let num_pixels = tile_set.len() / 2;
    //     let mut x = 0;
    //     let mut y = 0;
    //     for i in 0..num_pixels {
    //         let low = tile_set[2 * i as usize];
    //         let high = tile_set[(2 * i + 1) as usize];
    //         let row = self.parse_tile_data(low, high);
    //         for index in 0..row.len() {
    //             let c = PALETTE[row[index] as usize];
    //             let pixel_color = self.get_color(c);
    //             canvas.set_draw_color(pixel_color);
    //             let pixel = Rect::new(
    //                 (x + index) as i32,
    //                 y as i32,
    //                 1,
    //                 1
    //             );
    //             canvas.fill_rect(pixel);
    //         }

    //         x += 8;
    //         if x > 127 {
    //             x = 0;
    //             y += 1;
    //         }
    //     }

    //     canvas.present();

        // pub fn get_tile_set(&self) -> &[u8] {
        //     let lcd_reg = self.ram[LCD_DISP_REG];

        //     let tile_set = if self.get_bkgd_tile_set(lcd_reg) == 0 {
        //         self.get_tile_set_0()
        //     } else {
        //         self.get_tile_set_1()
        //     };

        //     tile_set
        // }

        // pub fn get_tile_map(&self) -> &[u8] {
        //     let lcd_reg = self.ram[LCD_DISP_REG];
        //     let tile_map = if self.get_bkgd_tile_map(lcd_reg) == 0 {
        //         self.get_tile_map_0()
        //     } else {
        //         self.get_tile_map_1()
        //     };

        //     tile_map
        // }

        // pub fn get_sprite_attributes(&self) -> &[u8] {
        //     &self.ram[SAM]
        // }
    }

    // ===================
    // = Private methods =
    // ===================
    // fn draw_background(&self, tile_set: &[u8], tile_map: &[u8], canvas: &mut Canvas<Window>, scale: usize) {
    //     let coords = self.get_scroll_coords(ram);
    //     let bkgd = self.get_background(tile_set, tile_map);
    //     let start_x = coords.0 as usize;
    //     let start_y = coords.1 as usize;

    //     for y in start_y..(start_y + SCREEN_WIDTH) {
    //         for x in start_x..(start_x + SCREEN_HEIGHT) {
    //             let i = y * MAPSIZE + x;
    //             let pixel = bkgd[i];
    //             if pixel != 0 {
    //                 canvas.set_draw_color(PALETTE[pixel as usize]);
    //                 let block = Rect::new(
    //                     (scale * x) as i32,
    //                     (scale * y) as i32,
    //                     scale as u32,
    //                     scale as u32,
    //                 );
    //                 canvas.fill_rect(block);
    //             }
    //         }
    //     }
    // }

    // fn get_background(&self, tile_set: &[u8], tile_map: &[u8]) -> [u8; MAPSIZE * MAPSIZE] {
    //     let mut map: [u8; MAPSIZE * MAPSIZE] = [0; MAPSIZE * MAPSIZE];

    //     // Iterate through tile_map, getting indices for tile_set. Store pixel values (0-3) into map
    //     for i in 0..tile_map.len() {
    //         let index = tile_map[i];
    //         // This is one row of a tile
    //         let row_low = tile_set[index as usize]; // May need to change to be RAM index, rather than offset
    //         let row_high = tile_set[(index + 1) as usize];
    //         let row = self.parse_tile_data(row_low, row_high);

    //         // Copy into map
    //         &map[TILESIZE * i..TILESIZE * (i + 1)].copy_from_slice(&row);
    //     }

    //     map
    // }

    // fn parse_tile_data(&self, low: u8, high: u8) -> [u8; 8] {
    //     let mut output = [0; 8];
    //     for i in 0..8 {
    //         let low_bit = low.get_bit(i);
    //         let high_bit = high.get_bit(i);
    //         let concat = self.concat_bits(low_bit, high_bit);
    //         output[7-i as usize] = concat;
    //     }

    //     output
    // }

    // fn concat_bits(&self, low: bool, high: bool) -> u8 {
    //     let low_bit = if low { 1 } else { 0 };
    //     let high_bit = if high { 1 } else { 0 };
    //     let concat = (high_bit << 1) | low_bit;
    //     concat
    // }

    // fn get_color(&self, color: (u8, u8, u8)) -> Color {
    //     Color::RGB(color.0, color.1, color.2)
    // }

    // fn get_lcd_disp_reg(&self, ram: &[u8]) -> u8 {
    //     ram[LCD_DISP_REG]
    // }

    // fn get_lcd_stat_reg(&self, ram: &[u8]) -> u8 {
    //     ram[LCD_STAT_REG]
    // }

    // fn is_bkgd_dspl(&self, reg: u8) -> bool {
    //     reg.get_bit(0)
    // }

    // fn is_spr_dspl(&self, reg: u8) -> bool {
    //     reg.get_bit(1)
    // }

    // fn is_wndw_dspl(&self, reg: u8) -> bool {
    //     reg.get_bit(5)
    // }

    // fn is_lcd_enabled(&self, reg: u8) -> bool {
    //     reg.get_bit(7)
    // }

    // fn get_bkgd_tile_set(&self, reg: u8) -> u8 {
    //     if reg.get_bit(4) { return 1 } else { return 0 }
    // }

    // fn get_bkgd_tile_map(&self, reg: u8) -> u8 {
    //     if reg.get_bit(3) { return 1 } else { return 0 }
    // }

    // fn get_wndw_tile_map(reg: u8) -> u8 {
    //     if reg.get_bit(6) { return 1 } else { return 0 }
    // }

    // fn get_scroll_coords(&self, ram: &[u8]) -> (u8, u8) {
    //     let x_coord = ram[SCX];
    //     let y_coord = ram[SCY];

    //     (x_coord, y_coord)
    // }

    // fn get_wndw_coords(&self, ram: &[u8]) -> (u8, u8) {
    //     let x_win = ram[WX];
    //     let y_win = ram[WY];

    //     (x_win, y_win)
    // }

    // fn parse_sprite_attributes(&self, attrs: &[u8], offset: usize) -> (u8, u8, u8, u8) {
    //     let x = attrs[offset];
    //     let y = attrs[offset + 1];
    //     let tile_num = attrs[offset + 2];
    //     let flags = attrs[offset + 3];

    //     (x, y, tile_num, flags)
    // }
}
