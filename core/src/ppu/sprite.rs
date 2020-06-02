use crate::utils::{ModifyBits, Point};

/*
 * Object Attribute Memory (OAM) Layout
 *
 * Byte | Info
 * -----+------------------------------
 * 0    | Y-coord (stored y-coord - 16)
 * 1    | X-coord (stored x-coord - 8)
 * 2    | Data tile number
 * -----+------------------------------
 * 3    | Bit | Info
 * -----+-----+------------------------
 * 3    | 7   | Drawn above background when reset
 * 3    | 6   | Vertical flip when set
 * 3    | 5   | Horizontal flip when set
 * 3    | 4   | OBJ Palette 0/1
 * 3    | 3   | Tile VRAM Bank (CGB only)
 * 3    | 2-0 | Palette number (CGB only)
 *
**/

const X_OFFSET: u8 = 8;
const Y_OFFSET: u8 = 16;

pub struct Sprite {
    tile_num: u8,
    x: u8,
    y: u8,
    above_bkgd: bool,
    x_flip: bool,
    y_flip: bool,
    palette_0: bool
}

impl Sprite {
    pub fn new() -> Sprite {
        Sprite {
            tile_num: 0,
            x: 0,
            y: 0,
            above_bkgd: true,
            x_flip: false,
            y_flip: false,
            palette_0: true
        }
    }

    // NOTE: I'm not sure if this will ever be used
    pub fn init(&mut self, metadata: [u8; 4]) {
        self.update_spr(metadata);
    }

    pub fn update_spr(&mut self, data: [u8; 4]) {
        self.parse_oam_bytes(data);
    }

    pub fn update_byte(&mut self, index: u16, byte: u8) {
        match index {
            0 => { self.parse_oam_byte1(byte); },
            1 => { self.parse_oam_byte2(byte); },
            2 => { self.parse_oam_byte3(byte); },
            3 => { self.parse_oam_byte4(byte); },
            _ => { panic!("Byte offset can only be from 0-3"); }
        }
    }

    pub fn get_tile_num(&self) -> u8 {
        self.tile_num
    }

    pub fn get_coords(&self) -> Point {
        let x = self.x - X_OFFSET;
        let y = self.y - Y_OFFSET;
        Point::new(x, y)
    }
}

impl Sprite {
    fn parse_oam_bytes(&mut self, data: [u8; 4]) {
        self.parse_oam_byte1(data[0]);
        self.parse_oam_byte2(data[1]);
        self.parse_oam_byte3(data[2]);
        self.parse_oam_byte4(data[3]);
    }

    fn parse_oam_byte1(&mut self, val: u8) {
        self.y = val - Y_OFFSET;
    }

    fn parse_oam_byte2(&mut self, val: u8) {
        self.x = val - X_OFFSET;
    }

    fn parse_oam_byte3(&mut self, val: u8) {
        self.tile_num = val;
    }

    fn parse_oam_byte4(&mut self, val: u8) {
        self.above_bkgd = !val.get_bit(7);
        self.x_flip = !val.get_bit(6);
        self.y_flip = !val.get_bit(5);
        self.palette_0 = val.get_bit(4);
    }
}
