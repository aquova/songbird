// Songbird GTK desktop build
// Austin Bricker 2020-2021

mod menubar;
mod ui;

use crate::ui::*;

use songbird_core::cpu::Cpu;
use songbird_core::utils::DISP_SIZE;

use std::cell::RefCell;
use std::fs::{File, OpenOptions};
use std::io::{prelude::*, Read};
use std::path::PathBuf;
use std::rc::Rc;
use std::sync::{Arc, Mutex, mpsc::channel};
use std::thread;
use std::time::Duration;

fn main() {
    let mut gb = Cpu::new();
    let mut force_dmg = false;
    let filename: Rc<RefCell<Option<PathBuf>>> = Rc::new(RefCell::new(None));
    let (ui_to_gb_sender, ui_to_gb_receiver) = channel::<UiAction>();
    let (gb_to_ui_sender, gb_to_ui_receiver) = channel::<CoreAction>();

    // We will share our screen buffer with the UI/Rendering thread via
    // a Arc<Mutex>

    // The UI thread is expecting this initial vec to be DISP_SIZE.
    // Even when it's initialized, it makes comparisons against height x width,
    // and will fail when given a vector of length 0.
    let frame = Arc::new(Mutex::new(vec![0; DISP_SIZE]));

    // The UI and rendering will be performed on a separate thread,
    // which will communicate to this one via UiAction and CoreAction
    // messages sent thru channels
    let thread_frame = frame.clone();
    let ui_thread = thread::spawn(move || {
        create_ui(ui_to_gb_sender, gb_to_ui_receiver, thread_frame);
    });

    // Main loop
    loop {
        if let Ok(evt) = ui_to_gb_receiver.try_recv() {
            // Parse messages sent from UI thread
            match evt {
                UiAction::Quit => break,
                UiAction::Load(f) => {
                    setup_emu(&mut gb, &f, force_dmg);
                    filename.borrow_mut().replace(f);
                },
                UiAction::Reset => {
                    if let Some(f) = filename.borrow().as_ref() {
                        gb = Cpu::new();
                        setup_emu(&mut gb, &f, force_dmg);
                    }
                },
                UiAction::BtnPress(btn) => {
                    gb.toggle_button(btn, true);
                },
                UiAction::BtnRelease(btn) => {
                    gb.toggle_button(btn, false);
                },
                UiAction::SetDMG(should_force) => {
                    force_dmg = should_force;
                }
            }
        }

        // Perform backend emulation, send message to UI/Rendering thread when ready
        if let Some(f) = filename.borrow().as_ref() {
            tick_until_draw(&mut gb, &f);
            let disp_arr = gb.render();
            if let Ok(mut buffer) = frame.lock() {
                *buffer = disp_arr.to_vec();
            }

            gb_to_ui_sender.send(CoreAction::Render).unwrap();
        }

        // TODO: Make this more precise than simply sleeping
        thread::sleep(Duration::from_millis(FRAME_DELAY as u64));
    }
    ui_thread.join().unwrap();
}

/// ```
/// Tick until draw
///
/// Repeatedly runs until it is time to render a frame
///
/// Inputs:
///     Game Boy CPU (&Cpu)
///     Filename of game ROM (&PathBuf)
/// ```
fn tick_until_draw(gb: &mut Cpu, filename: &PathBuf) {
    // Keep ticking until returns true, indicating time to render
    while !gb.tick() {}

    // Limiting saving battery state to only once per frame.
    // Doing it every tick is both overkill and causes some unknown issue on
    // Windows which traps us in an infinite loop on this frame
    if gb.is_battery_dirty() {
        write_battery_save(gb, &filename);
    }
}

/// ```
/// Setup emulator
///
/// Initializes emulation by loading ROM and saved data
///
/// Inputs:
///     Game Boy CPU object (&Cpu)
///     ROM file path (&PathBuf)
///     Whether to force DMG (bool)
/// ```
fn setup_emu(gb: &mut Cpu, filename: &PathBuf, force_dmg: bool) {
    // In case anything is currently running, simply make a new Cpu instance
    *gb = Cpu::new();
    let rom = load_rom(filename);
    gb.load_game(&rom, force_dmg);
    load_battery_save(gb, filename);
}

/// ```
/// Load ROM
///
/// Loads game ROM into memory
///
/// Input:
///     Path to game (&PathBuf)
///
/// Output:
///     Game data (Vec<u8>)
/// ```
fn load_rom(path: &PathBuf) -> Vec<u8> {
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
///     Name of ROM file (&PathBuf)
/// ```
fn load_battery_save(gb: &mut Cpu, filename: &PathBuf) {
    if gb.has_battery() {
        let mut battery_ram: Vec<u8> = Vec::new();
        let mut savename = filename.clone();
        savename.set_extension("sav");

        if let Ok(mut f) = OpenOptions::new().read(true).open(savename) {
            f.read_to_end(&mut battery_ram).expect("Error reading external RAM");
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
///     Name of ROM file (&PathBuf)
/// ```
fn write_battery_save(gb: &mut Cpu, filename: &PathBuf) {
    if gb.has_battery() {
        let ram_data = gb.get_ext_ram();
        let mut savename = filename.clone();
        savename.set_extension("sav");

        let mut file = OpenOptions::new().write(true).create(true).open(savename).expect("Error opening save file");
        file.write_all(ram_data).unwrap();
        file.flush().unwrap();
        gb.clean_battery_flag();
    }
}
