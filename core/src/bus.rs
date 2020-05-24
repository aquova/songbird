use crate::cartridge::{BANK_SIZE, Cart};
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
 * |   Work RAM Bank 0    |
 * +----------------------+ $D000
 * |   Work RAM Bank 1    |
 * +--------ECHO----------+ $E000
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
const RAM_ROM_SIZE: usize = 0x8000;
const VRAM_START: u16 = RAM_ROM_SIZE as u16;
const RAM_END: u16 = 0xFFFF;

const JOYPAD_REG: u16 = 0xFF00;

// RAM ranges
const CART_ROM_RANGE: Range<u16> = 0x0000..VRAM_START;
const VRAM_RANGE: RangeInclusive<u16> = VRAM_START..=RAM_END;

pub struct Bus {
    ram_enabled: bool,
    rom: Cart,
    io: IO,
    ppu: PPU
}

// ==================
// = Public methods =
// ==================
impl Bus {
    pub fn new() -> Bus {
        Bus {
            ram_enabled: false,
            rom: Cart::new(),
            io: IO::new(),
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
            self.rom.read_rom(addr)
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
            self.rom.write_rom(addr, val);
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
    /// Get Title
    ///
    /// Gets the title of the game
    ///
    /// Output:
    ///     Game title from ROM (&str)
    /// ```
    pub fn get_title(&self) -> &str {
        // Strip trailing null characters, if any
        let raw_title = self.rom.get_title();
        let title = raw_title.trim_end_matches(char::from(0));
        title
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
