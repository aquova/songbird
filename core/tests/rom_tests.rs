extern crate songbird_core;

use songbird_core::cartridge::MBC;
use songbird_core::cpu::*;
use std::fs::File;
use std::io::Read;

// Going to use a ROM for testing that definitely doesn't rhyme with "Metris"
// TODO: Someday make your own ROM you can use for unit tests
const GAME_PATH: &str = "tests/Tetris.gb";

#[test]
/// Tests that the title can be extracted from ROM header
fn test_title() {
    let mut gb = Cpu::new();
    gb.load_game(GAME_PATH);
    let raw_title = String::from(gb.bus.get_title());

    // Remove trailing null characters from string
    let title = raw_title.trim_end_matches(char::from(0));
    assert_eq!(title, "TETRIS");
}

#[test]
/// Tests that the MBC type can be fetched from ROM header
fn test_get_mbc() {
    let mut gb = Cpu::new();
    gb.load_game(GAME_PATH);
    let mbc = gb.bus.get_mbc();

    match mbc {
        MBC::NONE => { /* Correct, do nothing */ },
        _ =>         { panic!("Incorrect")       }
    }
}

#[test]
/// Tests that the entire ROM is loaded when there is no MBC
fn test_mbc_none() {
    let mut gb = Cpu::new();
    gb.load_game(GAME_PATH);
    // TODO: Check that mbc is NONE
    let ram = gb.bus.get_ram();

    // Load game file into a buffer
    let mut buffer: Vec<u8> = Vec::new();
    let mut f = File::open(GAME_PATH).expect("Error opening test ROM");
    f.read_to_end(&mut buffer).expect("Error reading to buffer");

    // Ensure that RAM values equal those in the buffer
    for i in 0..buffer.len() {
        assert_eq!(ram[i], buffer[i]);
    }
}
