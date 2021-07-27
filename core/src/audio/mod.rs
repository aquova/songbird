mod ch1;
mod ch2;
mod ch3;
mod ch4;

use ch1::*;
use ch2::*;
use ch3::*;
use ch4::*;

use crate::utils::ModifyBits;

const NR50: u16 = 0xFF24;
const NR51: u16 = 0xFF25;
const NR52: u16 = 0xFF26;

pub const AUDIO_REGS_START: u16 = NR10;
pub const AUDIO_REGS_END: u16 = WAVE_RAM_END;

pub struct APU {
    tone_sweep_ch: Channel1,
    tone_ch: Channel2,
    wave_ch: Channel3,
    noise_ch: Channel4,
    vin_to_s02: bool,
    so2_vol: u8,
    vin_to_s01: bool,
    so1_vol: u8,
    output_selection: u8,
    master_sound_enable: bool,
}

impl APU {
    pub fn new() -> Self {
        Self {
            tone_sweep_ch: Channel1::new(),
            tone_ch: Channel2::new(),
            wave_ch: Channel3::new(),
            noise_ch: Channel4::new(),
            vin_to_s02: false,
            so2_vol: 0,
            vin_to_s01: false,
            so1_vol: 0,
            output_selection: 0,
            master_sound_enable: false,
        }
    }

    pub fn read(&self, addr: u16) -> u8 {
        match addr {
            NR10..=NR14 => self.tone_sweep_ch.read(addr),
            NR21..=NR24 => self.tone_ch.read(addr),
            NR30..=NR34 | WAVE_RAM_START..=WAVE_RAM_END => self.wave_ch.read(addr),
            NR41..=NR44 => self.noise_ch.read(addr),
            NR50 => {
                (self.vin_to_s02 as u8) << 7 |
                self.so2_vol << 4 |
                (self.vin_to_s01 as u8) << 3 |
                self.so1_vol
            },
            NR51 => {
                self.output_selection
            },
            NR52 => {
                (self.master_sound_enable as u8) << 7 |
                (self.tone_sweep_ch.is_sound_on() as u8) << 3 |
                (self.tone_ch.is_sound_on() as u8) << 2 |
                (self.wave_ch.is_sound_on() as u8) << 1 |
                self.noise_ch.is_sound_on() as u8
            },
            _ => unreachable!("Address: {:#04x}", addr)
        }
    }

    pub fn write(&mut self, addr: u16, data: u8) {
        match addr {
            NR10..=NR14 => self.tone_sweep_ch.write(addr, data),
            NR21..=NR24 => self.tone_ch.write(addr, data),
            NR30..=NR34 | WAVE_RAM_START..=WAVE_RAM_END => self.wave_ch.write(addr, data),
            NR41..=NR44 => self.noise_ch.write(addr, data),
            NR50 => {
                self.vin_to_s02 = data.get_bit(7);
                self.so2_vol = (data & 0x70) >> 4;
                self.vin_to_s01 = data.get_bit(3);
                self.so1_vol = data & 0x7;
            },
            NR51 => {
                self.output_selection = data;
            },
            NR52 => {
                self.master_sound_enable = data.get_bit(7);
            }
            _ => unreachable!("Address: {:#04x} Value: {:#02x}", addr, data)
        }
    }
}
