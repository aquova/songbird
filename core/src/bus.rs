use crate::cartridge::{Cart, ROM_START, ROM_STOP, EXT_RAM_START, EXT_RAM_STOP};
use crate::io::{Buttons, IO};
use crate::ppu::PPU;
use crate::utils::{BYTE, DISP_SIZE, GB};

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
const JOYPAD_REG: u16 = 0xFF00;
const DMA_REG: u16 = 0xFF46;
const OAM: u16 = 0xFE00;

// RAM ranges
// NOTE: Rust *still* doesn't allow exclusive ranges in match statements
// So we have to define both start and end values
const VRAM_START: u16           = ROM_STOP + 1;
const VRAM_STOP: u16            = 0x9FFF;
const WORK_RAM_START: u16       = EXT_RAM_STOP + 1;
// const WORK_RAM_END: u16 = 0xDFFF;
const RAM_END: u16              = 0xFFFF;

pub struct Bus {
    rom: Cart,
    io: IO,
    ppu: PPU
}

// ==================
// = Public methods =
// ==================
impl Default for Bus {
    fn default() -> Self {
        Self::new()
    }
}

impl Bus {
    pub fn new() -> Bus {
        Bus {
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
    ///     ROM data (&[u8])
    ///
    /// Output:
    ///     Which type of system this cart requires
    /// ```
    pub fn load_game(&mut self, rom: &[u8]) -> GB {
        self.rom.load_cart(&rom)
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
    /// Read RAM
    ///
    /// Reads value from RAM
    ///
    /// Input:
    ///     RAM address (u16)
    ///     System mode (GB)
    ///
    /// Output:
    ///     Value at address (u8)
    /// ```
    pub fn read_ram(&self, addr: u16, mode: GB) -> u8 {
        match addr {
            ROM_START..=ROM_STOP | EXT_RAM_START..=EXT_RAM_STOP => {
                self.rom.read_cart(addr)
            },
            VRAM_START..=VRAM_STOP | WORK_RAM_START..=RAM_END => {
                if addr == JOYPAD_REG {
                    self.io.read_btns()
                } else {
                    self.ppu.read_vram(addr, mode)
                }
            }
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
    ///     System mode (GB)
    ///
    /// Output:
    ///     Whether data was written to battery-saved RAM
    /// ```
    pub fn write_ram(&mut self, addr: u16, val: u8, mode: GB) -> bool {
        match addr {
            ROM_START..=ROM_STOP | EXT_RAM_START..=EXT_RAM_STOP => {
                self.rom.write_cart(addr, val)
            },
            VRAM_START..=VRAM_STOP | WORK_RAM_START..=RAM_END => {
                match addr {
                    JOYPAD_REG => {
                        self.io.poll_btns(val);
                    },
                    DMA_REG => {
                        self.oam_dma(val, mode);
                    },
                    _ => {
                        self.ppu.write_vram(addr, val, mode);
                    }
                }
                false
            }
        }
    }

    /// ```
    /// Toggle button
    ///
    /// Sends a message to I/O class that button has been pressed/released
    ///
    /// Inputs:
    ///     Button that was toggled (Buttons)
    ///     If the button was pressed (versus released) (bool)
    /// ```
    pub fn toggle_button(&mut self, btn: Buttons, pressed: bool) {
        self.io.btn_toggle(btn, pressed);
    }

    /// ```
    /// Get external RAM
    ///
    /// Returns a slice to the external RAM object, used for battery saving
    ///
    /// Output:
    ///     External RAM, as a slice (&[u8])
    /// ```
    pub fn get_ext_ram(&self) -> &[u8] {
        self.rom.get_ext_ram()
    }

    /// ```
    /// Has battery
    ///
    /// Returns whether game has an external battery
    ///
    /// Output:
    ///     Whether cartridge has a battery (bool)
    /// ```
    pub fn has_battery(&self) -> bool {
        self.rom.has_battery()
    }

    /// ```
    /// Get Title
    ///
    /// Gets the title of the game
    ///
    /// Output:
    ///     Game title from ROM (&str)
    /// ```
    pub fn get_title(&self, is_cgb: bool) -> &str {
        // Strip trailing null characters, if any
        let raw_title = self.rom.get_title(is_cgb);
        raw_title.trim_end_matches(char::from(0))
    }

    /// ```
    /// Set scanline
    ///
    /// Sets the current scanline value into the LY RAM address
    ///
    /// Input:
    ///     Line number (u8)
    ///
    /// Output:
    ///     Whether to trigger LCDC interrupt
    /// ```
    pub fn set_scanline(&mut self, line: u8) -> bool {
        self.ppu.set_ly(line)
    }

    /// ```
    /// Render scanline
    ///
    /// Renders the current scanline into the screen buffer
    ///
    /// Input:
    ///     GB hardware type
    /// ```
    pub fn render_scanline(&mut self, mode: GB) {
        self.ppu.render_scanline(mode);
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

    /// ```
    /// Write external RAM
    ///
    /// Writes data to the external RAM memory, for battery saves
    ///
    /// Input:
    ///     Raw RAM data: (&[u8])
    /// ```
    pub fn write_ext_ram(&mut self, data: &[u8]) {
        self.rom.write_ext_ram(data);
    }
}

// Private functions
impl Bus {
    /// ```
    /// OAM DMA transfer
    ///
    /// Copies array of memory from specified area to OAM memory
    ///
    /// Input:
    ///     Upper byte of source memory location (u8)
    ///     System mode (GB)
    /// ```
    fn oam_dma(&mut self, val: u8, mode: GB) {
        // If value is $XX, then copy $XX00-$XX9F into OAM RAM
        let source_addr = (val as u16).wrapping_shl(BYTE as u32);
        let dest_addr = OAM;

        for i in 0..0xA0 {
            let byte = self.read_ram(source_addr + i, mode);
            self.write_ram(dest_addr + i, byte, mode);
        }
    }
}
