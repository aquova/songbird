use crate::utils::ModifyBits;

pub const NR41: u16 = 0xFF20;
pub const NR42: u16 = 0xFF21;
pub const NR43: u16 = 0xFF22;
pub const NR44: u16 = 0xFF23;

pub struct Channel4 {
    sound_on: bool,
    sound_len_data: u8,
    init_envelope_vol: u8,
    envelope_dir: bool,
    envelope_sweep_num: u8,
    shift_clock_freq: u8,
    counter_step: bool,
    freq_ratio: u8,
    freq_init: bool,
    freq_selection: bool,
}

impl Channel4 {
    pub fn new() -> Self {
        Self {
            sound_on: false,
            sound_len_data: 0,
            init_envelope_vol: 0,
            envelope_dir: false,
            envelope_sweep_num: 0,
            shift_clock_freq: 0,
            counter_step: false,
            freq_ratio: 0,
            freq_init: false,
            freq_selection: false,
        }
    }

    pub fn is_sound_on(&self) -> bool {
        self.sound_on
    }

    pub fn read(&self, addr: u16) -> u8 {
        match addr {
            NR41 => {
                self.sound_len_data
            },
            NR42 => {
                self.init_envelope_vol << 4 | (self.envelope_dir as u8) << 3 | self.envelope_sweep_num
            },
            NR43 => {
                self.shift_clock_freq << 4 | (self.counter_step as u8) << 3 | self.freq_ratio
            },
            NR44 => {
                (self.freq_selection as u8) << 6
            },
            _ => unreachable!("Address: {:#04x}", addr)
        }
    }

    pub fn write(&mut self, addr: u16, data: u8) {
        match addr {
            NR41 => {
                self.sound_len_data = data;
            },
            NR42 => {
                self.init_envelope_vol = (data & 0xF0) >> 4;
                self.envelope_dir = data.get_bit(3);
                self.envelope_sweep_num = data & 0x7;
            },
            NR43 => {
                self.shift_clock_freq = (data & 0xF0) >> 4;
                self.counter_step = data.get_bit(3);
                self.freq_ratio = data & 0x7;
            },
            NR44 => {
                self.freq_init = data.get_bit(7);
                self.freq_selection = data.get_bit(6);
            },
            _ => unreachable!("Address: {:#04x} Value: {:#02x}", addr, data)
        }
    }
}
