use crate::utils::ModifyBits;

pub const DIV: u16 = 0xFF04; // Divider register
pub const TIMA: u16 = 0xFF05; // Counter register
pub const TMA: u16 = 0xFF06; // Modulo register
pub const TAC: u16 = 0xFF07; // Control register

pub struct Timer {
    running: bool,
    internal_cnt: u16,
    tac: u8,
    counter: u8,
    modulo: u8,
    overflow: bool,
}

impl Default for Timer {
    fn default() -> Self {
        Self::new()
    }
}

// Much of the implementation for the timer is based off the Mooneye emulator
// https://github.com/Gekkio/mooneye-gb
impl Timer {
    pub fn new() -> Timer {
        Timer {
            running: false,
            internal_cnt: 0,
            tac: 0,
            counter: 0,
            modulo: 0,
            overflow: false,
        }
    }

    fn counter_mask(&self) -> u16 {
        match self.tac & 0b11 {
            0b11 => (1 << 5),
            0b10 => (1 << 3),
            0b01 => (1 << 1),
            _ => (1 << 7)
        }
    }

    fn counter_bit(&self) -> bool {
        (self.internal_cnt & self.counter_mask()) != 0
    }

    fn increment(&mut self) {
        let (counter, overflow) = self.counter.overflowing_add(1);
        self.counter = counter;
        self.overflow = overflow;
    }

    pub fn tick(&mut self) -> bool {
        let mut interrupt = false;

        if self.overflow {
            self.internal_cnt = self.internal_cnt.wrapping_add(1);
            self.counter = self.modulo;
            self.overflow = false;
            interrupt = true;
        } else if self.running && self.counter_bit() {
            self.internal_cnt = self.internal_cnt.wrapping_add(1);
            let new_bit = self.counter_bit();
            if !new_bit {
                self.increment();
            }
        } else {
            self.internal_cnt = self.internal_cnt.wrapping_add(1);
        }

        interrupt
    }

    pub fn read_timer(&mut self, addr: u16, inter: &mut bool) -> u8 {
        *inter = self.tick();

        match addr {
            DIV => {
                (self.internal_cnt >> 6) as u8
            },
            TIMA => {
                self.counter
            },
            TMA => {
                self.modulo
            },
            TAC => {
                0b1111_1100 | self.tac
            },
            _ => { panic!("Trying to read a non-timer register") }
        }
    }

    pub fn write_timer(&mut self, addr: u16, val: u8, inter: &mut bool) {
        match addr {
            DIV => {
                *inter = self.tick();
                if self.counter_bit() {
                    self.increment();
                }
                self.internal_cnt = 0;
            },
            TIMA => {
                let overflow = self.overflow;
                *inter = self.tick();
                if !overflow {
                    self.overflow = false;
                    self.counter = val;
                }
            },
            TMA => {
                let overflow = self.overflow;
                *inter = self.tick();
                self.modulo = val;
                if overflow {
                    self.counter = val;
                }
            },
            TAC => {
                *inter = self.tick();
                let old_bit = self.running && self.counter_bit();
                self.tac = val & 0b11;
                self.running = val.get_bit(2);
                let new_bit = self.running && self.counter_bit();
                if old_bit && !new_bit {
                    self.increment();
                }
            },
            _ => {
                panic!("Trying to write to non-timer register")
            }
        };
    }
}
