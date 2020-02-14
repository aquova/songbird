// Constants
const HBLANK_LEN: usize = 204;
const VBLANK_LEN: usize = 456;
const OAM_READ_LEN: usize = 80;
const VRAM_READ_LEN: usize = 172;

const VBLANK_LINE_START: u8 = 143;
const VBLANK_LINE_END: u8 = VBLANK_LINE_START + 10;

#[derive(PartialEq)]
enum ModeTypes {
    HBLANK,
    VBLANK,
    OAMReadMode,
    VRAMReadMode
}

pub struct Clock {
    cycles: usize,
    line: u8,
    mode: ModeTypes
}

impl Clock {
    pub fn new() -> Clock {
        Clock {
            cycles: 0,
            line: 0,
            mode: ModeTypes::HBLANK
        }
    }

    /// ```
    /// Clock Step
    ///
    /// Adds specified number of cycles to the clock, updating scanline and blank timings
    ///
    /// Input:
    ///     Number of cycles of most recent instruction (u8)
    ///
    /// Output:
    ///     Whether or not to render the screen (bool)
    /// ```
    pub fn clock_step(&mut self, cycles: u8) -> bool {
        self.cycles += cycles as usize; // cycles / 4?
        let mut draw_screen = false;

        match self.mode {
            // Screen gets drawn after final hblank
            ModeTypes::HBLANK => {
                if self.cycles >= HBLANK_LEN {
                    self.cycles = 0;
                    self.line += 1;

                    if self.line == VBLANK_LINE_START {
                        self.mode = ModeTypes::VBLANK;
                        // VBLANK is starting, time to draw screen
                        draw_screen = true;
                    } else {
                        self.mode = ModeTypes::OAMReadMode;
                    }
                }
            },
            // VBLANK lasts for 10 lines
            ModeTypes::VBLANK => {
                if self.cycles >= VBLANK_LEN {
                    self.cycles = 0;
                    self.line += 1;

                    if self.line > VBLANK_LINE_END {
                        self.mode = ModeTypes::OAMReadMode;
                        self.line = 0;
                    }
                }
            },
            ModeTypes::OAMReadMode => {
                if self.cycles >= OAM_READ_LEN {
                    self.cycles = 0;
                    self.mode = ModeTypes::VRAMReadMode;
                }
            },
            ModeTypes::VRAMReadMode => {
                if self.cycles >= VRAM_READ_LEN {
                    self.cycles = 0;
                    self.mode = ModeTypes::HBLANK;
                    // Renders scanline here, if we were to do so
                }
            }
        }

        draw_screen
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
    /// Returns the current clock mode, as an int
    ///
    /// Output:
    ///     Current mode (u8)
    /// ```
    pub fn get_mode(&self) -> u8 {
        match self.mode {
            ModeTypes::HBLANK => { 0 },
            ModeTypes::VBLANK => { 1 },
            ModeTypes::OAMReadMode => { 2 },
            ModeTypes::VRAMReadMode => { 3 }
        }
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
        self.mode == ModeTypes::VBLANK
    }
}
