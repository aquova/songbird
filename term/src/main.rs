// Songbird Game Boy emulator TUI
// Austin Bricker 2019-2020

// Includes
use songbird_core::cpu::Cpu;
use songbird_core::io::Buttons;
use songbird_core::utils::{SCREEN_HEIGHT, SCREEN_WIDTH, COLOR_CHANNELS};
use termion::{async_stdin, cursor, clear, style};
use termion::color::{Bg, Fg, Rgb};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::{IntoRawMode, RawTerminal};

use std::env;
use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write};
use std::thread::sleep;
use std::time::Duration;

const FRAME_TIME: u64 = 16667; // In microseconds

struct Emu {
    gb: Cpu,
    filename: String,
}

impl Emu {
    pub fn new(filename: &str) -> Emu {
        Emu {
            gb: Cpu::new(),
            filename: filename.to_string(),
        }
    }

    pub fn init(&mut self) {
        let rom = self.load_rom();
        self.gb.load_game(&rom);
        self.load_battery_save();
    }

    pub fn run(&mut self) {
        let mut stdin = async_stdin().keys();
        let mut stdout = io::stdout().into_raw_mode().unwrap();
        let delay = Duration::from_micros(FRAME_TIME);
        let mut old_btns = Vec::new();
        let mut new_btns = Vec::new();
        write!(stdout, "{}", clear::All).unwrap();

        'gameloop: loop {
            while let Some(Ok(k)) = stdin.next() {
                if k == Key::Char('q') {
                    break 'gameloop;
                } else if let Some(btn) = key2btn(k) {
                    new_btns.push(btn);
                    if !old_btns.contains(&btn) {
                        self.gb.toggle_button(btn, true);
                    }
                }
            }

            for i in 0..old_btns.len() {
                let btn = old_btns[i];
                if !new_btns.contains(&btn) {
                    self.gb.toggle_button(btn, false);
                }
            }
            old_btns.clear();
            old_btns.append(&mut new_btns);

            self.tick_until_draw();
            let disp_arr = self.gb.render();
            self.print_frame(&mut stdout, &disp_arr);
            sleep(delay);
        }
    }

    fn print_frame(&mut self, stdout: &mut RawTerminal<io::Stdout>, arr: &[u8]) {
        // Input array is 160x144 RGBA values, where four elements together make up one pixel
        // Alpha channel is always 100%, and can be ignored
        // Each unicode character is a pair of vertical pixels
        let pixel_char = '▀';
        write!(stdout, "{}{}", style::Reset, cursor::Goto(1, 1)).unwrap();
        for y in 0..(SCREEN_HEIGHT / 2) {
            for x in 0..SCREEN_WIDTH {
                let top_index = COLOR_CHANNELS * (2 * y * SCREEN_WIDTH + x);
                let bot_index = top_index + COLOR_CHANNELS * SCREEN_WIDTH;

                // The top pixel will use the foreground color
                let top_r = arr[top_index];
                let top_g = arr[top_index + 1];
                let top_b = arr[top_index + 2];
                let fg = Fg(Rgb(top_r, top_g, top_b));

                // The bottom pixel will use the background color
                let bot_r = arr[bot_index];
                let bot_g = arr[bot_index + 1];
                let bot_b = arr[bot_index + 2];
                let bg = Bg(Rgb(bot_r, bot_g, bot_b));

                write!(stdout, "{}{}{}", fg, bg, pixel_char).unwrap();
            }
            writeln!(stdout, "{}{}", style::Reset, cursor::Goto(1, y as u16 + 1)).unwrap();
            stdout.flush().unwrap();
        }
        stdout.flush().unwrap();
    }

    fn tick_until_draw(&mut self) {
        loop {
            let draw_time = self.gb.tick();
            if draw_time {
                break;
            }
        }

        if self.gb.is_battery_dirty() {
            self.write_battery_save();
        }
    }

    fn load_rom(&mut self) -> Vec<u8> {
        let mut buffer: Vec<u8> = Vec::new();

        let mut f = File::open(&self.filename).expect("Error opening ROM");
        f.read_to_end(&mut buffer).expect("Error reading ROM to buffer");

        buffer
    }

    fn load_battery_save(&mut self) {
        if self.gb.has_battery() {
            let mut battery_ram: Vec<u8> = Vec::new();
            let mut filename = self.filename.to_owned();
            filename.push_str(".sav");

            if let Ok(mut f) = OpenOptions::new().read(true).open(filename) {
                f.read_to_end(&mut battery_ram).expect("Error reading external RAM");
                self.gb.write_ext_ram(&battery_ram);
            }
        }
    }

    fn write_battery_save(&mut self) {
        if self.gb.has_battery() {
            let ram_data = self.gb.get_ext_ram();
            let mut filename = self.filename.to_owned();
            filename.push_str(".sav");

            let mut file = OpenOptions::new().write(true).create(true).open(filename).expect("Error opening save file");
            file.write_all(ram_data).unwrap();
            file.flush().unwrap();
            self.gb.clean_battery_flag();
        }
    }
}

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() == 1 {
        println!("Usage: songbird_term path/to/game.gb");
        return;
    }

    let mut emu = Emu::new(&args[1]);
    emu.init();
    emu.run();
    print!("{}", style::Reset);
}

fn key2btn(key: Key) -> Option<Buttons> {
    match key {
        Key::Down =>            { Some(Buttons::Down)   },
        Key::Up =>              { Some(Buttons::Up)     },
        Key::Right =>           { Some(Buttons::Right)  },
        Key::Left =>            { Some(Buttons::Left)   },
        Key::Char('\n') =>      { Some(Buttons::Start)  },
        Key::Backspace =>       { Some(Buttons::Select) },
        Key::Char('x') =>       { Some(Buttons::A)      },
        Key::Char('z') =>       { Some(Buttons::B)      },
        _ =>                    { None                  }
    }
}
