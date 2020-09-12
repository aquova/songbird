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

    /// ```
    /// Reset
    ///
    /// Resets the CPU
    /// ```
    #[wasm_bindgen]
    pub fn reset(&mut self) {
        self.cpu = Cpu::new();
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

        // TODO: Add option to allow for DMG force in browser
        self.cpu.load_game(&rom_data, false)
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
    /// Load save data
    ///
    /// Loads save data into external RAM
    ///
    /// Input:
    ///     Battery RAM data (Uint8Array)
    /// ```
    #[wasm_bindgen]
    pub fn load_save_data(&mut self, data: Uint8Array) {
        self.cpu.write_ext_ram(&(data.to_vec()));
    }

    /// ```
    /// Get save data
    ///
    /// Gets contents of external RAM to be saved
    ///
    /// Output:
    ///     Battery-backed RAM contents (Uint8Array)
    /// ```
    #[wasm_bindgen]
    pub fn get_save_data(&self) -> Uint8Array {
        let data = self.cpu.get_ext_ram();
        let data_len = data.len() as u32;
        let output_array = Uint8Array::new_with_length(data_len);
        for i in 0..data_len {
            output_array.set_index(i, data[i as usize]);
        }

        output_array
    }

    /// ```
    /// Is battery dirty?
    ///
    /// Has battery-backed RAM been modified?
    ///
    /// Output:
    ///     Whether external RAM has been written to (bool)
    /// ```
    #[wasm_bindgen]
    pub fn is_battery_dirty(&self) -> bool {
        self.cpu.is_battery_dirty()
    }

    /// ```
    /// Has battery
    ///
    /// Returns whether game has an external battery
    ///
    /// Output:
    ///     Whether cartridge has a battery (bool)
    /// ```
    #[wasm_bindgen]
    pub fn has_battery(&self) -> bool {
        self.cpu.has_battery()
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
