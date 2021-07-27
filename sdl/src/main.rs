// Songbird SDL desktop build
// Austin Bricker, 2019-2021

use songbird_core::cpu::Cpu;
use songbird_core::io::Buttons;
use songbird_core::utils::{DISP_SIZE, SCREEN_HEIGHT, SCREEN_WIDTH};

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::PixelFormatEnum;
use sdl2::render::Canvas;
use sdl2::video::Window;

use std::env;
use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::io::Read;

// Constants
const SCALE: usize = 5;

pub fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() == 1 {
        println!("cargo run path/to/game");
        return;
    }

    // Start game
    let mut gb = Cpu::new();
    let filename = &args[1];
    let rom = load_rom(filename);
    gb.load_game(&rom, false);
    load_battery_save(&mut gb, filename);
    let title = gb.get_title();

    // Set up SDL
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem.window(title, (SCALE * SCREEN_WIDTH) as u32, (SCALE * SCREEN_HEIGHT) as u32).position_centered().opengl().build().unwrap();
    let mut canvas = window.into_canvas().present_vsync().build().unwrap();
    canvas.clear();
    canvas.present();

    // Setup I/O
    let mut events = sdl_context.event_pump().unwrap();

    // Main loop
    'gameloop: loop {
        // Check for UI key presses
        for event in events.poll_iter() {
            match event {
                // Quit game
                Event::Quit{..} |
                Event::KeyDown{keycode: Some(Keycode::Escape), ..} |
                Event::KeyDown{keycode: Some(Keycode::Q), ..} => {
                    break 'gameloop;
                },
                // Send keypresses to CPU
                Event::KeyDown { keycode: Some(keycode), .. } => {
                    if let Some(btn) = key2btn(keycode) {
                        gb.toggle_button(btn, true);
                    }
                },
                // Send key releases to CPU
                Event::KeyUp { keycode: Some(keycode), .. } => {
                    if let Some(btn) = key2btn(keycode) {
                        gb.toggle_button(btn, false);
                    }
                },
                _ => {}
            }
        }

        // Game loop
        let draw_time = gb.tick();
        if draw_time {
            let disp_arr = gb.render();
            draw_screen(&disp_arr, &mut canvas);
        }

        // Update save file if needed
        if gb.is_battery_dirty() {
            write_battery_save(&mut gb, filename);
        }
    }
}

/// ```
/// Draw screen
///
/// Renders pixel data onto SDL2 canvas
///
/// Inputs:
///     Pixel data ([u8])
///     SDL2 Canvas (Canvas<Window>)
/// ```
fn draw_screen(data: &[u8; DISP_SIZE], canvas: &mut Canvas<Window>) {
    canvas.set_scale(SCALE as f32, SCALE as f32).unwrap();

    let texture_creator = canvas.texture_creator();
    let mut texture = texture_creator.create_texture_streaming(PixelFormatEnum::RGBA32, SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32).unwrap();
    texture.with_lock(None, |buffer: &mut [u8], _: usize| {
        for i in 0..data.len() {
            buffer[i] = data[i];
        }
    }).unwrap();

    canvas.copy(&texture, None, None).unwrap();
    canvas.present();
}

/// ```
/// Key to Button
///
/// Converts keycode into GameBoy button
///
/// Input:
///     SDL keybode keycode (Keycode)
///
/// Output:
///     Gameboy button (Option<Buttons>)
/// ```
fn key2btn(key: Keycode) -> Option<Buttons> {
    match key {
        Keycode::Down =>    { Some(Buttons::Down)   },
        Keycode::Up =>      { Some(Buttons::Up)     },
        Keycode::Right =>   { Some(Buttons::Right)  },
        Keycode::Left =>    { Some(Buttons::Left)   },
        Keycode::Return =>  { Some(Buttons::Start)  },
        Keycode::Select =>  { Some(Buttons::Select) },
        Keycode::X =>       { Some(Buttons::A)      },
        Keycode::Z =>       { Some(Buttons::B)      },
        _ =>                { None                  }
    }
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

/// ```
/// Write Battery save
///
/// Updates save file to latest contents of battery RAM
///
/// Inputs:
///     Game Boy CPU object (Cpu)
///     Name of ROM file (&str)
/// ```
fn write_battery_save(gb: &mut Cpu, gamename: &str) {
    if gb.has_battery() {
        let ram_data = gb.get_ext_ram();
        let mut filename = gamename.to_owned();
        filename.push_str(".sav");

        let mut file = OpenOptions::new().write(true).create(true).open(filename).expect("Error opening save file");
        file.write(ram_data).unwrap();
        gb.clean_battery_flag();
    }
}
