use crate::utils::{ModifyBits, ModifyBytes};

pub const NR10: u16 = 0xFF10;
pub const NR11: u16 = 0xFF11;
pub const NR12: u16 = 0xFF12;
pub const NR13: u16 = 0xFF13;
pub const NR14: u16 = 0xFF14;

pub struct Channel1 {
    sound_on: bool,
    sweep_time: u8,
    sweep_dir: bool,
    sweep_shift_num: u8,
    wave_pattern_duty: u8,
    sound_len_data: u8,
    init_envelope_vol: u8,
    envelope_dir: bool,
    envelope_sweep_num: u8,
    freq_data: u16,
    freq_init: bool,
    freq_selection: bool,
}

impl Channel1 {
    pub fn new() -> Self {
        Self {
            sound_on: false,
            sweep_time: 0,
            sweep_dir: false,
            sweep_shift_num: 0,
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
            NR10 => {
                self.sweep_time << 4 | (self.sweep_dir as u8) << 3 | self.sweep_shift_num
            },
            NR11 => {
                self.wave_pattern_duty << 6
            },
            NR12 => {
                self.init_envelope_vol << 4 | (self.envelope_dir as u8) << 3 | self.envelope_sweep_num
            },
            NR13 => {
                (self.freq_data & 0xFF) as u8
            },
            NR14 => {
                (self.freq_selection as u8) << 6
            },
            _ => unreachable!("Address: {:#04x}", addr)
        }
    }

    pub fn write(&mut self, addr: u16, data: u8) {
        match addr {
            NR10 => {
                self.sweep_time = (data & 0x70) >> 4;
                self.sweep_dir = data.get_bit(3);
                self.sweep_shift_num = data & 0x7;
            },
            NR11 => {
                self.wave_pattern_duty = (data & 0xC0) >> 6;
                self.sound_len_data = data & 0x3F;
            },
            NR12 => {
                self.init_envelope_vol = (data & 0xF0) >> 4;
                self.envelope_dir = data.get_bit(3);
                self.envelope_sweep_num = data & 0x7;
            },
            NR13 => {
                self.freq_data.set_low_byte(data);
            },
            NR14 => {
                self.freq_init = data.get_bit(7);
                self.freq_selection = data.get_bit(6);
                self.freq_data.set_high_byte(data & 0x7);
            },
            _ => unreachable!("Address: {:#04x} Value: {:#02x}", addr, data)
        }
    }
}
