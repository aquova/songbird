use crate::cartridge::{Cart, ROM_START, ROM_STOP, EXT_RAM_START, EXT_RAM_STOP};
use crate::io::{Buttons, IO};
use crate::ppu::{PPU, LY};
use crate::ppu::palette::Palettes;
use crate::utils::*;
use crate::wram::{WRAM, WRAM_START, WRAM_END, SVBK_REG, ECHO_START, ECHO_END};

use std::cmp::min;

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
 * |      High RAM        |
 * +----------------------+ $FFFE
 * | Interrupt Enable Reg |
 * +----------------------+ $FFFF
 *
**/

// =============
// = Constants =
// =============
const JOYPAD_REG: u16       = 0xFF00;
const DMA_REG: u16          = 0xFF46;
const HDMA1_REG: u16        = 0xFF51;
const HDMA2_REG: u16        = 0xFF52;
const HDMA3_REG: u16        = 0xFF53;
const HDMA4_REG: u16        = 0xFF54;
const HDMA5_REG: u16        = 0xFF55;

const OAM: u16 = 0xFE00;
const HRAM_START: u16 = 0xFF80;
const HRAM_END: u16 = 0xFFFF; // Include $FFFF as part of HRAM
const HRAM_SIZE: usize = (HRAM_END - HRAM_START + 1) as usize;
const VRAM_DMA_PER_HBLANK: u16 = 0x10;

pub struct Bus {
    rom: Cart,
    io: IO,
    ppu: PPU,
    wram: WRAM,
    hram: [u8; HRAM_SIZE],
    vram_dma_remaining: Option<VRAM_DMA>,
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone)]
struct VRAM_DMA {
    pub src_addr: u16,
    pub dst_addr: u16,
    pub len: u16,
    pub transferred: u16,
    pub last_scanline: u8,
    pub active: bool,
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
            ppu: PPU::new(),
            wram: WRAM::new(),
            hram: [0; HRAM_SIZE],
            vram_dma_remaining: None,
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
            WRAM_START..=WRAM_END => {
                self.wram.read_wram(addr)
            },
            ECHO_START..=ECHO_END => {
                self.wram.read_echo(addr)
            },
            JOYPAD_REG => {
                self.io.read_btns()
            },
            SVBK_REG => {
                self.wram.get_wram_bank()
            },
            HDMA5_REG => {
                match self.vram_dma_remaining {
                    Some(dma_data) => {
                        let remaining = dma_data.len - dma_data.transferred;
                        let raw_remaining = (remaining / 0x10 - 1) as u8;
                        if dma_data.active {
                            raw_remaining
                        } else {
                            0x80 | raw_remaining
                        }
                    },
                    None => {
                        0xFF
                    }
                }
            },
            HRAM_START..=HRAM_END => {
                let hram_index = addr - HRAM_START;
                self.hram[hram_index as usize]
            },
            _ => { // $8000-$9FFF, $FE00-$FE9F, $FF00-$FF7F
                self.ppu.read_vram(addr, mode)
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
        let mut battery_write = false;
        match addr {
            ROM_START..=ROM_STOP | EXT_RAM_START..=EXT_RAM_STOP => {
                self.rom.write_cart(addr, val);
                battery_write = true;
            },
            WRAM_START..=WRAM_END => {
                self.wram.write_wram(addr, val);
            },
            HRAM_START..=HRAM_END => {
                let hram_addr = addr - HRAM_START;
                self.hram[hram_addr as usize] = val;
            },
            JOYPAD_REG => {
                self.io.poll_btns(val);
            },
            DMA_REG => {
                self.oam_dma(val, mode);
            },
            HDMA5_REG => {
                if mode == GB::CGB {
                    self.vram_dma(Some(val));
                } else {
                    self.ppu.write_vram(addr, val, mode);
                }
            },
            SVBK_REG => {
                self.wram.set_wram_bank(val, mode);
            },
            _ => { // $8000-$9FFF, $FE00-$FE9F, $FF00-$FF7F
                self.ppu.write_vram(addr, val, mode);
            }
        }

        battery_write
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
    ///     Game Boy mode (GB)
    /// ```
    pub fn set_status_reg(&mut self, clock_mode: u8, gb_mode: GB) {
        let clock_mode = clock_mode & 0b0000_0011;
        // TODO: The clock_mode comparision should be a const or inherited from Clock or something
        // If in HBLANK:
        if clock_mode == 0 && gb_mode == GB::CGB {
            self.vram_dma(None);
        }
        self.ppu.set_status(clock_mode);
    }

    /// ```
    /// Set system palette
    ///
    /// Set which palette the DMG should use
    ///
    /// Input:
    ///     Which palette to use (Palettes)
    /// ```
    pub fn set_sys_pal(&mut self, pal: Palettes) {
        self.ppu.set_sys_pal(pal);
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

    /// ```
    /// VRAM DMA transfer
    ///
    /// Copies sprite data from stored area into VRAM, initiated when data written to HDMA5
    ///
    /// Input:
    ///     Length in bytes to transfer, if any (Option<u8>)
    /// ```
    fn vram_dma(&mut self, val: Option<u8>) {
        // Game Boy Color only, source and destination areas are encoded as such:
        // $FF51 - DMA Source, High
        // $FF52 - DMA Source, Low
        // $FF53 - DMA Destination, High
        // $FF54 - DMA Destination, Low
        // $FF55 - DMA Length
        match &self.vram_dma_remaining {
            Some(mut dma_data) => {
                if dma_data.active {
                    if let Some(pause_data) = val {
                        // If newly written data to HDMA5 has 7th bit clear, halt transfer
                        // This effectively ends the transfer, but metadata needs to be available to be read
                        if !pause_data.get_bit(7) {
                            dma_data.active = false;
                            return;
                        }
                    }

                    // Complete data transfer of at most $16 bytes, noting if we're done
                    let scanline = self.read_ram(LY, GB::CGB);
                    if scanline != dma_data.last_scanline {
                        let remaining = min(VRAM_DMA_PER_HBLANK, dma_data.len - dma_data.transferred);
                        for i in 0..remaining {
                            let byte = self.read_ram(dma_data.src_addr + dma_data.transferred + i, GB::CGB);
                            self.write_ram(dma_data.dst_addr + dma_data.transferred + i, byte, GB::CGB);
                        }
                        dma_data.transferred += remaining;
                        if dma_data.transferred == dma_data.len {
                            self.vram_dma_remaining = None;
                        } else {
                            dma_data.last_scanline = scanline;
                        }
                    }
                } else if let Some(raw_transfer_len) = val {
                    // If the previous transfer is no longer active, but new data has come in, start a new transfer
                    self.vram_dma_helper(raw_transfer_len);
                }
            },
            None => {
                // Only initiate a new VRAM DMA transfer if a value was written to HDMA5
                if let Some(raw_transfer_len) = val {
                    self.vram_dma_helper(raw_transfer_len);
                }
            }
        }
    }

    /// ```
    /// VRAM DMA helper
    ///
    /// Performs a complete VRAM DMA transfer of the entire specified data amount
    ///
    /// Input:
    ///     Amount of bytes to transfer (u8)
    /// ```
    fn vram_dma_helper(&mut self, raw_transfer_len: u8) {
        let src_addr_high = self.read_ram(HDMA1_REG, GB::CGB);
        let src_addr_low = self.read_ram(HDMA2_REG, GB::CGB);
        let dst_addr_high = self.read_ram(HDMA3_REG, GB::CGB);
        let dst_addr_low = self.read_ram(HDMA4_REG, GB::CGB);

        let src_addr = merge_bytes(src_addr_high, src_addr_low) & 0xFFF0; // Lower 4 bits are always zero
        let dst_addr = merge_bytes(dst_addr_high, dst_addr_low) & 0x1FF0; // Lower 4 bits are ignored, as well as highest 3, as dest is always in VRAM

        // Transfer length is (lower 7 bits of HDMA5 value) * $10 + 1
        let transfer_len = (((raw_transfer_len as u16) & 0b0111_1111) + 1) * 0x10;
        let hblank_transfer = raw_transfer_len.get_bit(7);

        if hblank_transfer {
            // If 7th bit was set, then we transfer $10 bits at a time during each HBLANK scanline
            for i in 0..VRAM_DMA_PER_HBLANK {
                let byte = self.read_ram(src_addr + i, GB::CGB);
                self.write_ram(dst_addr + i, byte, GB::CGB);
            }

            self.vram_dma_remaining = Some(
                VRAM_DMA {
                    src_addr: src_addr,
                    dst_addr: dst_addr,
                    len: transfer_len,
                    transferred: VRAM_DMA_PER_HBLANK,
                    last_scanline: self.read_ram(LY, GB::CGB),
                    active: true,
                }
            );
        } else {
            // Otherwise, simply transfer all data at once
            for i in 0..transfer_len {
                let byte = self.read_ram(src_addr + i, GB::CGB);
                self.write_ram(dst_addr + i, byte, GB::CGB);
            }
        }
    }
}
