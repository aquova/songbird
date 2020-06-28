use crate::utils::ModifyBits;

pub const DIV: u16 = 0xFF04; // Divider register
pub const TIMA: u16 = 0xFF05; // Counter register
pub const TMA: u16 = 0xFF06; // Modulo register
pub const TAC: u16 = 0xFF07; // Control register

// const TIMA_SPEED_IN_CYCLES: [u16; 4] = [1024, 16, 64, 256];
const DIV_SPEED_IN_CYCLES: u16 = 64;
const TIMA_SPEED_IN_CYCLES: [u16; 4] = [256, 4, 16, 64];

pub struct Timer {
    running: bool,
    div_cycles: u16,
    tima_cycles: u16,
    tima_index: usize,
    div: u8, // $FF04
    tima: u8, // $FF05
    tma: u8, // $FF06
}

impl Timer {
    pub fn new() -> Timer {
        Timer {
            running: false,
            div_cycles: 0,
            tima_cycles: 0,
            tima_index: 0,
            div: 0,
            tima: 0,
            tma: 0,
        }
    }

    pub fn tick(&mut self, cycles: u8) -> bool {
        let mut interrupt = false;

        // Timer clock runs slower than CPU clock
        // So timer registers only increment on set multiple of clock cycles
        // DIV always runs, while TIMA only runs when set
        self.div_cycles += cycles as u16;
        if self.div_cycles >= DIV_SPEED_IN_CYCLES {
            self.div = self.div.wrapping_add(1);
            self.div_cycles %= DIV_SPEED_IN_CYCLES;
        }

        if self.running {
            self.tima_cycles += cycles as u16;

            let cnt_spd = TIMA_SPEED_IN_CYCLES[self.tima_index];
            if self.tima_cycles >= cnt_spd {
                self.tima_cycles %= cnt_spd;
                let overflow = self.tima.checked_add(1);
                // If overflow, set Timer counter to Timer Modulo value
                if overflow.is_none() {
                    self.tima = self.tma;
                    interrupt = true;
                } else {
                    self.tima += 1;
                }
            }
        }

        interrupt
    }

    pub fn read_timer(&self, addr: u16) -> u8 {
        let val = match addr {
            DIV => { self.div },
            TIMA => { self.tima },
            TMA => { self.tma },
            TAC => {
                let running_val = if self.running { 0b100 } else { 0 };
                let output = running_val | (self.tima_index as u8);
                output
            },
            _ => { panic!("Trying to read a non-timer register") }
        };

        val
    }

    pub fn write_timer(&mut self, addr: u16, val: u8) {
        match addr {
            DIV => { self.div = 0 },
            TIMA => { self.tima = 0 },
            TMA => { self.tma = val },
            TAC => {
                self.running = val.get_bit(2);

                let clock_spd = val & 0x3;
                self.tima_index = clock_spd as usize;
            },
            _ => {
                panic!("Trying to write to non-timer register")
            }
        };
    }
}
