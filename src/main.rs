// agba Game Boy Emulator
// Austin Bricker, 2019-2020

// TODO: Need to provide portable SDL2 libraries
extern crate sdl2;

// Includes
use gb_core::cpu::Cpu;
use gb_core::debug::debugger;
use sdl2::event::Event;
use sdl2::image::LoadSurface;
use sdl2::keyboard::Keycode;
use sdl2::surface::Surface;
use std::{env, io, process};
use std::io::prelude::*;

// Constants
// const WIDTH: u32 = 160;
// const HEIGHT: u32 = 144;
const SCALE: u32 = 5;

pub fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() == 1 {
        println!("cargo run path/to/game");
        process::exit(1);
    }
    let mut paused = true;
    let mut debugging = false;

    // Start game
    let mut gb = Cpu::new();
    gb.load_game(&args[1]);

    // Initialize debugger
    let mut agbd = debugger::new();

    // Set up SDL
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    // let window = video_subsystem.window(&args[1], SCALE * WIDTH, SCALE * HEIGHT).position_centered().opengl().build().unwrap();
    let mut window = video_subsystem.window("agba", 128 * SCALE, 128 * SCALE).position_centered().opengl().build().unwrap();
    let window_icon = Surface::from_file("assets/icon_purple.png").unwrap();
    window.set_icon(window_icon);
    let mut canvas = window.into_canvas().build().unwrap();

    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();

    // Main loop
    'gameloop: loop {
        // Check for key presses
        for event in event_pump.poll_iter() {
            match event {
                // Quit game
                Event::Quit{..} |
                Event::KeyDown{keycode: Some(Keycode::Escape), ..} |
                Event::KeyDown{keycode: Some(Keycode::Q), ..} => {
                    break 'gameloop;
                },
                // Trigger debugger with ctrl + c
                Event::KeyDown{keycode: Some(Keycode::C), keymod, ..} if
                (keymod.contains(sdl2::keyboard::Mod::LCTRLMOD) ||
                 keymod.contains(sdl2::keyboard::Mod::RCTRLMOD)) => {
                    debugging = true;
                    agbd.print_info(gb.get_pc());
                },
                // Pause with Space
                Event::KeyDown{keycode: Some(Keycode::Space), ..} => {
                    paused = !paused;
                    if paused {
                        println!("Paused");
                    }
                },
                // Step through operation with N
                Event::KeyDown{keycode: Some(Keycode::N), ..} => {
                    if paused {
                        gb.tick();
                        gb.draw(&mut canvas);
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
                        agbd.print_registers(&gb);
                    },
                    _ => {
                        // Do nothing, accept another input
                    }
                }
            }
        }

        if !paused {
            // Game loop
            let draw_time = gb.tick();
            if draw_time {
                gb.draw(&mut canvas);
            }
            // Break if we hit a breakpoint
            if agbd.check_break(gb.get_pc()) {
                debugging = true;
            }
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
