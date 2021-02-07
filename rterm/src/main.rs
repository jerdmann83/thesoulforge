extern crate clap;
extern crate log;
extern crate rustbox;

use log::LevelFilter;
use rand::Rng;
use rustbox::{Color, RustBox};
use rustbox::{InitOptions, Key};
use std::thread::sleep;
use std::time::Duration;
use std::time::SystemTime;

use clap::{App, Arg};

fn scroll_colors(rustbox: RustBox) {
    const COLOR_MAX: i32 = 255;
    let mut cur: i32 = 0;
    let mut done = false;
    while !done {
        for col in 0..rustbox.height() {
            let mut color = cur as u32 + col as u32;
            color = color % COLOR_MAX as u32;
            rustbox.print(
                0,
                col,
                rustbox::RB_BOLD,
                Color::Byte(color as u16),
                Color::Black,
                &format!("{:8}: {:0>8b}", color, color),
            );
        }
        rustbox.present();
        loop {
            match rustbox.poll_event(false) {
                Ok(rustbox::Event::KeyEvent(key)) => match key {
                    Key::Char('q') => {
                        done = true;
                        break;
                    }
                    Key::Char('n') => {
                        cur += rustbox.height() as i32;
                        break;
                    }
                    Key::Char('p') => {
                        cur -= rustbox.height() as i32;
                        break;
                    }
                    Key::Char('j') => {
                        cur += 1;
                        break;
                    }
                    Key::Char('k') => {
                        cur -= 1;
                        break;
                    }
                    _ => {}
                },
                Err(e) => panic!("{}", e.to_string()),
                _ => {}
            }
        }

        if cur < 0 {
            cur = COLOR_MAX;
        } else if cur > COLOR_MAX {
            cur = 0;
        }
    }
}

fn get_heat_color(heat: f32) -> u16 {
    let mut vals: Vec<u16> = vec![124, 160, 196, 202, 208, 214, 220, 226, 0];
    vals.reverse();

    // normalize 0.0 - 1.0 into an index into the above values
    let mut idx = (heat * (vals.len() as f32 + 0.05)) as usize;
    if idx >= vals.len() {
        idx = vals.len() - 1;
    }
    vals[idx]
}

// TODO: really want to make this generic, but to do so I apparantly need to
// bring in the num-traits crate?  Getting some headache-inducing compile errors
// so will revisit...
fn bump_range(range: &mut std::ops::Range<f32>, bump: f32) {
    range.start += bump;
    range.end += bump;
}

struct FireBox {
    rustbox: RustBox,
    grid: Vec<Vec<f32>>,
    // percent range that a given heat cell will move to another cell
    // 0.0 is no chance and >=1.0 is perfect chance, eg zero loss/decay
    heat_transfer: std::ops::Range<f32>,
    wind: f32,
}

impl FireBox {
    fn new(rustbox: RustBox) -> Self {
        FireBox {
            rustbox: rustbox,
            grid: vec![],
            heat_transfer: 0.85..0.95,
            wind: 0.0,
        }
    }

    fn make_heat_source(width: usize) -> Vec<f32> {
        let mut heat_source = vec![1.0; width];
        let srclen = heat_source.len();
        for i in 0..(width as f32 * 0.1) as usize {
            heat_source[i] = 0.0;
            heat_source[srclen - 1 - i] = 0.0;
        }
        heat_source
    }

    // 50 * 0.8

    // ridiculous rust practice exercise inspired by:
    // https://fabiensanglard.net/doom_fire_psx/
    fn run(&mut self) {
        let width = self.rustbox.width();
        let height = self.rustbox.height();
        self.grid.push(Self::make_heat_source(width));
        for _ in 1..height {
            self.grid.push(vec![0 as f32; width]);
        }

        let mut rng = rand::thread_rng();
        loop {
            log::info!("enter");
            for y in (1..self.grid.len()).rev() {
                for x in 0..self.grid[y].len() {
                    let src_y = y - 1;
                    let mut src_x: i64 = x as i64;
                    let wind = self.wind + rng.gen_range(-1.2..1.2);
                    src_x += wind as i64;
                    if src_x < 0 || src_x >= self.grid[y].len() as i64 {
                        continue;
                    }
                    let xfer = rng.gen_range(self.heat_transfer.clone());
                    self.grid[y][x] = self.grid[src_y][src_x as usize] * xfer;
                }
            }

            // draw the grid. upside-down because I apparently decided to store
            // it that way in the grid
            for y in 0..self.grid.len() {
                for x in 0..self.grid[y].len() {
                    let heat = self.grid[y][x];
                    self.rustbox.print(
                        x,
                        self.grid.len() - y,
                        rustbox::RB_BOLD,
                        Color::Byte(get_heat_color(heat)),
                        Color::Byte(get_heat_color(heat)),
                        " ",
                    );
                }
            }

            let ui = format!("xfer:{:?} wind:{:?}", self.heat_transfer, self.wind);
            // clear everything up to where the ui row begins
            self.rustbox.print(
                0,
                0,
                rustbox::RB_BOLD,
                Color::Black,
                Color::Black,
                &" ".repeat(width - ui.len()),
            );
            // ...and draw the (lame) ui itself
            self.rustbox.print(
                width - ui.len(),
                0,
                rustbox::RB_BOLD,
                Color::White,
                Color::Black,
                &ui,
            );

            log::info!("spread complete");
            self.rustbox.present();

            const WIND_TICK: f32 = 0.5;
            const HEAT_XFER_TICK: f32 = 0.01;
            const TIMEOUT_MS: u64 = 30;
            let ts_start = SystemTime::now();
            let timeout = Duration::from_millis(TIMEOUT_MS);
            match self.rustbox.peek_event(timeout, false) {
                Ok(rustbox::Event::KeyEvent(key)) => match key {
                    Key::Char('q') => {
                        break;
                    }
                    Key::Char(' ') => {
                        // space toggles the heat source
                        if self.grid[0][width / 2] > 0.0 {
                            self.grid[0] = vec![0.0; width];
                        } else {
                            self.grid[0] = Self::make_heat_source(width);
                        }
                    }
                    Key::Char('h') => {
                        self.wind += WIND_TICK;
                    }
                    Key::Char('l') => {
                        self.wind -= WIND_TICK;
                    }
                    Key::Char('k') => {
                        bump_range(&mut self.heat_transfer, HEAT_XFER_TICK);
                    }
                    Key::Char('j') => {
                        bump_range(&mut self.heat_transfer, -HEAT_XFER_TICK);
                    }
                    _ => {}
                },
                Err(e) => panic!("{}", e),
                _ => {}
            }
            log::info!("done");

            // burn off any extra time if a key was pressed above
            if let Ok(elapsed) = ts_start.elapsed() {
                if elapsed > timeout {
                    continue;
                }
                let remain = timeout - elapsed;
                if remain.as_micros() > 0 {
                    sleep(remain);
                }
            }
        }
    }
}

enum Mode {
    Colors,
    Firebox,
}

fn mode_to_string(mode: Mode) -> String {
    match mode {
        Mode::Colors => "colors".to_string(),
        Mode::Firebox => "firebox".to_string(),
    }
}

fn is_selected(matches: &clap::ArgMatches, mode: Mode) -> bool {
    matches.occurrences_of(mode_to_string(mode)) > 0
}

fn main() {
    let matches = App::new("rterm")
        .version("1.0")
        .about("various terminal practice things of dubious usefulness")
        .arg(
            Arg::with_name("firebox")
                .long("firebox")
                .help("pretty fire simulation thing"),
        )
        .arg(
            Arg::with_name("colors")
                .long("colors")
                .help("scrollable colors viewer"),
        )
        .get_matches();

    let mut modes = 0;
    if is_selected(&matches, Mode::Firebox) {
        modes += 1;
    }
    if is_selected(&matches, Mode::Colors) {
        modes += 1;
    }
    if modes != 1 {
        panic!("must select exactly one mode");
    }

    simple_logging::log_to_file("/tmp/rustbox", LevelFilter::Info).unwrap();

    let opt = InitOptions {
        input_mode: rustbox::InputMode::EscMouse,
        output_mode: rustbox::OutputMode::EightBit,
        buffer_stderr: false,
    };
    let rustbox = match RustBox::init(opt) {
        Result::Ok(v) => v,
        Result::Err(e) => panic!("{}", e),
    };

    if is_selected(&matches, Mode::Firebox) {
        let mut fb = FireBox::new(rustbox);
        fb.run();
    } else if is_selected(&matches, Mode::Colors) {
        scroll_colors(rustbox)
    }
}
