use crate::utils::{ModifyBits, ModifyBytes};

pub const NR30: u16 = 0xFF1A;
pub const NR31: u16 = 0xFF1B;
pub const NR32: u16 = 0xFF1C;
pub const NR33: u16 = 0xFF1D;
pub const NR34: u16 = 0xFF1E;

pub const WAVE_RAM_START: u16 = 0xFF30;
pub const WAVE_RAM_END: u16 = 0xFF3F;

pub struct Channel3 {
    sound_on: bool,
    playing: bool,
    sound_len: u8,
    select_output: u8,
    freq_data: u16,
    freq_init: bool,
    freq_selection: bool,
    wave_pattern: [u8; 16],
}

impl Channel3 {
    pub fn new() -> Self {
        Self {
            sound_on: false,
            playing: false,
            sound_len: 0,
            select_output: 0,
            freq_data: 0,
            freq_init: false,
            freq_selection: false,
            wave_pattern: [0; 16],
        }
    }

    pub fn is_sound_on(&self) -> bool {
        self.sound_on
    }

    pub fn read(&self, addr: u16) -> u8 {
        match addr {
            NR30 => {
                (self.playing as u8) << 7
            },
            NR31 => {
                self.sound_len
            },
            NR32 => {
                self.select_output << 5
            },
            NR33 => {
                self.freq_data.get_low_byte()
            },
            NR34 => {
                (self.freq_selection as u8) << 6
            },
            WAVE_RAM_START..=WAVE_RAM_END => {
                let offset = (addr - WAVE_RAM_START) as usize;
                self.wave_pattern[offset]
            },
            _ => unreachable!("Address: {:#04x}", addr)
        }
    }

    pub fn write(&mut self, addr: u16, data: u8) {
        match addr {
            NR30 => {
                self.playing = data.get_bit(7);
            },
            NR31 => {
                self.sound_len = data;
            },
            NR32 => {
                self.select_output = (data & 0x60) >> 5;
            },
            NR33 => {
                self.freq_data.set_low_byte(data);
            },
            NR34 => {
                self.freq_init = data.get_bit(7);
                self.freq_selection = data.get_bit(6);
                self.freq_data.set_high_byte(data & 0x7);
            },
            WAVE_RAM_START..=WAVE_RAM_END => {
                let offset = (addr - WAVE_RAM_START) as usize;
                self.wave_pattern[offset] = data;
            },
            _ => unreachable!("Address: {:#04x} Value: {:#02x}", addr, data)
        }

    }
}
