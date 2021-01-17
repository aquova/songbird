// Constants
const HBLANK_LEN: usize = 204;
const VBLANK_LEN: usize = 456;
const OAM_READ_LEN: usize = 80;
const VRAM_READ_LEN: usize = 172;

const VBLANK_LINE_START: u8 = 143;
const VBLANK_LINE_END: u8 = VBLANK_LINE_START + 10;

#[derive(PartialEq)]
pub enum LcdResults {
    NoAction,
    RenderScanline,
    RenderFrame
}

#[derive(PartialEq, Clone, Copy)]
pub enum LcdModeType {
    HBLANK,
    VBLANK,
    OAMReadMode,
    VRAMReadMode
}

impl LcdModeType {
    pub fn get_idx(&self) -> u8 {
        match *self {
            LcdModeType::HBLANK =>        0,
            LcdModeType::VBLANK =>        1,
            LcdModeType::OAMReadMode =>   2,
            LcdModeType::VRAMReadMode =>  3,
        }
    }
}

pub struct Lcd {
    cycles: usize,
    line: u8,
    mode: LcdModeType
}

impl Default for Lcd {
    fn default() -> Self {
        Self::new()
    }
}

impl Lcd {
    pub fn new() -> Lcd {
        Lcd {
            cycles: 0,
            line: 0,
            mode: LcdModeType::HBLANK
        }
    }

    /// ```
    /// Lcd Step
    ///
    /// Adds specified number of cycles to the LCD counter, updating scanline and blank timings
    ///
    /// Input:
    ///     Number of cycles of most recent instruction (u8)
    ///
    /// Output:
    ///     Action to take following this lcd cycle (LcdResults)
    /// ```
    pub fn lcd_step(&mut self, cycles: u8) -> LcdResults {
        self.cycles += cycles as usize;
        let mut result = LcdResults::NoAction;

        match self.mode {
            // Screen gets drawn after final hblank
            LcdModeType::HBLANK => {
                if self.cycles >= HBLANK_LEN {
                    self.cycles = 0;
                    self.line += 1;

                    if self.line == VBLANK_LINE_START {
                        self.mode = LcdModeType::VBLANK;
                        // VBLANK is starting, time to draw screen
                        // The VBLANK interrupt is triggered here
                        result = LcdResults::RenderFrame;
                    } else {
                        self.mode = LcdModeType::OAMReadMode;
                    }
                }
            },
            // VBLANK lasts for 10 lines
            LcdModeType::VBLANK => {
                if self.cycles >= VBLANK_LEN {
                    self.cycles = 0;
                    self.line += 1;

                    if self.line > VBLANK_LINE_END {
                        self.mode = LcdModeType::OAMReadMode;
                        self.line = 0;
                    }
                }
            },
            LcdModeType::OAMReadMode => {
                if self.cycles >= OAM_READ_LEN {
                    self.cycles = 0;
                    self.mode = LcdModeType::VRAMReadMode;
                }
            },
            LcdModeType::VRAMReadMode => {
                if self.cycles >= VRAM_READ_LEN {
                    self.cycles = 0;
                    self.mode = LcdModeType::HBLANK;
                    // Render current scanline here
                    result = LcdResults::RenderScanline;
                }
            }
        }

        result
    }

    /// ```
    /// Get scanline
    ///
    /// Returns which scanline is currently being drawn
    ///
    /// Output:
    ///     Scanline Y-value (u8)
    /// ```
    pub fn get_scanline(&self) -> u8 {
        self.line
    }

    /// ```
    /// Get mode
    ///
    /// Returns the current lcd mode, as an int
    ///
    /// Output:
    ///     Current mode (LcdModeType)
    /// ```
    pub fn get_mode(&self) -> LcdModeType {
        self.mode
    }

    /// ```
    /// Reset Line
    ///
    /// Resets scanline counter
    /// Occurs if $FF44 is written to
    /// ```
    pub fn reset_line(&mut self) {
        self.line = 0;
    }

    /// ```
    /// Is VBLANK iterrupt
    ///
    /// Returns if currently in VBLANK
    ///
    /// Output:
    ///     True if currently in VBLANK (bool)
    /// ```
    pub fn is_vblank_interrupt(&self) -> bool {
        self.mode == LcdModeType::VBLANK
    }
}
