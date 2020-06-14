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

const X_OFFSCREEN: u8 = 168;
const Y_OFFSCREEN: u8 = 160;

#[derive(Copy, Clone)]
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

    /// ```
    /// Update byte
    ///
    /// Updates metadata for this sprite
    ///
    /// Inputs:
    ///     Which metadata byte to edit (u16)
    ///     New byte value (u8)
    /// ```
    pub fn update_byte(&mut self, index: u16, byte: u8) {
        match index {
            0 => { self.parse_oam_byte1(byte); },
            1 => { self.parse_oam_byte2(byte); },
            2 => { self.parse_oam_byte3(byte); },
            3 => { self.parse_oam_byte4(byte); },
            _ => { panic!("Byte offset can only be from 0-3"); }
        }
    }

    /// ```
    /// Is onscreen
    ///
    /// Is sprite onscreen?
    ///
    /// Output:
    ///     Is sprite onscreen (bool)
    /// ```
    pub fn is_onscreen(&self) -> bool {
        let x_visible = self.x > 0 && self.x < X_OFFSCREEN;
        let y_visible = self.y > 0 && self.y < Y_OFFSCREEN;

        x_visible || y_visible
    }

    /// ```
    /// Get tile num
    ///
    /// Gets associated tile number for this sprite
    ///
    /// Output:
    ///     Tile num (u8)
    /// ```
    pub fn get_tile_num(&self) -> u8 {
        self.tile_num
    }

    /// ```
    /// Get coords
    ///
    /// Get screen coordinates for this sprite
    ///
    /// Output:
    ///     Sprite coordinates (Point)
    /// ```
    pub fn get_coords(&self) -> Point {
        let x = self.x;
        let y = self.y;
        Point::new(x, y)
    }

    /// ```
    /// Is palette 0?
    ///
    /// Whether this sprite uses palette 0 or 1
    ///
    /// Output:
    ///     Whether sprite uses sprite palette 0 (bool)
    /// ```
    pub fn is_pal_0(&self) -> bool {
        self.palette_0
    }

    /// ```
    /// Is X flipped?
    ///
    /// Should the sprite be flipped in the X-direction (up-down)?
    ///
    /// Output:
    ///     Whether sprite should be flipped
    /// ```
    pub fn is_x_flip(&self) -> bool {
        self.x_flip
    }

    /// ```
    /// Is Y flipped?
    ///
    /// Should the sprite be flipped in the Y-direction (left-right)?
    ///
    /// Output:
    ///     Whether sprite should be flipped
    /// ```
    pub fn is_y_flip(&self) -> bool {
        self.y_flip
    }
}

impl Sprite {
    /// ```
    /// Parse OAM byte 1
    ///
    /// Parses byte corresponding to sprite Y-coordinate
    ///
    /// Input:
    ///     Value to parse (u8)
    /// ```
    fn parse_oam_byte1(&mut self, val: u8) {
        self.y = val.wrapping_sub(Y_OFFSET);
    }

    /// ```
    /// Parse OAM byte 2
    ///
    /// Parses byte corresponding to sprite X-coordinate
    ///
    /// Input:
    ///     Value to parse (u8)
    /// ```
    fn parse_oam_byte2(&mut self, val: u8) {
        self.x = val.wrapping_sub(X_OFFSET);
    }

    /// ```
    /// Parse OAM byte 3
    ///
    /// Parses byte corresponding to sprite tile number
    ///
    /// Input:
    ///     Value to parse (u8)
    /// ```
    fn parse_oam_byte3(&mut self, val: u8) {
        self.tile_num = val;
    }

    /// ```
    /// Parse OAM byte 4
    ///
    /// Parses byte corresponding to X/Y flip, palette choice, and draw priority
    ///
    /// Input:
    ///     Value to parse (u8)
    /// ```
    fn parse_oam_byte4(&mut self, val: u8) {
        self.above_bkgd = !val.get_bit(7);
        self.x_flip = !val.get_bit(6);
        self.y_flip = !val.get_bit(5);
        self.palette_0 = !val.get_bit(4);
    }
}
