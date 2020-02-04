use crate::utils::*;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::pixels::Color;
use sdl2::video::Window;

// =============
// = Constants =
// =============
const TILESIZE: usize = 8;

// Colors
const BLACK: (u8, u8, u8)            = (0,   0,   0);
const DARK_GRAY: (u8, u8, u8)        = (148, 148, 165);
const LIGHT_GRAY: (u8, u8, u8)       = (107, 107, 90);
const WHITE: (u8, u8, u8)            = (255, 255, 255);

// TODO: Need to reference palette register at some point
const PALETTE: [(u8, u8, u8); 4] = [
    BLACK,
    DARK_GRAY,
    LIGHT_GRAY,
    WHITE,
];

struct Row {
    pixels: [u8; TILESIZE]
}

impl Row {
    pub fn get_pixel(&self, index: usize) -> u8 {
        return self.pixels[index];
    }
}

pub struct Tile {
    rows: Vec<Row>
}

impl Tile {
    pub fn new(data: &[u8]) -> Tile {
        let mut new_tile = Tile {
            rows: Vec::new()
        };

        for i in 0..TILESIZE {
            let mut row = Row {
                pixels: [0; TILESIZE]
            };

            let low = data[2 * i];
            let high = data[2 * i + 1];
            row.pixels = get_pixel_row(low, high);

            new_tile.rows.push(row);
        }

        new_tile
    }

    pub fn draw(&self, x: usize, y: usize, scale: usize, canvas: &mut Canvas<Window>) {
        let x_start = x * TILESIZE * scale;
        let y_start = y * TILESIZE * scale;

        for y_index in 0..TILESIZE {
            let curr_row = &self.rows[y_index];
            for x_index in 0..TILESIZE {
                let pixel = curr_row.get_pixel(x_index);
                let color_val = PALETTE[pixel as usize];
                let color = get_color(color_val);
                canvas.set_draw_color(color);

                let rect = Rect::new(
                    (x_start + (x_index * scale)) as i32,
                    (y_start + (y_index * scale)) as i32,
                    scale as u32,
                    scale as u32
                );
                canvas.fill_rect(rect).expect("Unable to draw to canvas");
            }
        }
    }
}

fn get_pixel_row(low: u8, high: u8) -> [u8; TILESIZE] {
    let mut output = [0; TILESIZE];
    for i in 0..TILESIZE {
        let low_bit = low.get_bit(i as u8);
        let high_bit = high.get_bit(i as u8);
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
