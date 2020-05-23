use crate::bus::Bus;
use crate::utils::get_bit;

const DIV_REG: u16 = 0xFF04; // Divider register
const CNT_REG: u16 = 0xFF05; // Counter register
const MOD_REG: u16 = 0xFF06; // Modulo register
const CON_REG: u16 = 0xFF07; // Control register

const TIMER_SPEED_IN_CYCLES: u8 = 16;
const COUNT_SPEED_IN_CYCLES: [u8; 4] = [64, 1, 4, 16];

pub struct Timer {
    div_cycles: u8,
    cnt_cycles: u8
}

impl Timer {
    pub fn new() -> Timer {
        Timer {
            div_cycles: 0,
            cnt_cycles: 0
        }
    }

    pub fn step(&mut self, bus: &mut Bus) {
        let con = bus.read_ram(CON_REG);
        if con.get_bit(2) {
            let cnt_spd = COUNT_SPEED_IN_CYCLES[con & 0b0000_0011];
            self.div_cycles += 1
            self.cnt_cycles += 1;

        }

    }
}
