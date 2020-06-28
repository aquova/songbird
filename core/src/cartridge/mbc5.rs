use crate::cartridge::*;
use crate::utils::ModifyBits;

const MBC5_LOW_BITS_ROM_BANK_STOP: u16 = 0x2FFF;
const MBC5_HIGH_BIT_ROM_BANK_START: u16 = MBC5_LOW_BITS_ROM_BANK_STOP + 1;

pub fn mbc5_read_byte(cart: &Cart, addr: u16) -> u8 {
    if cart.ext_ram_enable {
        let rel_addr = (addr - EXT_RAM_START) as usize;
        // Reading from external RAM
        let ram_bank_addr = (cart.ram_bank as usize) * RAM_BANK_SIZE + rel_addr;
        cart.ram[ram_bank_addr]
    } else {
        0
    }
}

pub fn mbc5_write_byte(cart: &mut Cart, addr: u16, val: u8) -> bool {
    let mut battery_write = false;

    match addr {
        RAM_ENABLE_START..=RAM_ENABLE_STOP => {
            // External RAM access enabled if $0A written
            if val == 0x0A {
                cart.ext_ram_enable = true;
            } else if val == 0x00 {
                cart.ext_ram_enable = false;
            }
        },
        ROM_BANK_NUM_START..=MBC5_LOW_BITS_ROM_BANK_STOP => {
            cart.rom_bank &= 0xFF00;
            cart.rom_bank |= val as u16;
        },
        MBC5_HIGH_BIT_ROM_BANK_START..=ROM_BANK_NUM_STOP => {
            let ninth_bit = val != 0;
            cart.rom_bank.write_bit(9, ninth_bit);
        },
        RAM_BANK_NUM_START..=RAM_BANK_NUM_STOP => {
            // RAM bank switching
            cart.ram_bank = val & 0xF;
        },
        ROM_RAM_MODE_START..=ROM_RAM_MODE_STOP => {
            // Do nothing
        },
        EXT_RAM_START..=EXT_RAM_STOP => {
            if cart.ext_ram_enable {
                let rel_addr = (addr - EXT_RAM_START) as usize;
                let ram_addr = (cart.ram_bank as usize) * RAM_BANK_SIZE + rel_addr;
                cart.ram[ram_addr] = val;
                battery_write = true;
            }
        }
        _ => {
            panic!("Invalid RAM access");
        }
    }

    battery_write
}
