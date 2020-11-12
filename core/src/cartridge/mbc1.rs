use crate::cartridge::*;

/// ```
/// Read byte
///
/// Read byte from cartridge
///
/// Inputs:
///     Cartridge object (&Cart)
///     Memory address (u16)
///     RAM bank (u8)
///
/// Output:
///     Byte read (u8)
/// ```
pub fn mbc1_read_byte(cart: &Cart, addr: u16, bank: u8) -> u8 {
    let rel_addr = (addr - EXT_RAM_START) as usize;
    // Reading from external RAM
    let ram_bank_addr = (bank as usize) * RAM_BANK_SIZE + rel_addr;
    cart.ram[ram_bank_addr]
}

/// ```
/// Write byte
///
/// Write byte to cartridge memory
///
/// Inputs:
///     Cartridge object (&Cart)
///     Memory address (u16)
///     Value to write (u8)
///
/// Output:
///     Whether external RAM is dirty (bool)
/// ```
pub fn mbc1_write_byte(cart: &mut Cart, addr: u16, val: u8) -> bool {
    let mut battery_write = false;

    match addr {
        RAM_ENABLE_START..=RAM_ENABLE_STOP => {
            // External RAM access enabled if $0A written
            cart.ext_ram_enable = val == 0x0A;
        },
        ROM_BANK_NUM_START..=ROM_BANK_NUM_STOP => {
            let bank_val = (val & 0x1F) as u16;

            // Bank numbers $00, $20, $40, or $60 aren't used
            // Instead they load $01, $21, $41, $61 respectively
            match bank_val {
                0x00 | 0x20 | 0x40 | 0x60 => {
                    cart.rom_bank = bank_val + 1;
                },
                _ => {
                    cart.rom_bank = bank_val;
                }
            }
        },
        RAM_BANK_NUM_START..=RAM_BANK_NUM_STOP => {
            let bits = val & 0b11;

            if cart.rom_mode {
                // Set bits 5 & 6 of ROM bank
                cart.rom_bank |= (bits << 4) as u16;
            } else {
                // RAM bank switching
                cart.ram_bank = bits;
            }
        },
        ROM_RAM_MODE_START..=ROM_RAM_MODE_STOP => {
            // ROM banking mode if $00
            // RAM banking mode if $01
            cart.rom_mode = val == 0x00;
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
            panic!("Address too large for cartridge!");
        }
    }

    battery_write
}
