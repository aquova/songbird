// Songbird Game Boy Emulator Desktop
// Austin Bricker 2019-2020

// Includes
use songbird_core::cpu::Cpu;
use songbird_core::io::Buttons;
use songbird_core::utils::{DISP_SIZE, SCREEN_HEIGHT, SCREEN_WIDTH};

use glium::glutin::ContextBuilder;
use glium::glutin::dpi::LogicalSize;
use glium::glutin::event::{ElementState, Event, KeyboardInput, WindowEvent, VirtualKeyCode};
use glium::glutin::event_loop::{ControlFlow, EventLoop};
use glium::glutin::window::WindowBuilder;

use glium::BlitTarget;
use glium::Display;
use glium::Surface;
use glium::texture::{MipmapsOption, RawImage2d, Texture2d, UncompressedFloatFormat};
use glium::uniforms::MagnifySamplerFilter;

use std::{env, process};
use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::io::Read;

// Constants
const SCALE: usize = 5;
const WINDOW_WIDTH: u32 = (SCREEN_WIDTH * SCALE) as u32;
const WINDOW_HEIGHT: u32 = (SCREEN_HEIGHT * SCALE) as u32;

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
    let wb = WindowBuilder::new().with_inner_size(LogicalSize {
                    width: WINDOW_WIDTH,
                    height: WINDOW_HEIGHT,
                }).with_title(title);
    let cb = ContextBuilder::new();
    let display = Display::new(wb, cb, &event_loop).unwrap();
    let dest_texture = Texture2d::empty_with_format(
        &display,
        UncompressedFloatFormat::U8U8U8U8,
        MipmapsOption::NoMipmap,
        WINDOW_WIDTH,
        WINDOW_HEIGHT
    ).unwrap();

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
                if gb.tick() {
                    let disp_arr = gb.render();
                    draw_screen(&disp_arr, &display, &dest_texture);
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
/// Renders pixel data onto window
///
/// Inputs:
///     Pixel data ([u8])
/// ```
fn draw_screen(data: &[u8; DISP_SIZE], display: &Display, dest: &Texture2d) {
    let image = RawImage2d::from_raw_rgba_reversed(&data.to_vec(), (SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32));

    let texture = Texture2d::new(display, image).unwrap();
    let dest_rect = BlitTarget {
        left: 0,
        bottom: 0,
        width: WINDOW_WIDTH as i32,
        height: WINDOW_HEIGHT as i32,
    };

    texture.as_surface().blit_whole_color_to(
        &dest.as_surface(),
        &dest_rect,
        MagnifySamplerFilter::Linear
    );

    let target = display.draw();
    dest.as_surface().fill(&target, MagnifySamplerFilter::Linear);
    target.finish().unwrap();
}
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
