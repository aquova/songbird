// Songbird Game Boy Emulator Desktop
// Austin Bricker 2019-2020

// Includes
mod menu;

#[macro_use]
extern crate glium;

#[macro_use]
extern crate imgui;

#[cfg(feature = "debug")]
use coredump::register_panic_handler;

use crate::menu::{MenuState, DisplayOptions, Shaders};
use songbird_core::cpu::Cpu;
use songbird_core::debug::debugger;
use songbird_core::io::Buttons;
use songbird_core::ppu::palette::Palettes;
use songbird_core::utils::{SCREEN_HEIGHT, SCREEN_WIDTH};

use imgui::Context;
use imgui_glium_renderer::Renderer;
use imgui_winit_support::{HiDpiMode, WinitPlatform};

use glium::glutin::ContextBuilder;
use glium::glutin::dpi::LogicalSize;
use glium::glutin::event::{ElementState, Event, KeyboardInput, WindowEvent, VirtualKeyCode};
use glium::glutin::event_loop::{ControlFlow, EventLoop};
use glium::glutin::window::WindowBuilder;

use glium::{Display, Program, Surface, VertexBuffer};
use glium::index::{NoIndices, PrimitiveType};
use glium::texture::{RawImage2d, Texture2d};
use glium::uniforms::{MinifySamplerFilter, MagnifySamplerFilter};

use std::env;
use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::io::Read;
use std::process::exit;

// Constants
const SCALE: usize = 5;
const WINDOW_WIDTH: u32 = (SCREEN_WIDTH * SCALE) as u32;
const WINDOW_HEIGHT: u32 = (SCREEN_HEIGHT * SCALE) as u32;
const IMGUI_OFFSET: u32 = 8;
const MENU_BAR_HEIGHT: u32 = 11;

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
}

implement_vertex!(Vertex, position);

fn main() {
    #[cfg(feature = "debug")]
    register_panic_handler().unwrap();

    let args: Vec<_> = env::args().collect();
    let mut filename = None;
    let mut dmg = false;

    // Running my own argparse, because I don't care for how most of the crates work
    for i in 1..args.len() {
        match args[i].as_str() {
            "--dmg" => {
                dmg = true;
            },
            "-" => {
                // Needed to send flags to debug builds, do nothing
            },
            _ => {
                if filename.is_none() {
                    filename = Some(args[i].clone());
                }
            }
        }
    }

    let is = ImguiSystem::new();
    is.main_loop(filename, dmg);
}

// Imgui interface taken from here: https://gist.github.com/RainbowCookie32/7e5d76acf33d88f2145d5ebc047a5799
pub struct ImguiSystem {
    pub event_loop: EventLoop<()>,
    pub display: Display,
    pub imgui: Context,
    pub platform: WinitPlatform,
    pub renderer: Renderer,
}

impl Default for ImguiSystem {
    fn default() -> Self {
        Self::new()
    }
}

impl ImguiSystem {
    pub fn new() -> ImguiSystem {
        let event_loop = EventLoop::new();
        let cb = ContextBuilder::new().with_vsync(true);
        let wb = WindowBuilder::new().with_inner_size(LogicalSize {
                    width: WINDOW_WIDTH,
                    height: WINDOW_HEIGHT + MENU_BAR_HEIGHT + IMGUI_OFFSET,
                })
                .with_title("Songbird");
        let display = Display::new(wb, cb, &event_loop).unwrap();
        let mut imgui = Context::create();
        let mut platform = WinitPlatform::init(&mut imgui);
        // Limit scope to appease borrow checker
        {
            let gl_window = display.gl_window();
            let window = gl_window.window();
            platform.attach_window(imgui.io_mut(), window, HiDpiMode::Rounded);
        }
        let renderer = Renderer::init(&mut imgui, &display).expect("Failed to initialize renderer");

        ImguiSystem {
            event_loop,
            display,
            imgui,
            platform,
            renderer,
        }
    }

    /// ```
    /// Main event loop
    ///
    /// Passes control of emulation to the event loop, which runs forever
    /// ```
    pub fn main_loop(self, filename: Option<String>, force_dmg: bool) {
        let ImguiSystem {
            event_loop,
            display,
            mut imgui,
            mut platform,
            mut renderer
        } = self;

        let mut main_menu = MenuState::new();
        if let Some(f) = filename {
            main_menu.set_rom_filename(f);
        }
        main_menu.set_force_dmg(force_dmg);
        let mut gb = Cpu::new();
        let mut dbg = debugger::new();
        let mut curr_disp_opts = DisplayOptions::new(Palettes::GRAYSCALE, Shaders::None, force_dmg);
        let mut running = false;

        event_loop.run(move |event, _, control_flow| {
            let mut program = load_shader(&display, curr_disp_opts.shader);
            // Render 2 triangles covering whole screen
            let vertices = [
                // Top left
                Vertex{position: [-1.0, 1.0]},
                Vertex{position: [1.0, 1.0]},
                Vertex{position: [-1.0, -1.0]},

                // Bottom right
                Vertex{position: [-1.0, -1.0]},
                Vertex{position: [1.0, 1.0]},
                Vertex{position: [1.0, -1.0]},
            ];

            let vertex_buffer = VertexBuffer::new(&display, &vertices).unwrap();

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
                        } else if keycode == VirtualKeyCode::Space {
                            if state == ElementState::Pressed {
                                dbg.set_debugging(true);
                                dbg.print_info(gb.get_pc());
                            }
                        }
                },
                Event::MainEventsCleared => {
                    let gl_window = display.gl_window();
                    platform.prepare_frame(imgui.io_mut(), &gl_window.window()).unwrap();
                    gl_window.window().request_redraw();
                },
                Event::RedrawRequested(_) => {
                    // If new file has been selected in menu, load that ROM into emulator
                    if main_menu.is_load_time() {
                        let filename = main_menu.get_rom_filename();
                        setup_emu(&mut gb, filename, curr_disp_opts.force_dmg);
                        running = true;
                    }

                    let ui = imgui.frame();
                    // Always draw menu bar, regardless if running game or not
                    main_menu.create_menu(&ui);
                    main_menu.handle_file_dialog(&ui);
                    let new_opts = main_menu.handle_display_dialog(&ui);
                    if new_opts != curr_disp_opts {
                        gb.set_sys_pal(new_opts.palette);
                        program = load_shader(&display, new_opts.shader);
                        curr_disp_opts = new_opts;
                    }

                    let gl_window = display.gl_window();
                    let mut target = display.draw();
                    target.clear_color_srgb(1.0, 1.0, 1.0, 1.0);

                    // Only run emulator and draw screen if ROM has actually been selected and loaded
                    if running {
                        let filename = main_menu.get_rom_filename();
                        tick_until_draw(&mut gb, filename, &mut dbg);
                        let disp_arr = gb.render();

                        let image = RawImage2d::from_raw_rgba_reversed(&disp_arr.to_vec(), (SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32));
                        let texture = Texture2d::new(&display, image).unwrap();
                        let uniform = uniform! {
                            tex: texture.sampled()
                                .minify_filter(MinifySamplerFilter::Nearest)
                                .magnify_filter(MagnifySamplerFilter::Nearest),
                            scale: SCALE as i32
                        };

                        target.draw(
                            &vertex_buffer,
                            &NoIndices(PrimitiveType::TrianglesList),
                            &program,
                            &uniform,
                            &Default::default()
                        ).unwrap();
                    }

                    platform.prepare_render(&ui, gl_window.window());

                    let draw_data = ui.render();
                    renderer.render(&mut target, draw_data).unwrap();
                    target.finish().unwrap();
                },
                event => {
                    let gl_window = display.gl_window();
                    platform.handle_event(imgui.io_mut(), gl_window.window(), &event);
                }
            }
        });
    }
}

fn load_shader(display: &Display, shad: Shaders) -> Program {
    match shad {
        Shaders::None => {
            Program::from_source(
                display,
                include_str!("shaders/base.vert"),
                include_str!("shaders/none.frag"),
                None
            ).unwrap()
        },
        Shaders::CRT => {
            Program::from_source(
                display,
                include_str!("shaders/base.vert"),
                include_str!("shaders/crt.frag"),
                None
            ).unwrap()
        },
        Shaders::AsciiMono => {
            Program::from_source(
                display,
                include_str!("shaders/base.vert"),
                include_str!("shaders/ascii-1bit.frag"),
                None
            ).unwrap()
        },
        Shaders::AsciiColor => {
            Program::from_source(
                display,
                include_str!("shaders/base.vert"),
                include_str!("shaders/ascii-color.frag"),
                None
            ).unwrap()
        },
    }
}

/// ```
/// Tick until draw
///
/// Repeatedly runs until it is time to render a frame
///
/// Inputs:
///     Game Boy CPU (&Cpu)
///     Filename of game ROM (&str)
///     Debugger object (&debugger)
/// ```
fn tick_until_draw(mut gb: &mut Cpu, filename: &str, dbg: &mut debugger) {
    loop {
        let draw_time = gb.tick();

        dbg.check_break(gb.get_pc());
        dbg.check_watch(&gb);

        if dbg.is_debugging() {
            let is_quitting = dbg.debugloop(&mut gb);
            if is_quitting {
                exit(0);
            } else {
                break;
            }
        }

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
