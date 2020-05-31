// agba Game Boy Emulator
// Austin Bricker, 2019-2020

// TODO: Need to provide portable SDL2 libraries
extern crate sdl2;

// Includes
use agba_core::cpu::Cpu;
use agba_core::debug::debugger;
use agba_core::io::Buttons;
use agba_core::utils::{DISP_SIZE, SCREEN_HEIGHT, SCREEN_WIDTH};

use sdl2::event::Event;
use sdl2::image::LoadSurface;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::surface::Surface;
use sdl2::video::Window;

use std::fs::File;
use std::io::Read;
use std::{env, io, process, thread, time};
use std::io::prelude::*;

// Constants
const SCALE: usize = 5;
const CLOCK_SPEED_IN_NS: u64 = 238; // 4.2 MHz

// Colors
const BLACK: (u8, u8, u8)            = (0,   0,   0);
const LIGHT_GRAY: (u8, u8, u8)       = (148, 148, 165);
const DARK_GRAY: (u8, u8, u8)        = (107, 107, 90);
const WHITE: (u8, u8, u8)            = (255, 255, 255);

const COLORS: [(u8, u8, u8); 4] = [
    WHITE,
    LIGHT_GRAY,
    DARK_GRAY,
    BLACK,
];

pub fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() == 1 {
        println!("cargo run path/to/game");
        process::exit(1);
    }
    let mut paused = false;
    let mut debugging = false;

    // Start game
    let mut gb = Cpu::new();
    let rom = load_rom(&args[1]);
    gb.load_game(&rom);
    let title = gb.get_title();

    // Initialize debugger
    let mut agbd = debugger::new();

    // Set up SDL
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let mut window = video_subsystem.window(title, (SCALE * SCREEN_WIDTH) as u32, (SCALE * SCREEN_HEIGHT) as u32).position_centered().opengl().build().unwrap();
    let window_icon = Surface::from_file("assets/icon_purple.png").unwrap();
    window.set_icon(window_icon);
    let mut canvas = window.into_canvas().build().unwrap();

    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();

    let clock_sleep = time::Duration::from_nanos(CLOCK_SPEED_IN_NS);

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
                // Game controls
                // TODO: Put this into other function or something
                Event::KeyDown{keycode: Some(Keycode::Return), ..} => {
                    gb.toggle_button(Buttons::Start, true);
                },
                Event::KeyDown{keycode: Some(Keycode::Backspace), ..} => {
                    gb.toggle_button(Buttons::Selectj, true);
                },
                Event::KeyDown{keycode: Some(Keycode::X), ..} => {
                    gb.toggle_button(Buttons::A, true);
                },
                Event::KeyDown{keycode: Some(Keycode::Z), ..} => {
                    gb.toggle_button(Buttons::B, true);
                },
                Event::KeyDown{keycode: Some(Keycode::Up), ..} => {
                    gb.toggle_button(Buttons::Up, true);
                },
                Event::KeyDown{keycode: Some(Keycode::Left), ..} => {
                    gb.toggle_button(Buttons::Left, true);
                },
                Event::KeyDown{keycode: Some(Keycode::Down), ..} => {
                    gb.toggle_button(Buttons::Down, true);
                },
                Event::KeyDown{keycode: Some(Keycode::Right), ..} => {
                    gb.toggle_button(Buttons::Right, true);
                },
                Event::KeyUp{keycode: Some(Keycode::Return), ..} => {
                    gb.toggle_button(Buttons::Start, false);
                },
                Event::KeyUp{keycode: Some(Keycode::Backspace), ..} => {
                    gb.toggle_button(Buttons::Select, false);
                },
                Event::KeyUp{keycode: Some(Keycode::X), ..} => {
                    gb.toggle_button(Buttons::A, false);
                },
                Event::KeyUp{keycode: Some(Keycode::Z), ..} => {
                    gb.toggle_button(Buttons::B, false);
                },
                Event::KeyUp{keycode: Some(Keycode::Up), ..} => {
                    gb.toggle_button(Buttons::Up, false);
                },
                Event::KeyUp{keycode: Some(Keycode::Left), ..} => {
                    gb.toggle_button(Buttons::Left, false);
                },
                Event::KeyUp{keycode: Some(Keycode::Down), ..} => {
                    gb.toggle_button(Buttons::Down, false);
                },
                Event::KeyUp{keycode: Some(Keycode::Right), ..} => {
                    gb.toggle_button(Buttons::Right, false);
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
                        agbd.print_registers(&gb);
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

        if !paused {
            let watch_vals = agbd.get_watch_vals(&gb);

            // Game loop
            let draw_time = gb.tick();
            if draw_time {
                let disp_arr = gb.render();
                let palette = gb.get_palette();
                draw_screen(palette, disp_arr, &mut canvas);
            }

            // Break if we hit a break/watchpoint
            if agbd.check_break(gb.get_pc()) || agbd.check_watch(&gb, watch_vals) {
                debugging = true;
            }
        }

        thread::sleep(clock_sleep);
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

fn get_color(color: (u8, u8, u8)) -> Color {
    Color::RGB(color.0, color.1, color.2)
}

fn draw_screen(palette: [u8; 4], data: [u8; DISP_SIZE], canvas: &mut Canvas<Window>) {
    canvas.set_draw_color(get_color(WHITE));
    canvas.clear();

    for y in 0..SCREEN_HEIGHT {
        for x in 0..SCREEN_WIDTH {
            let index = y * SCREEN_WIDTH + x;
            let pixel = data[index];
            let color_val = COLORS[palette[pixel as usize] as usize];
            let color = get_color(color_val);
            canvas.set_draw_color(color);

            let rect = Rect::new(
                (x * SCALE) as i32,
                (y * SCALE) as i32,
                SCALE as u32,
                SCALE as u32
            );
            canvas.fill_rect(rect).expect("Unable to draw to canvas");
        }
    }

    canvas.present();
}

fn load_rom(path: &str) -> Vec<u8> {
    let mut buffer: Vec<u8> = Vec::new();

    let mut f = File::open(path).expect("Error opening ROM");
    f.read_to_end(&mut buffer).expect("Error reading ROM to buffer");

    buffer
}
