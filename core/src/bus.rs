use crate::cartridge::{BANK_SIZE, MBC, Cart};
use crate::io::{Buttons, IO};
use crate::ppu::PPU;
use crate::utils::DISP_SIZE;
use std::ops::{Range, RangeInclusive};

/*
 * RAM Map
 * Not drawn to scale
 *
 * +----Cartridge-ROM-----+ $0000
 * |                      |
 * |                      |
 * |        Bank 0        |
 * |                      |
 * |                      |
 * +----------------------+ $4000
 * |                      |
 * |                      |
 * |        Bank N        |
 * |                      |
 * |                      |
 * +----Internal-RAM------+ $8000
 * |                      |
 * |      Video RAM       |
 * |                      |
 * +----Cartridge-RAM-----+ $A000
 * |                      |
 * |    Switchable RAM    |
 * |                      |
 * +----Internal-RAM------+ $C000
 * |                      |
 * +----------------------+ $E000
 * | Echo of Internal RAM |
 * +----------------------+ $FE00
 * | Sprite Attribute RAM |
 * +-----Special-I/O------+ $FEA0
 * |        Empty         |
 * +----------------------+ $FF00
 * |  Special (I/O Ports) |
 * +----------------------+ $FF4C
 * |        Empty         |
 * +----------------------+ $FF80
 * |     Internal RAM     |
 * +----------------------+ $FFFE
 * | Interrupt Enable Reg |
 * +----------------------+ $FFFF
 *
**/

// =============
// = Constants =
// =============
const RAM_SIZE: usize = 0x8000;
const VRAM_START: u16 = RAM_SIZE as u16;
const RAM_END: u16 = 0xFFFF;

const JOYPAD_REG: u16 = 0xFF00;

// RAM ranges
const CART_ROM_RANGE: Range<u16> = 0x0000..VRAM_START;
const VRAM_RANGE: RangeInclusive<u16> = VRAM_START..=RAM_END;

pub struct Bus {
    ram: [u8; RAM_SIZE],
    ram_enabled: bool,
    rom: Cart,
    io: IO,
    mbc: MBC,
    ppu: PPU
}

// ==================
// = Public methods =
// ==================
impl Bus {
    pub fn new() -> Bus {
        Bus {
            ram: [0; RAM_SIZE],
            ram_enabled: false,
            rom: Cart::new(),
            io: IO::new(),
            mbc: MBC::NONE,
            ppu: PPU::new()
        }
    }

    /// ```
    /// Load game
    ///
    /// Loads game into ROM
    ///
    /// Input:
    ///     Path to game (&str)
    /// ```
    pub fn load_game(&mut self, rom: Vec<u8>) {
        self.rom.load_cart(&rom);
        self.load_bank_0();
        self.mbc = self.rom.get_mbc();
        // If no MBC, then load the rest of ROM into RAM
        if self.mbc == MBC::NONE {
            self.load_bank_n(1);
        }
    }

    /// ```
    /// Render
    ///
    /// Renders the screen
    ///
    /// Output:
    ///     Array of pixels to draw ([u8])
    /// ```
    pub fn render(&self) -> [u8; DISP_SIZE] {
        self.ppu.render_screen()
    }

    /// ```
    /// Get palette
    ///
    /// Gets the currently used palette
    ///
    /// Output:
    ///     Palette indices ([u8])
    /// ```
    pub fn get_palette(&self) -> [u8; 4] {
        self.ppu.get_palette()
    }

    /// ```
    /// Read RAM
    ///
    /// Reads value from RAM
    ///
    /// Input:
    ///     RAM address (u16)
    ///
    /// Output:
    ///     Value at address (u8)
    /// ```
    pub fn read_ram(&self, addr: u16) -> u8 {
        if CART_ROM_RANGE.contains(&addr) {
            self.ram[addr as usize]
        } else if VRAM_RANGE.contains(&addr) {
            if addr == JOYPAD_REG {
                self.io.read_btns()
            } else {
                self.ppu.read_vram(addr)
            }
        } else {
            panic!("Unimplemented!");
        }
    }

    /// ```
    /// Write RAM
    ///
    /// Writes value to RAM
    ///
    /// Input:
    ///     RAM address (u16)
    ///     Value to write (u8)
    /// ```
    pub fn write_ram(&mut self, addr: u16, val: u8) {
        if CART_ROM_RANGE.contains(&addr) {
            match self.mbc {
                MBC::NONE => {
                    self.ram[addr as usize] = val;
                },
                MBC::MBC1 => {
                    self.write_mbc1(addr, val);
                },
                MBC::MBC2 => {
                    self.write_mbc2(addr, val);
                },
                MBC::MBC3 => {
                    self.write_mbc3(addr, val);
                }
            }
        } else if VRAM_RANGE.contains(&addr) {
            self.ppu.write_vram(addr, val);
            if addr == JOYPAD_REG {
                self.io.set_btns(val);
            }
        } else {
            panic!("Unimplemented!");
        }
    }

    pub fn toggle_button(&mut self, btn: Buttons, pressed: bool) {
        self.io.btn_toggle(btn, pressed);
    }

    /// ```
    /// Get RAM
    ///
    /// Returns the entire RAM array. Used for testing.
    /// TODO: Delete this
    ///
    /// Output:
    ///     RAM array ([u8])
    /// ```
    pub fn get_ram(&self) -> [u8; RAM_SIZE] {
        self.ram
    }

    /// ```
    /// Get Title
    ///
    /// Gets the title of the game
    ///
    /// Output:
    ///     Game title from ROM (&str)
    /// ```
    pub fn get_title(&self) -> &str {
        self.rom.get_title()
    }

    /// ```
    /// Get MBC
    ///
    /// Returns the MBC type for the game
    ///
    /// Output:
    ///     MBC type from ROM (MBC)
    /// ```
    pub fn get_mbc(&self) -> MBC {
        self.mbc
    }

    /// ```
    /// Set scanline
    ///
    /// Sets the current scanline value into the LY RAM address
    ///
    /// Input:
    ///     Line number (u8)
    /// ```
    pub fn set_scanline(&mut self, line: u8) {
        self.ppu.set_ly(line);
    }

    /// ```
    /// Set status register
    ///
    /// Sets the status register to match current screen mode
    ///
    /// Input:
    ///     Clock mode (u8)
    /// ```
    pub fn set_status_reg(&mut self, mode: u8) {
        let mode = mode & 0b0000_0011;
        self.ppu.set_status(mode);
    }
}

// ===================
// = Private methods =
// ===================
impl Bus {
    /// ```
    /// Load Bank 0
    ///
    /// Loads ROM bank 0 into RAM
    /// ```
    fn load_bank_0(&mut self) {
        let bank0 = self.rom.get_bank_0();
        &self.ram[0..BANK_SIZE].copy_from_slice(&bank0);
    }

    /// ```
    /// Bank Switch
    ///
    /// Switches appropriate ROM bank into RAM
    ///
    /// Input:
    ///     num (u8): The bank number to load into RAM at $4000-$7FFF
    /// ```
    fn load_bank_n(&mut self, num: u8) {
        let bank = self.rom.get_bank_n(num);
        &self.ram[0x4000..=0x7FFF].copy_from_slice(&bank);
    }

    /// ```
    /// Write MBC1
    ///
    /// Writes value to the specified RAM address, given MBC1 chip
    ///
    /// Inputs:
    ///     Address to write to (u16)
    ///     Value to write to the address (u8)
    /// ```
    fn write_mbc1(&mut self, addr: u16, val: u8) {
        match addr {
            0x0000..=0x1FFF => {
                // Enable RAM if $0A written, else disable
                // TODO: Does this write to $FFFF?
                if val == 0x0A {
                    self.ram_enabled = true;
                } else {
                    self.ram_enabled = false;
                }
            },
            0x2000..=0x3FFF => {
                let mut bank_n = val & 0x1F;
                if bank_n == 0 {
                    bank_n += 1;
                }
                self.load_bank_n(bank_n);
            },
            0x8000..=0xFFFF => {
                // TODO: This should not be all writable
                if self.ram_enabled {
                    self.ram[addr as usize] = val;
                }
            }
            _ => {
                // Do nothing
            }
        }
    }

    /// ```
    /// Write MBC2
    ///
    /// Writes value to the specified RAM address, given MBC2 chip
    ///
    /// Inputs:
    ///     Address to write to (u16)
    ///     Value to write to the address (u8)
    /// ```
    fn write_mbc2(&mut self, addr: u16, val: u8) {

    }

    /// ```
    /// Write MBC3
    ///
    /// Writes value to the specified RAM address, given MBC3 chip
    ///
    /// Inputs:
    ///     Address to write to (u16)
    ///     Value to write to the address (u8)
    /// ```
    fn write_mbc3(&mut self, addr: u16, val: u8) {

    }
}
