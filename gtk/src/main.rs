// Songbird GTK desktop build
// Austin Bricker 2020

mod menubar;

use std::env::args;
use std::fs::{File, OpenOptions};
use std::io::{prelude::*, Read};
use std::path::PathBuf;

use songbird_core::cpu::Cpu;
use songbird_core::io::Buttons;
use songbird_core::ppu::palette::Palette;
use songbird_core::utils::{SCREEN_HEIGHT, SCREEN_WIDTH};
use crate::menubar::EmuMenubar;

use gio::prelude::*;
use gtk::prelude::*;
use gtk::{AccelFlags, AccelGroup, Application, ApplicationWindow, FileChooserAction, FileChooserDialog, Orientation, WindowPosition};

#[cfg(feature = "debug")]
use coredump::register_panic_handler;

const SCALE: usize = 5;
const WINDOW_WIDTH: usize = SCREEN_WIDTH * SCALE;
const WINDOW_HEIGHT: usize = SCREEN_HEIGHT * SCALE;

struct App {
    window: ApplicationWindow,
    accel_group: AccelGroup,
    menubar: EmuMenubar,
}

impl App {
    pub fn new(app: &Application) -> Self {
        let window = ApplicationWindow::new(app);
        window.set_title("Songbird");
        window.set_position(WindowPosition::Center);
        window.set_size_request(WINDOW_WIDTH as i32, WINDOW_HEIGHT as i32);

        // Add items to window
        let v_box = gtk::Box::new(Orientation::Vertical, 10);
        let menubar = EmuMenubar::new();
        let placeholder = gtk::Label::new(Some("Placeholder item"));
        v_box.pack_start(&menubar.menubar, false, false, 0);
        v_box.pack_start(&placeholder, true, true, 0);
        window.add(&v_box);

        let accel_group = AccelGroup::new();
        window.add_accel_group(&accel_group);

        window.show_all();

        let us = Self {
            window,
            accel_group,
            menubar,
        };

        us.connect_events();
        us
    }

    fn connect_events(&self) {
        let window = self.window.clone();
        self.menubar.quit_btn.connect_activate(move |_| window.close());
        let (quit_key, quit_modifier) = gtk::accelerator_parse("<Primary>Q");
        self.menubar.quit_btn.add_accelerator("activate", &self.accel_group, quit_key, quit_modifier, AccelFlags::VISIBLE);

        let window = self.window.clone(); // Shadow another one I guess
        self.menubar.open_btn.connect_activate(move |_| {
            let filename = gtk_open_file(&window);
            if let Some(f) = filename {
                println!("{:?}", f);
            }
        });

        // Set shortcut keys
        let (open_key, open_modifier) = gtk::accelerator_parse("<Primary>O");
        self.menubar.open_btn.add_accelerator("activate", &self.accel_group, open_key, open_modifier, AccelFlags::VISIBLE);

    }
}

fn main() {
    #[cfg(feature = "debug")]
    register_panic_handler().unwrap();

    let application = Application::new(Some("com.github.aquova.songbird"), Default::default()).expect("Initialization failed");
    application.connect_activate(|application| {
        let _app = App::new(application);
    });
    application.run(&args().collect::<Vec<_>>());
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
