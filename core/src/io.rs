// Input/Output functions

/*
 * Game Boy joypad layout
 * Address $FF00
 *
 * Bit | Function
 * ----+---------
 * 7   | Unused
 * 6   | Unused
 * 5   | High if polling for D-pad
 * 4   | High if polling for other btns
 * 3   | Down / Start
 * 2   | Up / Select
 * 1   | Left / B
 * 0   | Right / A
 *
 * CPU can only poll for D-pad or other buttons, not both.
 * Buttons will be LOW if pressed
 */

pub enum Buttons {
    A,
    B,
    Select,
    Start,
    Right,
    Left,
    Up,
    Down,
}

impl Buttons {
    fn get_index(&self) -> usize {
        // Buttons are expected to be in this order
        match self {
            Buttons::A =>       { 0 },
            Buttons::B =>       { 1 },
            Buttons::Select =>  { 2 },
            Buttons::Start =>   { 3 },
            Buttons::Right =>   { 4 },
            Buttons::Left =>    { 5 },
            Buttons::Up =>      { 6 },
            Buttons::Down =>    { 7 },
        }
    }
}

pub struct IO {
    btns: [bool; 8],
    get_btn_keys: bool,
    get_dir_keys: bool
}

impl IO {
    pub fn new() -> IO {
        IO {
            btns: [false; 8],
            get_btn_keys: false,
            get_dir_keys: false
        }
    }

    pub fn btn_toggle(&mut self, btn: Buttons, pressed: bool) {
        // Rust hashmaps leave much to be desired, so do it this way
        let i = btn.get_index();
        self.btns[i] = pressed;
    }

    pub fn set_btns(&mut self, val: u8) {
        self.get_btn_keys = (val & 0b0010_0000) != 0;
        self.get_dir_keys = (val & 0b0001_0000) != 0;
    }

    pub fn read_btns(&self) -> u8 {
        // AFAIK, the system can't ask for both values
        if self.get_btn_keys {
            self.pack_btn_keys()
        } else if self.get_dir_keys {
            self.pack_dir_keys()
        } else {
            0
        }
    }

    fn pack_btn_keys(&self) -> u8 {
        let mut output = 0;
        for i in 0..4 {
            // 0 if pressed, 1 if unpressed
            let pressed = if self.btns[i] { 0 } else { 1 };
            output |= pressed << i;
        }

        output
    }

    fn pack_dir_keys(&self) -> u8 {
        let mut output = 0;
        for i in 0..4 {
            // 0 if pressed, 1 if unpressed
            let pressed = if self.btns[i + 4] { 0 } else { 1 };
            output |= pressed << i;
        }

        output
    }
}
