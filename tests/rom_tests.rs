extern crate gb_core;

use gb_core::cartridge::MBC;
use gb_core::cpu::*;

// Going to use a ROM for testing that definitely doesn't rhyme with "Metris"
// TODO: Someday make your own ROM you can use for unit tests
const GAME_PATH: &str = "tests/test_roms/Tetris.gb";

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
