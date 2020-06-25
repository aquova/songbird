use crate::cartridge::*;

pub fn mbc3_read_byte(cart: &Cart, addr: u16) -> u8 {
    let rel_addr = (addr - EXT_RAM_START) as usize;
    // Reading from external RAM
    let ram_bank_addr = (cart.ram_bank as usize) * RAM_BANK_SIZE + rel_addr;
    cart.ram[ram_bank_addr]
}

pub fn mbc3_write_byte(cart: &mut Cart, addr: u16, val: u8) -> bool {
    let mut battery_write = false;

    match addr {
        RAM_ENABLE_START..=RAM_ENABLE_STOP => {
            // External RAM access enabled if $0A written
            cart.ext_ram_enable = val == 0x0A;
        },
        ROM_BANK_NUM_START..=ROM_BANK_NUM_STOP => {
            // Bank numbers $00 isn't used, instead selecting $01
            if val == 0x00 {
                cart.rom_bank = (val + 1) as u16;
            } else {
                cart.rom_bank = val as u16;
            }
        },
        RAM_BANK_NUM_START..=RAM_BANK_NUM_STOP => {
            match val {
                0x00..=0x03 => {
                    cart.ram_bank = val;
                },
                0x08..=0x0C => {
                    // Map in RTC register
                    // TODO: Set up RTC
                },
                _ => {
                    // Unknown behavior
                }
            }
        },
        ROM_RAM_MODE_START..=ROM_RAM_MODE_STOP => {
            // TODO: Setup RTC registers
        },
        EXT_RAM_START..=EXT_RAM_STOP => {
            // TODO: Setup RTC
            if cart.ext_ram_enable {
                let rel_addr = (addr - EXT_RAM_START) as usize;
                let ram_addr = (cart.ram_bank as usize) * RAM_BANK_SIZE + rel_addr;
                cart.ram[ram_addr] = val;
                battery_write = true;
            }
        }
        _ => {
            panic!("Address too large for cartridge!");
        }
    }

    battery_write
}
