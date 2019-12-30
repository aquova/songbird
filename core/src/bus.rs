extern crate sdl2;

use crate::cartridge::{BANK_SIZE, MBC, ROM};
use crate::ppu::PPU;
use crate::utils::ModifyBits;
use sdl2::render::Canvas;
use sdl2::video::Window;

const RAM_SIZE: usize = 0x10000;

// VRAM ranges
const TILE_SET_0_RANGE: std::ops::Range<usize> = 0x8000..0x9000;
const TILE_SET_1_RANGE: std::ops::Range<usize> = 0x8800..0x9800;
const TILE_MAP_0_RANGE: std::ops::Range<usize> = 0x9800..0x9C00;
const TILE_MAP_1_RANGE: std::ops::Range<usize> = 0x9C00..0xA000;
const SAM:              std::ops::Range<usize> = 0xFE00..0xFEA0;

// VRAM regsiters
const LCD_DISP_REG: usize                       = 0xFF40;
const LCD_STAT_REG: usize                       = 0xFF41;

/*
 * RAM Map
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
 */

pub struct Bus {
    ram: [u8; RAM_SIZE],
    ram_enabled: bool,
    rom: ROM,
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
            rom: ROM::new(),
            mbc: MBC::NONE,
            ppu: PPU::new(1)
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
    pub fn load_game(&mut self, path: &str) {
        self.rom.load_cart(path);
        self.load_bank_0();
        self.mbc = self.rom.get_mbc();
        // If no MBC, then load the rest of ROM into RAM
        if self.mbc == MBC::NONE {
            self.load_bank_n(1);
        }
    }

    pub fn draw(&self, canvas: &mut Canvas<Window>) {
        // draw_screen(&self.ram, canvas, scale);
        self.ppu.draw_tile_set(&self.ram, canvas);
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
        self.ram[addr as usize]
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
    }

    /// ```
    /// Get RAM
    ///
    /// Returns the entire RAM array. Used for testing.
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

    pub fn get_tile_set(&self) -> &[u8] {
        let lcd_reg = self.ram[LCD_DISP_REG];

        let tile_set = if self.get_bkgd_tile_set(lcd_reg) == 0 {
            self.get_tile_set_0()
        } else {
            self.get_tile_set_1()
        };

        tile_set
    }

    pub fn get_tile_map(&self) -> &[u8] {
        let lcd_reg = self.ram[LCD_DISP_REG];
        let tile_map = if self.get_bkgd_tile_map(lcd_reg) == 0 {
            self.get_tile_map_0()
        } else {
            self.get_tile_map_1()
        };

        tile_map
    }

    pub fn get_sprite_attributes(&self) -> &[u8] {
        &self.ram[SAM]
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

    fn write_mbc2(&mut self, addr: u16, val: u8) {

    }

    fn write_mbc3(&mut self, addr: u16, val: u8) {

    }

    fn get_bkgd_tile_set(&self, reg: u8) -> u8 {
        if reg.get_bit(4) { return 1 } else { return 0 }
    }

    fn get_bkgd_tile_map(&self, reg: u8) -> u8 {
        if reg.get_bit(3) { return 1 } else { return 0 }
    }

    fn get_tile_set_0(&self) -> &[u8] {
        &self.ram[TILE_SET_0_RANGE]
    }

    fn get_tile_set_1(&self) -> &[u8] {
        &self.ram[TILE_SET_1_RANGE]
    }

    fn get_tile_map_0(&self) -> &[u8] {
        &self.ram[TILE_MAP_0_RANGE]
    }

    fn get_tile_map_1(&self) -> &[u8] {
        &self.ram[TILE_MAP_1_RANGE]
    }
}
