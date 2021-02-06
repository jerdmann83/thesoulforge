extern crate rustbox;
extern crate log;

use std::error::Error;

use rustbox::{Color, RustBox};
use rustbox::{InitOptions, Key};
use std::time::Duration;
use std::thread::sleep;
use log::LevelFilter;
use rand::Rng;

use std::time::{SystemTime};

fn scroll_colors(rustbox: RustBox) {
    let mut cur: u16 = 0;
    let mut done = false;
    while !done {
        for col in 0..rustbox.height() {
            let color = cur + col as u16;
            rustbox.print(
                0,
                col,
                rustbox::RB_BOLD,
                Color::Byte(color),
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
                        cur += rustbox.height() as u16;
                        break;
                    }
                    Key::Char('p') => {
                        cur -= rustbox.height() as u16;
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
                Err(e) => panic!("{}", e.description()),
                _ => {}
            }
        }
    }
}

struct HeatMap {
    grid: Vec<Vec<f32>>,
}

impl HeatMap {
    fn new(width: usize, height: usize) -> Self {
        let grid: Vec<Vec<f32>> = vec![vec![]];
        HeatMap{grid: grid}

    }
}

fn get_heat_color(heat: f32) -> u16 {
    let mut vals: Vec<u16> = vec![
        124,
        160,
        196,
        202,
        208,
        214,
        220,
        226,
        0,
        ];
    vals.reverse();

    // normalize 0.0 - 1.0 into an index into the above values 
    let mut idx  = (heat * (vals.len() as f32 + 0.05)) as usize;
    if idx >= vals.len() {
        idx = vals.len() - 1;
    }
    vals[idx]
}

// positive bump means widen the range, negative narrows
// hey, that last bit is even alliterative!
//
// TODO: really want to make this generic, but to do so I apparantly need to
// bring in the num-traits crate?  too lazy for now...
fn widen_range(range: &mut std::ops::Range<f32>, bump: f32) {
    if range.end - range.start <= bump {
        return;
    }
    range.start += bump;
    range.end   -= bump;
}

struct FireBox {
    rustbox: RustBox,
    grid: Vec<Vec<f32>>,
    heat_transfer: std::ops::Range<f32>,
    wind: f32,
}

impl FireBox {
    fn new(rustbox: RustBox) -> Self {
        FireBox{
            rustbox: rustbox,
            grid: vec![],
            heat_transfer: 0.6..0.9,
            wind: 0.0,
        }

    }

    // ridiculous rust practice exercise inspired by:
    // https://fabiensanglard.net/doom_fire_psx/
    fn run(&mut self) {
        let mut heat_source = vec![1.0; self.rustbox.width()];
        let srclen = heat_source.len();
        for i in 0..(self.rustbox.width() as f32 * 0.1) as usize {
            heat_source[i] = 0.0;
            heat_source[srclen-1-i] = 0.0;
        }
        self.grid.push(heat_source);
        for _ in 1..self.rustbox.height() {
            self.grid.push(vec![0 as f32; self.rustbox.width()]);
        }

        let mut rng = rand::thread_rng();
        loop {
            log::info!("enter");
            for y in (1..self.grid.len()).rev() {
                for x in 0..self.grid[y].len() {
                    let src_y = y-1;
                    let mut src_x : i64 = x as i64;
                    let mut seed = rng.gen_range(0.0..1.0);
                    seed += self.wind;
                    if seed < 0.1 {
                        src_x -= 1;
                    } else if seed > 0.9 {
                        src_x += 1;
                    }
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
                        " ");
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
                &" ".repeat(self.rustbox.width() - ui.len()));
            // ...and draw the (lame) ui itself
            self.rustbox.print(
                self.rustbox.width() - ui.len(),
                0,
                rustbox::RB_BOLD,
                Color::White,
                Color::Black,
                &ui);

            log::info!("spread complete");
            self.rustbox.present();

            const WIND_TICK : f32 = 0.05;
            const HEAT_XFER_TICK : f32 = 0.01;
            const TIMEOUT_MS : u64 = 30;
            let ts_start = SystemTime::now();
            let timeout = Duration::from_millis(TIMEOUT_MS);
            match self.rustbox.peek_event(timeout, false) {
                Ok(rustbox::Event::KeyEvent(key)) => match key {
                    Key::Char('q') => {
                        break;
                    }
                    Key::Char('h') => {
                        self.wind += WIND_TICK;
                    }
                    Key::Char('l') => {
                        self.wind -= WIND_TICK;
                    }
                    Key::Char('k') => {
                        widen_range(&mut self.heat_transfer, HEAT_XFER_TICK);
                    }
                    Key::Char('j') => {
                        widen_range(&mut self.heat_transfer, -HEAT_XFER_TICK);
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

fn main() {
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

    let mut fb = FireBox::new(rustbox);
    fb.run();
    // scroll_colors(rustbox)
}
