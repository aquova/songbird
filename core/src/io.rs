// Input/Output functions

pub enum Buttons {
    Start,
    Select,
    B,
    A,
    Down,
    Up,
    Left,
    Right
}

impl Buttons {
    pub fn get_index(&self) -> usize {
        match self {
            Buttons::Start =>  { 0 },
            Buttons::Select => { 1 },
            Buttons::B =>      { 2 },
            Buttons::A =>      { 3 },
            Buttons::Down =>   { 4 },
            Buttons::Up =>     { 5 },
            Buttons::Left =>   { 6 },
            Buttons::Right =>  { 7 },
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

    pub fn btn_pressed(&mut self, btn: Buttons) {
        let i = btn.get_index();
        self.btns[i] = true;
    }

    pub fn btn_released(&mut self, btn: Buttons) {
        let i = btn.get_index();
        self.btns[i] = false;
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

    // TODO: See if these functions can be merged
    fn pack_btn_keys(&self) -> u8 {
        let mut output = 0b1110_0000;
        for i in 0..4 {
            // 0 if pressed, 1 if unpressed
            let pressed = if self.btns[i] { 0 } else { 1 };
            output |= (pressed << (3 - i));
        }

        output
    }

    fn pack_dir_keys(&self) -> u8 {
        let mut output = 0b1101_0000;
        for i in 4..8 {
            // 0 if pressed, 1 if unpressed
            let pressed = if self.btns[i] { 0 } else { 1 };
            output |= (pressed << (7 - i));
        }

        output
    }
}
