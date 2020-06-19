use crate::cartridge::*;
use crate::utils::ModifyBits;

const MBC2_EXT_RAM_STOP: u16 = 0xA1FF;

pub fn mbc2_read_byte(cart: &Cart, addr: u16) -> u8 {
    // MBC2 only uses the lower four bits
    let rel_addr = ((addr - EXT_RAM_START) & 0x0F) as usize;
    // Reading from external RAM
    let ram_bank_addr = (cart.ram_bank as usize) * RAM_BANK_SIZE + rel_addr;
    cart.ram[ram_bank_addr]
}

pub fn mbc2_write_byte(cart: &mut Cart, addr: u16, val: u8) {
    match addr {
        RAM_ENABLE_START..=RAM_ENABLE_STOP => {
            // RAM enable is toggled if bit 5 is 0
            let toggle = val.get_bit(5);
            if toggle {
                cart.ext_ram_enable = !cart.ext_ram_enable;
            }
        },
        ROM_BANK_NUM_START..=ROM_BANK_NUM_STOP => {
            let bank_val = val & 0x0F;
            cart.rom_bank = bank_val as u16;
        },
        EXT_RAM_START..=MBC2_EXT_RAM_STOP => {
            if cart.ext_ram_enable {
                let rel_addr = (addr - EXT_RAM_START) as usize;
                let ram_addr = (cart.ram_bank as usize) * RAM_BANK_SIZE + rel_addr;
                cart.ram[ram_addr] = val;
            }
        }
        _ => {
            panic!("Writing to undefined address!");
        }
    }
}
