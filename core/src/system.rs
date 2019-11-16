use crate::cartridge::ROM;
use crate::cpu::Cpu;
use crate::mmu::RAM;

pub struct Gameboy {
    pub cpu: Cpu,
    pub ram: RAM,
    pub rom: ROM,
}

impl Gameboy {
    pub fn new() -> Gameboy {
        let new_ram = RAM::new();

        Gameboy {
            // Not sure if this is being passed by copy?
            cpu: Cpu::new(new_ram),
            ram: new_ram,
            rom: ROM::new()
        }
    }
}
