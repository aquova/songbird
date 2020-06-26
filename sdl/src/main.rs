// songbird Game Boy Emulator
// Austin Bricker, 2019-2020

extern crate sdl2;

// Includes
use songbird_core::cpu::Cpu;
use songbird_core::debug::debugger;
use songbird_core::io::Buttons;
use songbird_core::utils::{DISP_SIZE, SCREEN_HEIGHT, SCREEN_WIDTH};

#[cfg(feature = "debug")]
use coredump::register_panic_handler;

use sdl2::event::Event;
use sdl2::keyboard::{Keycode, Mod};
use sdl2::pixels::PixelFormatEnum;
use sdl2::render::Canvas;
use sdl2::video::Window;

use std::{env, io, process};
use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::io::Read;
use std::time::{Duration, SystemTime};
use std::thread::sleep;

// Constants
const SCALE: usize = 5;
const FRAME_TIME: u64 = 16670; // In microseconds

pub fn main() {
    #[cfg(feature = "debug")]
    register_panic_handler().unwrap();

    let args: Vec<_> = env::args().collect();
    if args.len() == 1 {
        println!("cargo run path/to/game");
        process::exit(1);
    }

    // Start game
    let mut gb = Cpu::new();
    let filename = &args[1];
    let rom = load_rom(filename);
    gb.load_game(&rom);
    load_battery_save(&mut gb, filename);
    let title = gb.get_title();

    // Initialize debugger
    let mut agbd = debugger::new();
    let mut debugging = false;
    let trace_log = false;

    // Set up SDL
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem.window(title, (SCALE * SCREEN_WIDTH) as u32, (SCALE * SCREEN_HEIGHT) as u32).position_centered().opengl().build().unwrap();
    let mut canvas = window.into_canvas().present_vsync().build().unwrap();
    canvas.clear();
    canvas.present();

    // Setup I/O
    let mut events = sdl_context.event_pump().unwrap();

    let frame_duration = Duration::from_micros(FRAME_TIME);
    let mut last_frame = SystemTime::now();

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
                // Trigger debugger with ctrl + c
                Event::KeyDown{keycode: Some(Keycode::C), keymod, ..} if
                (keymod.contains(Mod::LCTRLMOD) ||
                    keymod.contains(Mod::RCTRLMOD)) => {
                    debugging = true;
                    agbd.print_info(gb.get_pc());
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

        // Debugging menu
        if debugging {
            'debugloop: loop {
                // Print console prompt ala gdb
                print!("(agbd) ");
                io::stdout().flush().unwrap();

                let mut input = String::new();
                // Await user input
                let stdin = io::stdin();
                stdin.read_line(&mut input).expect("Your user input was... weird");
                trim_newline(&mut input);
                let words: Vec<&str> = input.split(" ").collect();

                match words[0] {
                    // TODO: Better string matching, abbreviations?
                    // TODO: Maybe move this into debugger module

                    // Add new breakpoint
                    "b" => {
                        let hex = u16::from_str_radix(words[1], 16);
                        match hex {
                            Ok(addr) => {
                                agbd.add_break(addr);
                            },
                            Err(e) => {
                                println!("{} is not a valid address", e);
                            }
                        }
                    },
                    // Continue with execution
                    "c" => {
                        debugging = false;
                        break 'debugloop;
                    },
                    // Delete breakpoint (if any) at given address
                    "del" => {
                        let hex = u16::from_str_radix(words[1], 16);
                        match hex {
                            Ok(addr) => {
                                agbd.del_break(addr);
                            },
                            Err(e) => {
                                println!("{} is not a valid address", e);
                            }
                        }
                    },
                    // Disassemble next 5 instructions
                    "disass" => {
                        agbd.disassemble(&gb);
                    },
                    // Print help message
                    "help" => {
                        agbd.print_help();
                    },
                    // List watch/breakpoints
                    "info" => {
                        agbd.list_points();
                    },
                    // Execute next instruction
                    "n" => {
                        gb.tick();
                        println!("PC: ${:04x}", gb.get_pc());
                    },
                    // Print RAM values at given address
                    "p" => {
                        let hex = u16::from_str_radix(words[1], 16);
                        match hex {
                            Ok(addr) => {
                                agbd.print_ram(addr, &gb);
                            },
                            Err(e) => {
                                println!("{} is not a valid address", e);
                            }
                        }
                    },
                    // Quit program
                    "q" => {
                        break 'gameloop;
                    },
                    // List register values
                    "reg" => {
                        let info = agbd.print_registers(&gb);
                        println!("{}", info);
                    },
                    // Set watchpoint
                    "w" => {
                        let hex = u16::from_str_radix(words[1], 16);
                        match hex {
                            Ok(addr) => {
                                agbd.add_watch(addr);
                            },
                            Err(e) => {
                                println!("{} is not a valid address", e);
                            }
                        }
                    },
                    _ => {
                        println!("Unknown command.");
                    }
                }
            }
        }

        let watch_vals = agbd.get_watch_vals(&gb);

        // Game loop
        let draw_time = gb.tick();
        if draw_time {
            let disp_arr = gb.render();

            // Need to align to 60 FPS before drawing screen
            let elapsed = last_frame.elapsed().unwrap();
            let frame_wait = frame_duration.checked_sub(elapsed);
            if frame_wait.is_some() {
                sleep(frame_wait.unwrap());
            }

            draw_screen(&disp_arr, &mut canvas);
            last_frame = SystemTime::now();
        }

        // Update save file if needed
        if gb.is_battery_dirty() {
            write_battery_save(&mut gb, filename);
        }

        // Break if we hit a break/watchpoint
        if agbd.check_break(gb.get_pc()) || agbd.check_watch(&gb, watch_vals) {
            debugging = true;
        }

        // Output to trace log if enabled
        if trace_log {
            log_to_file(&gb, &agbd);
        }
    }
}

/// ```
/// Trim Newline
///
/// Helper function that removes trailing newline characters
/// Works on *nix systems and Windows
///
/// Input:
///     String to trim (&mut String)
/// ```
fn trim_newline(s: &mut String) {
    // Strip newline
    if s.ends_with('\n') {
        s.pop();
        // For Windows
        if s.ends_with('\r') {
            s.pop();
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
    let mut battery_ram: Vec<u8> = Vec::new();
    let mut filename = gamename.to_owned();
    filename.push_str(".sav");

    let f = OpenOptions::new().read(true).open(filename);
    if f.is_ok() {
        f.unwrap().read_to_end(&mut battery_ram).expect("Error reading external RAM");
        gb.write_ext_ram(&battery_ram);
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
    let ram_data = gb.get_ext_ram();
    let mut filename = gamename.to_owned();
    filename.push_str(".sav");

    let mut file = OpenOptions::new().write(true).create(true).open(filename).expect("Error opening save file");
    file.write(ram_data).unwrap();
    gb.clean_battery_flag();
}

fn log_to_file(gb: &Cpu, debug: &debugger) {
    let mut f = OpenOptions::new().create(true).append(true).open("trace.log").expect("Error opening trace log file");

    let info = debug.print_registers(gb);
    f.write(info.as_bytes()).unwrap();
}
