// Songbird Game Boy Emulator Desktop
// Austin Bricker 2019-2020

// Includes
use songbird_core::cpu::Cpu;
use songbird_core::io::Buttons;
use songbird_core::utils::{DISP_SIZE, SCREEN_HEIGHT, SCREEN_WIDTH};

use imgui::*;
use winit::dpi::LogicalSize;
use winit::event::{ElementState, Event, KeyboardInput, WindowEvent, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::Window;

use std::{env, process};
use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::io::Read;

// Constants
const SCALE: usize = 5;

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

    // Setup window
    let event_loop = EventLoop::new();
    let window = Window::new(&event_loop).unwrap();
    window.set_inner_size(LogicalSize {
        width: (SCREEN_WIDTH * SCALE) as f64,
        height: (SCREEN_HEIGHT * SCALE) as f64,
    });
    window.set_title(title);

    // Setup imgui
    // let mut imgui = imgui::Context::create();

    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => {
                // Exit program if specified
                *control_flow = ControlFlow::Exit;
            },
            Event::WindowEvent { event: WindowEvent::KeyboardInput {
                input: KeyboardInput { virtual_keycode: Some(keycode), state, ..}, ..}, ..} => {
                    // Send keyboard inputs to emulator core
                    if let Some(btn) = key2btn(keycode) {
                        gb.toggle_button(btn, state == ElementState::Pressed);
                    }
            },
            Event::RedrawEventsCleared => {
                // let ui = imgui.frame();

                if gb.tick() {
                    let disp_arr = gb.render();
                    // draw_screen(&disp_arr, &mut canvas);
                }

                // Update save file if needed
                if gb.is_battery_dirty() {
                    write_battery_save(&mut gb, &filename);
                }

            },
            _ => {}
        }
    });
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
// fn draw_screen(data: &[u8; DISP_SIZE], canvas: &mut Canvas<Window>) {
//     canvas.set_scale(SCALE as f32, SCALE as f32).unwrap();

//     let texture_creator = canvas.texture_creator();
//     let mut texture = texture_creator.create_texture_streaming(PixelFormatEnum::RGBA32, SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32).unwrap();
//     texture.with_lock(None, |buffer: &mut [u8], _: usize| {
//         for i in 0..data.len() {
//             buffer[i] = data[i];
//         }
//     }).unwrap();

//     canvas.copy(&texture, None, None).unwrap();
//     canvas.present();
// }

/// ```
/// Key to Button
///
/// Converts keycode into GameBoy button
///
/// Input:
///     SDL keybode keycode (VirtualKeyCode)
///
/// Output:
///     Gameboy button (Option<Buttons>)
/// ```
fn key2btn(key: VirtualKeyCode) -> Option<Buttons> {
    match key {
        VirtualKeyCode::Down =>    { Some(Buttons::Down)   },
        VirtualKeyCode::Up =>      { Some(Buttons::Up)     },
        VirtualKeyCode::Right =>   { Some(Buttons::Right)  },
        VirtualKeyCode::Left =>    { Some(Buttons::Left)   },
        VirtualKeyCode::Return =>  { Some(Buttons::Start)  },
        VirtualKeyCode::Back =>    { Some(Buttons::Select) },
        VirtualKeyCode::X =>       { Some(Buttons::A)      },
        VirtualKeyCode::Z =>       { Some(Buttons::B)      },
        _ =>                       { None                  }
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
