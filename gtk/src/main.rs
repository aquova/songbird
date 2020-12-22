// Songbird GTK desktop build
// Austin Bricker 2020

use std::cell::RefCell;
use std::env::args;
use std::fs::{File, OpenOptions};
use std::io::{prelude::*, Read};
use std::path::PathBuf;
use std::rc::Rc;

use songbird_core::cpu::Cpu;
use songbird_core::io::Buttons;
use songbird_core::ppu::palette::Palette;
use songbird_core::utils::{SCREEN_HEIGHT, SCREEN_WIDTH};

use gio::prelude::*;
use glib::clone;
use gtk::{AccelFlags, prelude::*};
use gtk::{AccelGroup, Application, ApplicationWindow, Menu, MenuBar, MenuItem, Orientation, WindowPosition};

#[cfg(feature = "debug")]
use coredump::register_panic_handler;

const SCALE: usize = 5;
const WINDOW_WIDTH: usize = SCREEN_WIDTH * SCALE;
const WINDOW_HEIGHT: usize = SCREEN_HEIGHT * SCALE;

fn main() {
    #[cfg(feature = "debug")]
    register_panic_handler().unwrap();

    // let args: Vec<_> = args().collect();
    // let mut filename = None;
    // let mut dmg = false;

    // // Running my own argparse, because I don't care for how most of the crates work
    // for i in 1..args.len() {
    //     match args[i].as_str() {
    //         "--dmg" => {
    //             dmg = true;
    //         },
    //         "-" => {
    //             // Needed to send flags to debug builds, do nothing
    //         },
    //         _ => {
    //             if filename.is_none() {
    //                 filename = Some(args[i].clone());
    //             }
    //         }
    //     }
    // }

    let app = Application::new(Some("com.github.aquova.songbird"), Default::default()).expect("Initialization failed");
    app.connect_activate(|app| build_ui(app));
    app.run(&args().collect::<Vec<_>>());
}

fn build_ui(app: &Application) {
    let window = ApplicationWindow::new(app);
    window.set_title("Songbird");
    window.set_position(WindowPosition::Center);
    window.set_size_request(SCREEN_WIDTH as i32, SCREEN_HEIGHT as i32);

    let v_box = gtk::Box::new(Orientation::Vertical, 10);

    let accel_group = AccelGroup::new();
    window.add_accel_group(&accel_group);
    let menu_bar = MenuBar::new();

    // Create drop down menu, populate with items and append to menu bar
    let file_menu = Menu::new();
    let open = MenuItem::with_label("Open");
    let quit = MenuItem::with_label("Quit");
    file_menu.append(&open);
    file_menu.append(&quit);

    let file = MenuItem::with_label("File");
    file.set_submenu(Some(&file_menu));

    menu_bar.append(&file);

    let game_filepath: Rc<RefCell<Option<PathBuf>>> = Rc::new(RefCell::new(None));
    open.connect_activate(clone!(@weak window => move |_| {
        let filename = gtk_open_file(&window);
        if let Some(f) = filename {
            // println!("{:?}", f);
            *game_filepath.borrow_mut() = Some(f);
        }
    }));

    quit.connect_activate(clone!(@weak window => move |_| window.close()));

    // Set shortcut keys
    let (open_key, open_modifier) = gtk::accelerator_parse("<Primary>O");
    open.add_accelerator("activate", &accel_group, open_key, open_modifier, AccelFlags::VISIBLE);
    let (quit_key, quit_modifier) = gtk::accelerator_parse("<Primary>Q");
    quit.add_accelerator("activate", &accel_group, quit_key, quit_modifier, AccelFlags::VISIBLE);

    // Add items to window
    let placeholder = gtk::Label::new(Some("Placeholder item"));
    v_box.pack_start(&menu_bar, false, false, 0);
    v_box.pack_start(&placeholder, true, true, 0);

    window.add(&v_box);
    window.show_all();
}

fn gtk_open_file(win: &ApplicationWindow) -> Option<PathBuf> {
    let file_chooser = gtk::FileChooserDialog::new(
        Some("Open file"),
        Some(win),
        gtk::FileChooserAction::Open,
    );

    file_chooser.add_buttons(&[
        ("Open", gtk::ResponseType::Ok),
        ("Cancel", gtk::ResponseType::Cancel),
    ]);

    let run = file_chooser.run();
    if run == gtk::ResponseType::Ok {
        file_chooser.close();
        file_chooser.get_filename()
    } else {
        None
    }
}

// fn load_shader(display: &Display, shad: Shaders) -> Program {
//     match shad {
//         Shaders::None => {
//             Program::from_source(
//                 display,
//                 include_str!("shaders/base.vert"),
//                 include_str!("shaders/none.frag"),
//                 None
//             ).unwrap()
//         },
//         Shaders::CRT => {
//             Program::from_source(
//                 display,
//                 include_str!("shaders/base.vert"),
//                 include_str!("shaders/crt.frag"),
//                 None
//             ).unwrap()
//         },
//         Shaders::AsciiMono => {
//             Program::from_source(
//                 display,
//                 include_str!("shaders/base.vert"),
//                 include_str!("shaders/ascii-1bit.frag"),
//                 None
//             ).unwrap()
//         },
//         Shaders::AsciiColor => {
//             Program::from_source(
//                 display,
//                 include_str!("shaders/base.vert"),
//                 include_str!("shaders/ascii-color.frag"),
//                 None
//             ).unwrap()
//         },
//     }
// }

/// ```
/// Tick until draw
///
/// Repeatedly runs until it is time to render a frame
///
/// Inputs:
///     Game Boy CPU (&Cpu)
///     Filename of game ROM (&str)
/// ```
fn tick_until_draw(mut gb: &mut Cpu, filename: &str) {
    loop {
        let draw_time = gb.tick();

        if draw_time {
            break;
        }
    }

    // Limiting saving battery state to only once per frame.
    // Doing it every tick is both overkill and causes some unknown issue on
    // Windows which traps us in an infinite loop on this frame
    if gb.is_battery_dirty() {
        write_battery_save(gb, &filename);
    }
}

/// ```
/// Key to Button
///
/// Converts keycode into GameBoy button
///
/// Input:
///     Glium keybode keycode (VirtualKeyCode)
///
/// Output:
///     Gameboy button (Option<Buttons>)
/// ```
// fn key2btn(key: VirtualKeyCode) -> Option<Buttons> {
//     match key {
//         VirtualKeyCode::Down =>    { Some(Buttons::Down)   },
//         VirtualKeyCode::Up =>      { Some(Buttons::Up)     },
//         VirtualKeyCode::Right =>   { Some(Buttons::Right)  },
//         VirtualKeyCode::Left =>    { Some(Buttons::Left)   },
//         VirtualKeyCode::Return =>  { Some(Buttons::Start)  },
//         VirtualKeyCode::Back =>    { Some(Buttons::Select) },
//         VirtualKeyCode::X =>       { Some(Buttons::A)      },
//         VirtualKeyCode::Z =>       { Some(Buttons::B)      },
//         _ =>                       { None                  }
//     }
// }

/// ```
/// Setup emulator
///
/// Initializes emulation by loading ROM and saved data
///
/// Inputs:
///     Game Boy CPU object (&Cpu)
///     ROM file path (&str)
///     Whether to force DMG (bool)
/// ```
fn setup_emu(gb: &mut Cpu, filename: &str, force_dmg: bool) {
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

        if let Ok(mut f) = OpenOptions::new().read(true).open(filename) {
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
///     Name of ROM file (&str)
/// ```
fn write_battery_save(gb: &mut Cpu, gamename: &str) {
    if gb.has_battery() {
        let ram_data = gb.get_ext_ram();
        let mut filename = gamename.to_owned();
        filename.push_str(".sav");

        let mut file = OpenOptions::new().write(true).create(true).open(filename).expect("Error opening save file");
        file.write_all(ram_data).unwrap();
        file.flush().unwrap();
        gb.clean_battery_flag();
    }
}
