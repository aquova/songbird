// Chip-8 Emulator
// Austin Bricker, 2019

extern crate sdl2;

// Includes
use gb_core::system::Gameboy;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::{env, process};

const WIDTH: u32 = 160;
const HEIGHT: u32 = 144;
const SCALE: u32 = 1;

pub fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() == 1 {
        println!("cargo run path/to/game");
        process::exit(1);
    }
    let mut paused = false;

    // Set up SDL
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem.window(&args[1], WIDTH, HEIGHT).position_centered().opengl().build().unwrap();
    let mut canvas = window.into_canvas().build().unwrap();

    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut gb = Gameboy::new();
    // let mut game = ROM::new();
    // game.load_cart(&args[1]);

    // Main loop
    'gameloop: loop {
        // Check for key presses
        for event in event_pump.poll_iter() {
            match event {
                // Quit game
                Event::Quit{..} | Event::KeyDown{keycode: Some(Keycode::Escape), ..} => {
                    break 'gameloop;
                },
                Event::KeyDown{keycode: Some(Keycode::Space), ..} => {
                    paused = !paused;
                    if paused {
                        println!("Paused");
                    }
                },
                _ => {}
            }
        }

        // Game loop
        if !paused {
            // gb.tick();
            draw();
        }
    }
}

fn draw() {

}
