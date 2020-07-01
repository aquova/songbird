extern crate wasm_timer;
use wasm_timer::Instant;

const SECS_IN_MIN: u64 = 60;
const MIN_IN_HOURS: u64 = 60;
const HOURS_IN_DAYS: u64 = 24;

pub struct RTC {
    starttime: Instant,
    pub seconds: u8,
    pub minutes: u8,
    pub hours: u8,
    pub days: u16,
    halted: bool,
    enabled: bool,
}

impl RTC {
    pub fn new() -> RTC {
        RTC {
            starttime: Instant::now(),
            seconds: 0,
            minutes: 0,
            hours: 0,
            days: 0,
            halted: false,
            enabled: false,
        }
    }

    pub fn latch_time(&mut self) {
        let curr_time = Instant::now();
        let delta = curr_time.duration_since(self.starttime);
        let delta_sec = delta.as_secs();
        self.seconds = (delta_sec % SECS_IN_MIN) as u8;

        let delta_min = delta_sec / SECS_IN_MIN;
        self.minutes = (delta_min % MIN_IN_HOURS) as u8;

        let delta_hours = delta_min / MIN_IN_HOURS;
        self.hours = (delta_hours % HOURS_IN_DAYS) as u8;

        let delta_days = delta_hours / HOURS_IN_DAYS;
        self.days = delta_days as u16;
    }

    pub fn read_byte(&self, val: u8) -> u8 {
        match val {
            0x08 => { self.seconds },
            0x09 => { self.minutes },
            0x0A => { self.hours },
            0x0B => {
                (self.days & 0xFF) as u8
            },
            0x0C => {
                // Bit 0 is MSB of day counter
                // Bit 6 is set if halted
                // Bit 7 is day counter carry bit
                // All other bits are unused
                let bit_0 = ((self.days >> 8) & 0b1) as u8;
                let bit_6 = if self.halted { 0 } else { 1 };
                let bit_7 = ((self.days >> 9) & 0b1) as u8;

                let output = (bit_7 << 7) | (bit_6 << 6) | bit_0;
                output
            },
            _ => { unreachable!() }
        }
    }

    pub fn write_byte(&mut self, val: u8) {
        if val == 0x00 {
            self.enabled = false;
        } else if val == 0x01 {
            if !self.enabled {
                self.enabled = true;
                self.latch_time();
            }
        }
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
}
