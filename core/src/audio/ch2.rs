use crate::utils::{ModifyBits, ModifyBytes};

pub const NR21: u16 = 0xFF16;
pub const NR22: u16 = 0xFF17;
pub const NR23: u16 = 0xFF18;
pub const NR24: u16 = 0xFF19;

pub struct Channel2 {
    sound_on: bool,
    wave_pattern_duty: u8,
    sound_len_data: u8,
    init_envelope_vol: u8,
    envelope_dir: bool,
    envelope_sweep_num: u8,
    freq_data: u16,
    freq_init: bool,
    freq_selection: bool,
}

impl Channel2 {
    pub fn new() -> Self {
        Self {
            sound_on: false,
            wave_pattern_duty: 0,
            sound_len_data: 0,
            init_envelope_vol: 0,
            envelope_dir: false,
            envelope_sweep_num: 0,
            freq_data: 0,
            freq_init: false,
            freq_selection: false,
        }
    }

    pub fn is_sound_on(&self) -> bool {
        self.sound_on
    }

    pub fn read(&self, addr: u16) -> u8 {
        match addr {
            NR21 => {
                self.wave_pattern_duty << 6
            },
            NR22 => {
                self.init_envelope_vol << 4 | (self.envelope_dir as u8) << 3 | self.envelope_sweep_num
            },
            NR23 => {
                (self.freq_data & 0xFF) as u8
            },
            NR24 => {
                (self.freq_selection as u8) << 6
            },
            _ => unreachable!("Address: {:#04x}", addr)
        }
    }

    pub fn write(&mut self, addr: u16, data: u8) {
        match addr {
            NR21 => {
                self.wave_pattern_duty = (data & 0xC0) >> 6;
                self.sound_len_data = data & 0x3F;
            },
            NR22 => {
                self.init_envelope_vol = (data & 0xF0) >> 4;
                self.envelope_dir = data.get_bit(3);
                self.envelope_sweep_num = data & 0x7;
            },
            NR23 => {
                self.freq_data.set_low_byte(data);
            },
            NR24 => {
                self.freq_init = data.get_bit(7);
                self.freq_selection = data.get_bit(6);
                self.freq_data.set_high_byte(data & 0x7);
            },
            _ => unreachable!("Address: {:#04x} Value: {:#02x}", addr, data)
        }
    }
}
