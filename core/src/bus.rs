use crate::cartridge::ROM;
use crate::mmu::RAM;

pub struct Bus {
    ram: RAM,
    rom: Box<ROM>
}

impl Bus {
    pub fn new() -> Bus {
        let rom = ROM::new();

        Bus {
            ram: RAM::new(),
            rom: Box::new(rom)
        }
    }

    pub fn read_ram(self, addr: u16) -> u8 {
        self.ram.read_byte(addr)
    }

    pub fn write_ram(&mut self, addr: u16, val: u8) {
        self.ram.write_byte(addr, val);
    }
}
