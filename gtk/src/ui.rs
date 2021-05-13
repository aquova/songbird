use crate::menubar::{EmuMenubar, INIT_SCALE};

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
const MENUBAR_HEIGHT: i32 = 30;

pub enum UiAction {
    Quit,
    Load(PathBuf),
    BtnPress(Buttons),
    BtnRelease(Buttons),
    SetDMG(bool),
}

pub enum CoreAction {
    Render,
}

/// ```
/// Create UI
///
/// Function to create our GTK windows and initialize, as well as to handle rendering
///
/// Inputs:
///     Channel for messages from this thread to the main (Sender<UiAction>)
///     Channel for messages from the main thread to this one (Receiver<CoreAction>)
///     Shared frame buffer (Arc<Mutex<Vec<u8>>>)
/// ```
pub fn create_ui(
    ui_to_gb: Sender<UiAction>,
    gb_to_ui: Receiver<CoreAction>,
    frame: Arc<Mutex<Vec<u8>>>,
) {
    gtk::init().unwrap();
    let app = Application::new(Some("com.github.aquova.songbird"), Default::default()).expect("Initialization failed");
    let menubar = EmuMenubar::new();

    let window = ApplicationWindow::new(&app);
    window.set_title("Songbird");
    window.set_position(WindowPosition::Center);
    window.resize((INIT_SCALE * SCREEN_WIDTH) as i32, (INIT_SCALE * SCREEN_HEIGHT) as i32 + MENUBAR_HEIGHT);

    // Add items to window
    let v_box = gtk::Box::new(Orientation::Vertical, 0);
    let drawing_area = DrawingArea::new();
    v_box.pack_start(&menubar.menubar, false, false, 0);
    v_box.pack_start(&drawing_area, true, true, 0);
    window.add(&v_box);

    let accel_group = AccelGroup::new();
    window.add_accel_group(&accel_group);

    window.show_all();

    // Add event handlers for UI elements and keys
    connect_quit(&window, &menubar, &accel_group, &ui_to_gb);
    connect_open(&window, &menubar, &accel_group, &ui_to_gb);
    connect_keypress(&window, &ui_to_gb);
    connect_keyrelease(&window, &ui_to_gb);
    connect_force_dmg(&menubar, &ui_to_gb);
    connect_draw(&frame, &drawing_area, INIT_SCALE);
    connect_scale(&window, &drawing_area, &frame, &menubar);

    app.connect_activate(move |app| {
        app.add_window(&window);
    });

    // Sleep inbetween checking channels for rending messages
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

/// ```
/// Connect Quit
///
/// Sets up event handling for 'Quit' menubar option
///
/// Inputs:
///     GTK Application Window (&ApplicationWindow)
///     Our GTK menubar (&EmuMenubar)
///     Key combination shortcut handler (&AccelGroup)
///     Channel from UI thread to main (&Sender<UiAction>)
/// ```
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

/// ```
/// Connect Open
///
/// Sets up event handling for 'Open' menubar option
///
/// Inputs:
///     GTK Application Window (&ApplicationWindow)
///     Our GTK menubar (&EmuMenubar)
///     Key combination shortcut handler (&AccelGroup)
///     Channel from UI thread to main (&Sender<UiAction>)
/// ```
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

/// ```
/// Connect Key Press
///
/// Sets up event handling for key presses
///
/// Inputs:
///     GTK Application Window (&ApplicationWindow)
///     Channel from UI thread to main (&Sender<UiAction>)
/// ```
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

/// ```
/// Connect Key Release
///
/// Sets up event handling for key releases
///
/// Inputs:
///     GTK Application Window (&ApplicationWindow)
///     Channel from UI thread to main (&Sender<UiAction>)
/// ```
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

/// ```
/// Connect Force DMG
///
/// Sets up handling for the Force DMG menubar option
///
/// Inputs:
///     Our GTK menubar (&EmuMenubar)
///     Channel from UI thread to main (&Sender<UiAction>)
/// ```
fn connect_force_dmg(menubar: &EmuMenubar, ui_to_gb: &Sender<UiAction>) {
    let ui_to_gb_clone = ui_to_gb.clone();
    menubar.force_dmg.connect_toggled(move |btn| {
        let is_set = btn.get_active();
        ui_to_gb_clone.send(UiAction::SetDMG(is_set)).unwrap();
    });
}

/// ```
/// Connect Draw
///
/// Initializes our DrawingArea and sets scale factor
///
/// Inputs:
///     Our frame data to render (&Arc<Mutex<Vec<u8>>>)
///     Our canvas widget (&DrawingArea)
///     The scale factor (usize)
/// ```
fn connect_draw(frame: &Arc<Mutex<Vec<u8>>>, drawing_area: &DrawingArea, scale: usize) {
    // Initialize our drawing area.
    // Specifies which frame buffer is to be used, and scales to match our screen
    let frame = frame.clone();
    drawing_area.connect_draw(move |_, cr| {
        let data = frame.lock().unwrap().to_vec();
        // Despite the name, the DrawingArea requires data as BGRA, not ARGB
        let argb = rgba2bgra(&data);
        let img = ImageSurface::create_for_data(
            argb,
            Format::ARgb32,
            SCREEN_WIDTH as i32,
            SCREEN_HEIGHT as i32,
            Format::ARgb32.stride_for_width(SCREEN_WIDTH as u32).unwrap(),
        ).unwrap();

        let pattern = SurfacePattern::create(&img);
        pattern.set_filter(Filter::Nearest);
        cr.scale(scale as f64, scale as f64);
        cr.set_source(&pattern);
        cr.paint();

        Inhibit(false)
    });
}

/// ```
/// Connect scale
///
/// Event handling for the scale factor menubar options
///
/// Inputs:
///     The app window (&ApplicationWindow)
///     The rendering canvas (&DrawingArea)
///     Our frame data to render (&Arc<Mutex<Vec<u8>>>)
///     The UI menubar struct (&EmuMenubar)
/// ```
fn connect_scale(window: &ApplicationWindow, drawing_area: &DrawingArea, frame: &Arc<Mutex<Vec<u8>>>, menubar: &EmuMenubar) {
    let btn_num = menubar.scale_btns.len();
    let menubar_h = menubar.menubar.get_allocated_height();
    for (idx, btn) in menubar.scale_btns.iter().enumerate() {
        let window = window.clone();
        let drawing_area = drawing_area.clone();
        let frame = frame.clone();
        btn.connect_button_press_event(move |_, _evt| {
            let new_scale = btn_num - idx - 1;
            window.resize((new_scale * SCREEN_WIDTH) as i32, (new_scale * SCREEN_HEIGHT) as i32 + menubar_h);
            connect_draw(&frame, &drawing_area, new_scale);
            Inhibit(false)
        });
    }
}

/// ```
/// Show open dialog
///
/// Opens a new GTK file picker dialog menu
///
/// Input:
///     Our parent GTK window (&ApplicationWindow)
///
/// Output:
///     The path to our file, if any (Option<PathBuf>)
/// ```
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

/// ```
/// RGBA to BGRA
///
/// A helper function to convert RGBA encoded arrays to BGRA
///
/// Input:
///     RGBA encoded slice (&[u8])
///
/// Output:
///     Same data, but rearranged as BGRA ([u8; DISP_SIZE])
/// ```
fn rgba2bgra(rgba: &[u8]) -> [u8; DISP_SIZE] {
    let mut argb: [u8; DISP_SIZE] = [0; DISP_SIZE];

    for i in 0..(rgba.len() / COLOR_CHANNELS) {
        let idx = COLOR_CHANNELS * i;

        let r = rgba[idx];
        let g = rgba[idx + 1];
        let b = rgba[idx + 2];
        let a = rgba[idx + 3];

        argb[idx] = b;
        argb[idx + 1] = g;
        argb[idx + 2] = r;
        argb[idx + 3] = a;
    }

    argb
}
