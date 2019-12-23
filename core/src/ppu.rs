extern crate sdl2;

use crate::utils::*;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::pixels::Color;
use sdl2::video::Window;

// =============
// = Constants =
// =============

const TILESIZE: usize = 8;
const MAPSIZE: usize = 32 * TILESIZE;
const SCREEN_WIDTH: usize = 160;
const SCREEN_HEIGHT: usize = 144;

// VRAM ranges
const TILE_SET_0_RANGE: std::ops::Range<usize> = 0x8000..0x9000;
const TILE_SET_1_RANGE: std::ops::Range<usize> = 0x8800..0x9800;
const TILE_MAP_0_RANGE: std::ops::Range<usize> = 0x9800..0x9C00;
const TILE_MAP_1_RANGE: std::ops::Range<usize> = 0x9C00..0xA000;
const SAM: std::ops::Range<usize>              = 0xFE00..0xFEA0;

// VRAM registers
const LCD_DISP_REG: usize                       = 0xFF40;
const LCD_STAT_REG: usize                       = 0xFF41;
const SCY: usize                                = 0xFF42;
const SCX: usize                                = 0xFF43;
const LY: usize                                 = 0xFF44;
const LYC: usize                                = 0xFF45;
const BGP: usize                                = 0xFF46;
const OBP0: usize                               = 0xFF47;
const OBP1: usize                               = 0xFF48;
const WY: usize                                 = 0xFF49;
const WX: usize                                 = 0xFF4A;

// Colors
const BLACK: (u8, u8, u8)                       = (0,   0,   0);
const LIGHT_GRAY: (u8, u8, u8)                  = (211, 211, 211);
const DARK_GRAY: (u8, u8, u8)                   = (169, 169, 169);
const WHITE: (u8, u8, u8)                       = (255, 255, 255);

const PALETTE: [(u8, u8, u8); 4] = [
    BLACK,
    LIGHT_GRAY,
    DARK_GRAY,
    WHITE
];

// ==================
// = Public methods =
// ==================
pub fn draw_screen(ram: &[u8], canvas: &mut Canvas<Window>, scale: usize) {
    // Clear window
    let black_color = get_color(BLACK);
    canvas.set_draw_color(black_color);
    canvas.clear();

    let LCD_reg = ram[LCD_DISP_REG];
    if is_bkgd_dspl(LCD_reg) {
        draw_background(ram, canvas, scale);
    }

    canvas.present();
}

pub fn draw_tile_set(ram: &[u8], canvas: &mut Canvas<Window>) {
    let black_color = get_color(BLACK);
    canvas.set_draw_color(black_color);
    canvas.clear();

    let lcd_reg = ram[LCD_DISP_REG];
    let tile_set = if get_bkgd_tile_set(lcd_reg) == 0 {
        get_tile_set_0(ram)
    } else {
        get_tile_set_1(ram)
    };

    let num_pixels = tile_set.len() / 2;
    let mut x = 0;
    let mut y = 0;
    for i in 0..num_pixels {
        let low = tile_set[2 * i as usize];
        let high = tile_set[(2 * i + 1) as usize];
        let row = parse_tile_data(low, high);
        for index in 0..row.len() {
            let c = PALETTE[index];
            let pixel_color = get_color(c);
            canvas.set_draw_color(pixel_color);
            let pixel = Rect::new(
                (x + index) as i32,
                y as i32,
                1,
                1
            );
            canvas.fill_rect(pixel);
        }

        x += 8;
        if x > 255 {
            x = 0;
            y += 1;
        }
    }

    canvas.present();
}

// ===================
// = Private methods =
// ===================
fn draw_background(ram: &[u8], canvas: &mut Canvas<Window>, scale: usize) {
    let coords = get_scroll_coords(ram);
    let bkgd = get_background(ram);
    let start_x = coords.0 as usize;
    let start_y = coords.1 as usize;

    for y in start_y..(start_y + SCREEN_WIDTH) {
        for x in start_x..(start_x + SCREEN_HEIGHT) {
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
                canvas.fill_rect(block);
            }
        }
    }
}

fn get_background(ram: &[u8]) -> [u8; MAPSIZE * MAPSIZE] {
    let mut map: [u8; MAPSIZE * MAPSIZE] = [0; MAPSIZE * MAPSIZE];
    let LCD_reg = ram[LCD_DISP_REG];
    let tile_map = if get_bkgd_tile_map(LCD_reg) == 0 {
        get_tile_map_0(ram)
    } else {
        get_tile_map_1(ram)
    };

    let tile_set = if get_bkgd_tile_set(LCD_reg) == 0 {
        get_tile_set_0(ram)
    } else {
        get_tile_set_1(ram)
    };

    // Iterate through tile_map, getting indices for tile_set. Store pixel values (0-3) into map
    for i in 0..tile_map.len() {
        let index = tile_map[i];
        // This is one row of a tile
        let row_low = tile_set[index as usize]; // May need to change to be RAM index, rather than offset
        let row_high = tile_set[(index + 1) as usize];
        let row = parse_tile_data(row_low, row_high);

        // Copy into map
        &map[TILESIZE * i..TILESIZE * (i + 1)].copy_from_slice(&row);
    }

    map
}

fn parse_tile_data(low: u8, high: u8) -> [u8; 8] {
    let mut output = [0; 8];
    for i in 0..8 {
        let low_bit = low.get_bit(i);
        let high_bit = high.get_bit(i);
        let concat = concat_bits(low_bit, high_bit);
        output[7-i as usize] = concat;
    }

    output
}

fn concat_bits(low: bool, high: bool) -> u8 {
    let low_bit = if low { 1 } else { 0 };
    let high_bit = if high { 1 } else { 0 };
    let concat = (high_bit << 1) | low_bit;
    concat
}

fn get_color(color: (u8, u8, u8)) -> Color {
    Color::RGB(color.0, color.1, color.2)
}


fn get_tile_set_0(ram: &[u8]) -> &[u8] {
    &ram[TILE_SET_0_RANGE]
}

fn get_tile_set_1(ram: &[u8]) -> &[u8] {
    &ram[TILE_SET_1_RANGE]
}

fn get_tile_map_0(ram: &[u8]) -> &[u8] {
    &ram[TILE_MAP_0_RANGE]
}

fn get_tile_map_1(ram: &[u8]) -> &[u8] {
    &ram[TILE_MAP_1_RANGE]
}

fn get_sprite_attributes(ram: &[u8]) -> &[u8] {
    &ram[SAM]
}

fn get_lcd_disp_reg(ram: &[u8]) -> u8 {
    ram[LCD_DISP_REG]
}

fn get_lcd_stat_reg(ram: &[u8]) -> u8 {
    ram[LCD_STAT_REG]
}

fn is_bkgd_dspl(reg: u8) -> bool {
    reg.get_bit(0)
}

fn is_spr_dspl(reg: u8) -> bool {
    reg.get_bit(1)
}

fn is_wndw_dspl(reg: u8) -> bool {
    reg.get_bit(5)
}

fn is_lcd_enabled(reg: u8) -> bool {
    reg.get_bit(7)
}

fn get_bkgd_tile_map(reg: u8) -> u8 {
    if reg.get_bit(3) { return 1 } else { return 0 }
}

fn get_bkgd_tile_set(reg: u8) -> u8 {
    if reg.get_bit(4) { return 1 } else { return 0 }
}

fn get_wndw_tile_map(reg: u8) -> u8 {
    if reg.get_bit(6) { return 1 } else { return 0 }
}

fn get_scroll_coords(ram: &[u8]) -> (u8, u8) {
    let x_coord = ram[SCX];
    let y_coord = ram[SCY];

    (x_coord, y_coord)
}

fn get_wndw_coords(ram: &[u8]) -> (u8, u8) {
    let x_win = ram[WX];
    let y_win = ram[WY];

    (x_win, y_win)
}

fn parse_sprite_attributes(ram: &[u8], offset: usize) -> (u8, u8, u8, u8) {
    let attrs = get_sprite_attributes(ram);
    let x = attrs[offset];
    let y = attrs[offset + 1];
    let tile_num = attrs[offset + 2];
    let flags = attrs[offset + 3];

    (x, y, tile_num, flags)
}
