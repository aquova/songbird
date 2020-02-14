// Functions to be exported out of .wasm for JS usage
use agba_core::cpu::Cpu;
use agba_core::utils::DISP_SIZE;

use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static! {
    static ref GB: Mutex<Cpu> = Mutex::new(Cpu::new());
    static ref ROM: Mutex<Vec<u8>> = Mutex::new(Vec::new());
}

#[no_mangle]
pub fn push(val: u8) {
    ROM.lock().unwrap().push(val);
}

#[no_mangle]
pub fn get_palette() -> [u8; 4] {
    GB.lock().unwrap().get_palette()
}

#[no_mangle]
pub fn load() {
    GB.lock().unwrap().load_game(ROM.lock().unwrap().to_vec());
}

#[no_mangle]
pub fn get_gfx() -> [u8; DISP_SIZE] {
    GB.lock().unwrap().render()
}

#[no_mangle]
pub fn tick() -> bool {
    GB.lock().unwrap().tick()
}
