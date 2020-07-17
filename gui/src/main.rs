// Songbird Game Boy Emulator Desktop
// Austin Bricker 2019-2020

// Includes
mod ui;

#[macro_use]
extern crate imgui;

use songbird_core::cpu::Cpu;
use crate::ui::ImguiSystem;

use std::{env, process};
use std::fs::{File, OpenOptions};
use std::io::Read;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() == 1 {
        println!("cargo run path/to/game");
        process::exit(1);
    }

    // Start game
    let mut gb = Cpu::new();
    let filename = args[1].clone();
    let rom = load_rom(&filename);
    gb.load_game(&rom);
    load_battery_save(&mut gb, &filename);
    let title = gb.get_title();

    // Setup window and GUI
    let is = ImguiSystem::new(title);
    is.main_loop(gb, filename);
}


/// ```
/// Load ROM
///
/// Loads game ROM into memory
///
/// Input:
///     Path to game (&str)
///
/// Output:
///     Game data (Vec<u8>)
/// ```
fn load_rom(path: &str) -> Vec<u8> {
    let mut buffer: Vec<u8> = Vec::new();

    let mut f = File::open(path).expect("Error opening ROM");
    f.read_to_end(&mut buffer).expect("Error reading ROM to buffer");

    buffer
}

/// ```
/// Load Battery save
///
/// Loads battery save file (if one exists)
///
/// Inputs:
///     Game Boy CPU object (Cpu)
///     Name of ROM file (&str)
/// ```
fn load_battery_save(gb: &mut Cpu, gamename: &str) {
    if gb.has_battery() {
        let mut battery_ram: Vec<u8> = Vec::new();
        let mut filename = gamename.to_owned();
        filename.push_str(".sav");

        let f = OpenOptions::new().read(true).open(filename);
        if f.is_ok() {
            f.unwrap().read_to_end(&mut battery_ram).expect("Error reading external RAM");
            gb.write_ext_ram(&battery_ram);
        }
    }
}
