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
pub fn mbc3_read_byte(cart: &Cart, addr: u16, bank: u8) -> u8 {
    if cart.rtc.is_enabled() && (0x08 <= bank && bank <= 0x0C) {
        cart.rtc.read_byte(bank as u8)
    } else {
        let rel_addr = (addr - EXT_RAM_START) as usize;
        // Reading from external RAM
        let ram_bank_addr = (bank as usize) * RAM_BANK_SIZE + rel_addr;
        cart.ram[ram_bank_addr]
    }
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
            cart.ram_bank = val;
        },
        ROM_RAM_MODE_START..=ROM_RAM_MODE_STOP => {
            // RTC registers will latch if $00, then $01 is written
            cart.rtc.write_byte(val);
        },
        EXT_RAM_START..=EXT_RAM_STOP => {
            if cart.ext_ram_enable {
                match cart.ram_bank {
                    // RAM banks can go from 0-3
                    0x00..=0x03 => {
                        let rel_addr = (addr - EXT_RAM_START) as usize;
                        let ram_addr = (cart.ram_bank as usize) * RAM_BANK_SIZE + rel_addr;
                        cart.ram[ram_addr] = val;
                        battery_write = true;
                    },
                    0x08..=0x0C => {
                        cart.rtc.write_byte(val);
                    },
                    _ => {
                        // Unknown behavior, do nothing
                    }
                }
            }
        }
        _ => {
            panic!("Address too large for cartridge!");
        }
    }

    battery_write
}
