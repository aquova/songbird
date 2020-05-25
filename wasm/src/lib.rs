// Functions to be exported out of .wasm for JS usage
use js_sys::DataView;
use wasm_bindgen::prelude::*;

use agba_core::cpu::Cpu;
use agba_core::utils::DISP_SIZE;

#[wasm_bindgen]
pub struct GB {
    cpu: Cpu
}

#[wasm_bindgen]
impl GB {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Result<GB, JsValue> {
        let cpu = Cpu::new();
        let gb = GB {
            cpu: cpu
        };

        Ok(gb)
    }

    #[wasm_bindgen]
    pub fn init_cpu(&mut self) {
        self.cpu.init();
    }

    #[wasm_bindgen]
    pub fn load_rom(&mut self, data: DataView) {
        let mut rom_data: Vec<u8> = Vec::with_capacity(data.byte_length());

        for i in 0..data.byte_length() {
            rom_data.push(data.get_uint8(i));
        }

        self.cpu.load_game(&rom_data)
    }

    #[wasm_bindgen]
    pub fn tick(&mut self) -> bool {
        self.cpu.tick()
    }
}

// #[no_mangle]
// pub fn get_palette() -> [u8; 4] {
//     unsafe {
//         GB.get_palette()
//     }
// }

// #[no_mangle]
// pub fn get_gfx() -> [u8; DISP_SIZE] {
//     unsafe {
//         GB.render()
//     }
// }
