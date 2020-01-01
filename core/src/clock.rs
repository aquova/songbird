// Timings adapted from here: http://imrannazar.com/GameBoy-Emulation-in-JavaScript:-GPU-Timings

pub struct Clock {
    pub cycles: usize,
    pub line: u8,
    pub modeclock: u8,
    pub mode: u8
}

impl Clock {
    pub fn new() -> Clock {
        Clock {
            cycles: 0,
            line: 0,
            modeclock: 0,
            mode: 0
        }
    }

    /// ```
    /// Add Clock
    ///
    /// Adds the specified number of cycles to the clock
    ///
    /// Input:
    ///     Number of cycles (u8)
    /// ```
    pub fn add_cycles(&mut self, cycles: u8) {
        self.cycles += cycles as usize;
    }
}
