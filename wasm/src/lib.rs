// Functions to be exported out of .wasm for JS usage
use js_sys::Uint8Array;
use wasm_bindgen::prelude::*;
use wasm_bindgen::{Clamped, JsCast};
use web_sys::{ImageData, KeyboardEvent};

use songbird_core::cpu::Cpu;
use songbird_core::io::Buttons;
use songbird_core::utils::{SCREEN_HEIGHT, SCREEN_WIDTH};

#[wasm_bindgen]
pub struct GB {
    cpu: Cpu,
    ctx: web_sys::CanvasRenderingContext2d
}

#[wasm_bindgen]
impl GB {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Result<GB, JsValue> {
        let mut cpu = Cpu::new();
        cpu.reset();

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

    /// ```
    /// Reset
    ///
    /// Resets the CPU
    /// ```
    #[wasm_bindgen]
    pub fn reset(&mut self) {
        self.cpu.reset();
    }

    /// ```
    /// Load ROM
    ///
    /// Loads game data into memory
    ///
    /// Input:
    ///     JS data object (Uint8Array)
    /// ```
    #[wasm_bindgen]
    pub fn load_rom(&mut self, data: Uint8Array) {
        let mut rom_data: Vec<u8> = Vec::new();

        for i in 0..data.byte_length() {
            rom_data.push(data.get_index(i));
        }

        self.cpu.load_game(&rom_data)
    }

    /// ```
    /// Tick
    ///
    /// Performs one tick of emulation
    ///
    /// Output:
    ///     Whether it is time to render a frame (bool)
    /// ```
    #[wasm_bindgen]
    pub fn tick(&mut self) -> bool {
        self.cpu.tick()
    }

    /// ```
    /// Get title
    ///
    /// Get title of game, as stored in ROM header
    ///
    /// Output:
    ///     Internal game title (String)
    /// ```
    #[wasm_bindgen]
    pub fn get_title(&self) -> String {
        self.cpu.get_title().to_string()
    }

    /// ```
    /// Draw screen
    ///
    /// Renders a frame to HTML5 canvas
    /// ```
    #[wasm_bindgen]
    pub fn draw_screen(&mut self) {
        let mut disp_arr = self.cpu.render();
        let img_data = ImageData::new_with_u8_clamped_array_and_sh(Clamped(&mut disp_arr), SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32).unwrap();
        self.ctx.put_image_data(&img_data, 0.0, 0.0).unwrap();
    }

    /// ```
    /// Handle key event
    ///
    /// Sends HTML keypresses to the emulator
    ///
    /// Inputs:
    ///     Browser key event (KeyboardEvent)
    ///     Whether key was pressed or released (bool)
    /// ```
    #[wasm_bindgen]
    pub fn handle_key(&mut self, event: KeyboardEvent, pressed: bool) {
        let key = event.key();
        let btn = GB::key2btn(&key);
        if btn.is_some() {
            self.cpu.toggle_button(btn.unwrap(), pressed);
        }
    }

    /// ```
    /// Key to Button
    ///
    /// Converts keycode into GameBoy button
    ///
    /// Input:
    ///     JS key (String)
    ///
    /// Output:
    ///     Gameboy button (Option<Buttons>)
    /// ```
    fn key2btn(key: &str) -> Option<Buttons> {
        match key {
            "ArrowDown" =>    { Some(Buttons::Down)   },
            "ArrowUp" =>      { Some(Buttons::Up)     },
            "ArrowRight" =>   { Some(Buttons::Right)  },
            "ArrowLeft" =>    { Some(Buttons::Left)   },
            "Enter" =>        { Some(Buttons::Start)  },
            "Backspace" =>    { Some(Buttons::Select) },
            "x" =>            { Some(Buttons::A)      },
            "z" =>            { Some(Buttons::B)      },
            _ =>              { None                  }
        }
    }
}
