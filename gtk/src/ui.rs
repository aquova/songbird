use crate::menubar::EmuMenubar;

use songbird_core::io::Buttons;
use songbird_core::utils::{COLOR_CHANNELS, DISP_SIZE, SCREEN_HEIGHT, SCREEN_WIDTH};

use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{Receiver, Sender};

use cairo::{Filter, Format, ImageSurface, SurfacePattern};
use gdk::keys::Key;
use gio::prelude::*;
use glib::timeout_add_local;
use gtk::prelude::*;
use gtk::{AccelFlags, AccelGroup, Application, ApplicationWindow, DrawingArea, FileChooserAction, FileChooserDialog, FileFilter, Orientation, WindowPosition};

pub const FRAME_DELAY: u32 = 1000 / 60;
const SCALE: usize = 5;
const WINDOW_WIDTH: usize = SCREEN_WIDTH * SCALE;
const WINDOW_HEIGHT: usize = SCREEN_HEIGHT * SCALE;

pub enum UiAction {
    Quit,
    Load(PathBuf),
    BtnPress(Buttons),
    BtnRelease(Buttons),
}

pub enum CoreAction {
    Render,
}

pub fn create_ui(
    ui_to_gb: Sender<UiAction>,
    gb_to_ui: Receiver<CoreAction>,
    frame: Arc<Mutex<Vec<u8>>>,
) {
    gtk::init().unwrap();
    let app = Application::new(Some("com.github.aquova.songbird"), Default::default()).expect("Initialization failed");

    let window = ApplicationWindow::new(&app);
    window.set_title("Songbird");
    window.set_position(WindowPosition::Center);
    window.set_size_request(WINDOW_WIDTH as i32, WINDOW_HEIGHT as i32);

    // Add items to window
    let v_box = gtk::Box::new(Orientation::Vertical, 0);
    let menubar = EmuMenubar::new();
    let drawing_area = DrawingArea::new();
    v_box.pack_start(&menubar.menubar, false, false, 0);
    v_box.pack_start(&drawing_area, true, true, 0);
    window.add(&v_box);

    let accel_group = AccelGroup::new();
    window.add_accel_group(&accel_group);

    window.show_all();

    drawing_area.connect_draw(move |_, cr| {
        let data = frame.lock().unwrap().to_vec();
        // let argb = rgba2argb(&data);
        let img = ImageSurface::create_for_data(
            data,
            Format::ARgb32,
            SCREEN_WIDTH as i32,
            SCREEN_HEIGHT as i32,
            Format::ARgb32.stride_for_width(SCREEN_WIDTH as u32).unwrap(),
        ).unwrap();

        let pattern = SurfacePattern::create(&img);
        pattern.set_filter(Filter::Nearest);
        cr.set_source(&pattern);
        cr.scale(WINDOW_WIDTH as f64, WINDOW_HEIGHT as f64);
        cr.paint();

        Inhibit(false)
    });

    connect_quit(&window, &menubar, &accel_group, &ui_to_gb);
    connect_open(&window, &menubar, &accel_group, &ui_to_gb);
    connect_keypress(&window, &ui_to_gb);
    connect_keyrelease(&window, &ui_to_gb);

    app.connect_activate(move |app| {
        app.add_window(&window);
    });

    timeout_add_local(FRAME_DELAY, move || {
        if let Ok(evt) = gb_to_ui.try_recv() {
            match evt {
                CoreAction::Render => {
                    drawing_area.queue_draw_area(
                        0, 0,
                        drawing_area.get_allocated_width(),
                        drawing_area.get_allocated_height()
                    );
                }
            }
        }

        glib::Continue(true)
    });

    app.run(&[]);
    ui_to_gb.send(UiAction::Quit).unwrap();
}

fn connect_quit(
    window: &ApplicationWindow,
    menubar: &EmuMenubar,
    accel_group: &AccelGroup,
    ui_to_gb: &Sender<UiAction>
) {
    let window = window.clone();
    let ui_to_gb = ui_to_gb.clone();
    menubar.quit_btn.connect_activate(move |_| {
        ui_to_gb.send(UiAction::Quit).unwrap();
        window.close()
    });
    let (quit_key, quit_modifier) = gtk::accelerator_parse("<Primary>Q");
    menubar.quit_btn.add_accelerator("activate", accel_group, quit_key, quit_modifier, AccelFlags::VISIBLE);
}

fn connect_open(
    window: &ApplicationWindow,
    menubar: &EmuMenubar,
    accel_group: &AccelGroup,
    ui_to_gb: &Sender<UiAction>
) {
    let window = window.clone();
    let ui_to_gb = ui_to_gb.clone();
    menubar.open_btn.connect_activate(move |_| {
        let filename = show_open_dialog(&window);
        if let Some(f) = filename {
            ui_to_gb.send(UiAction::Load(f)).unwrap();
        }
    });
    let (open_key, open_modifier) = gtk::accelerator_parse("<Primary>O");
    menubar.open_btn.add_accelerator("activate", accel_group, open_key, open_modifier, AccelFlags::VISIBLE);
}

fn connect_keypress(window: &ApplicationWindow, ui_to_gb: &Sender<UiAction>) {
    let window = window.clone();
    let ui_to_gb = ui_to_gb.clone();
    window.connect_key_press_event(move|_, evt| {
        let mut key = evt.get_keyval();
        *key = gdk::keyval_to_upper(*key);
        if let Some(btn) = key2btn(key) {
            ui_to_gb.send(UiAction::BtnPress(btn)).unwrap();
        }

        Inhibit(false)
    });
}

fn connect_keyrelease(window: &ApplicationWindow, ui_to_gb: &Sender<UiAction>) {
    let window = window.clone();
    let ui_to_gb = ui_to_gb.clone();
    window.connect_key_release_event(move|_, evt| {
        let mut key = evt.get_keyval();
        *key = gdk::keyval_to_upper(*key);
        if let Some(btn) = key2btn(key) {
            ui_to_gb.send(UiAction::BtnRelease(btn)).unwrap();
        }

        Inhibit(false)
    });
}

fn show_open_dialog(parent: &ApplicationWindow) -> Option<PathBuf> {
    let mut file = None;
    let dialog = FileChooserDialog::new(Some("Select a Game Boy ROM"), Some(parent), FileChooserAction::Open);
    let filter = FileFilter::new();
    filter.add_mime_type("application/x-gameboy-rom");
    filter.add_mime_type("application/x-gameboy-color-rom");
    filter.set_name(Some("Game Boy ROM files"));
    dialog.add_filter(&filter);
    dialog.add_buttons(&[
        ("Open", gtk::ResponseType::Ok),
        ("Cancel", gtk::ResponseType::Cancel),
    ]);

    let result = dialog.run();
    if result == gtk::ResponseType::Ok {
        file = dialog.get_filename();
    }
    dialog.close();

    file
}

/// ```
/// Key to Button
///
/// Converts keycode into GameBoy button
///
/// Input:
///     GDK keybode keycode (Key)
///
/// Output:
///     Gameboy button (Option<Buttons>)
/// ```
fn key2btn(key: Key) -> Option<Buttons> {
    match key {
        gdk::keys::constants::Down =>    Some(Buttons::Down),
        gdk::keys::constants::Up =>      Some(Buttons::Up),
        gdk::keys::constants::Right =>   Some(Buttons::Right),
        gdk::keys::constants::Left =>    Some(Buttons::Left),
        gdk::keys::constants::Return =>  Some(Buttons::Start),
        gdk::keys::constants::Back =>    Some(Buttons::Select),
        gdk::keys::constants::X =>       Some(Buttons::A),
        gdk::keys::constants::Z =>       Some(Buttons::B),
        _ =>                             None
    }
}

fn rgba2argb(rgba: &[u8]) -> [u8; DISP_SIZE] {
    let mut argb: [u8; DISP_SIZE] = [0; DISP_SIZE];

    for i in 0..(rgba.len() / COLOR_CHANNELS) {
        let idx = COLOR_CHANNELS * i;
        let r = rgba[idx];
        let g = rgba[idx + 1];
        let b = rgba[idx + 2];
        let a = rgba[idx + 3];

        argb[idx] = a;
        argb[idx + 1] = r;
        argb[idx + 2] = g;
        argb[idx + 3] = b;
    }

    argb
}
