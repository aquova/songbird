// Functions to be exported out of .wasm for JS usage
use js_sys::DataView;
use wasm_bindgen::prelude::*;
use wasm_bindgen::{Clamped, JsCast};
use web_sys::ImageData;

use agba_core::cpu::Cpu;
use agba_core::utils::{SCREEN_HEIGHT, SCREEN_WIDTH};

//                           R,   G,   B,   A
const BLACK: [u8; 4] =      [0,   0,   0,   255];
const LIGHT_GRAY: [u8; 4] = [148, 148, 165, 255];
const DARK_GRAY: [u8; 4] =  [107, 107, 90,  255];
const WHITE: [u8; 4] =      [255, 255, 255, 255];

const COLORS: [[u8; 4]; 4] = [
    WHITE,
    LIGHT_GRAY,
    DARK_GRAY,
    BLACK
];

#[wasm_bindgen]
pub struct GB {
    cpu: Cpu,
    ctx: web_sys::CanvasRenderingContext2d
}

#[wasm_bindgen]
impl GB {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Result<GB, JsValue> {
        let cpu = Cpu::new();

        let document = web_sys::window().unwrap().document().unwrap();
        let canvas = document.get_element_by_id("canvas").unwrap();
        let canvas: web_sys::HtmlCanvasElement = canvas
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .map_err(|_| ())
            .unwrap();

        let ctx = canvas.get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap();

        let gb = GB {
            cpu: cpu,
            ctx: ctx
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
    pub fn run(&mut self) {
        let draw_time = self.cpu.tick();
        // if draw_time {
        self.draw_screen();
        // }
    }

    #[wasm_bindgen]
    pub fn get_title(&self) -> String {
        self.cpu.get_title().to_string()
    }

    fn draw_screen(&mut self) {
        let disp_arr = self.cpu.render();
        let palette = self.cpu.get_palette();

        let mut data = Vec::new();
        for y in 0..SCREEN_HEIGHT {
            for x in 0..SCREEN_WIDTH {
                let index = y * SCREEN_WIDTH + x;
                let pixel = disp_arr[index];
                let palette_val = palette[pixel as usize];
                let color_arr = COLORS[palette_val as usize];
                for i in 0..color_arr.len() {
                    data.push(color_arr[i]);
                }
            }
        }

        let width = SCREEN_WIDTH as u32;
        let height = SCREEN_HEIGHT as u32;
        let img_data = ImageData::new_with_u8_clamped_array_and_sh(Clamped(&mut data), width, height).unwrap();
        self.ctx.put_image_data(&img_data, 0.0, 0.0).unwrap();
    }
}
