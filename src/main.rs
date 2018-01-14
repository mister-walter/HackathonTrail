#![feature(match_default_bindings)]

// heavily inspired by https://github.com/redox-os/games/blob/master/src/minesweeper/main.rs
extern crate termion;

use termion::{clear, cursor, style};
use termion::raw::IntoRawMode;
use termion::input::TermRead;
use termion::event::Key;

use std::io::prelude::*;
use std::io::{self, Stdin, Stdout};
use std::{thread, time};
use std::time::{Duration, Instant};
extern crate rand;
use rand::Rng;

struct Menu {

}

trait DrawLayer {
    fn draw() -> Result<(), io::Error>;
}

enum HackerSitState {
    Sitting,
    Standing
}

enum HackerMood {
    Happy,
    Sad
}

struct Hacker {
    mood: HackerMood,
    hunger: u32,
    sit_state: HackerSitState,
    position: (u16, u16),
}

trait Widget<W: Write> {
    fn draw(&self, &mut W);
}

impl Hacker {
    pub fn default() -> Hacker {
        Hacker {
            mood: HackerMood::Happy,
            hunger: 0,
            sit_state: HackerSitState::Sitting,
            position: (0,0)
        }
    }

    fn face(&self) -> &'static str {
        use HackerMood::*;
        match self.mood {
            Happy => "🙂",
            Sad => "☹️",
        }
    }

    fn chair(&self) -> &'static str {
        match self.sit_state {
            HackerSitState::Sitting => "💺",
            HackerSitState::Standing => "䷋",
        }
    }
}

impl <W: Write> Widget<W> for Hacker {
    fn draw(&self, stdout: &mut W) {
        write!(
            stdout,
            "{}{}{}{}",
            cursor::Goto(self.position.0, self.position.1),
            self.face(),
            cursor::Goto(self.position.0, self.position.1 + 1),
            self.chair()
        ).unwrap();
    }
}

enum GameStatus {
    Splash,
    Planning,
    Tables,
}

enum WeatherState {
    Clear,
    Rain,
    Snow,
    Hail,
}

struct GameState {
    pub hackers: Vec<Hacker>,
    pub time: u32,
    pub money: u32,
    pub weather: WeatherState,
}

impl GameState {
    pub fn new() -> GameState {
        GameState {
            hackers: vec![],
            time: 0,
            money: 0,
            weather: WeatherState::Clear
        }
    }
}

struct Game<R, W: Write> {
    state: GameState,
    status: GameStatus,
    stdin: R,
    stdout: W
}

impl<R, W: Write> Drop for Game<R, W> {
    fn drop(&mut self) {
        //write!(self.stdout, "{}{}{}", clear::All, style::Reset, cursor::Goto(1, 1)).unwrap();
        write!(self.stdout, "{}{}", style::Reset, cursor::Goto(1, 1)).unwrap();
    }
}

impl<R: Iterator<Item=Result<Key, std::io::Error>>, W: Write> Game<R, W> {
    fn start(&mut self) {
        loop {
            let input = self.stdin.next();
            if input.is_some() {
                use termion::event::Key::*;
                match input.unwrap().unwrap() {
                    Char('a') => {
                        write!(self.stdout, "{}", cursor::Goto(1, 1)).unwrap();
                        write!(self.stdout, "hi").unwrap();
                    },
                    Char('\n') => {},
                    Char('s') => {
                        self.state.hackers[0].sit_state = HackerSitState::Standing;
                    },
                    Ctrl('c') => { return; }
                    _ => {}
                };
            }

            for hacker in &self.state.hackers {
                hacker.update(&mut self.state);
                hacker.draw(&mut self.stdout);
            }

            self.stdout.flush().unwrap();
            thread::sleep(Duration::from_millis(100));
        }
    }
}
//impl<R: TermRead, W: Write> Game<R, W> {
//    fn start(&mut self) {
//        let interval = 100;
//        let mut before = Instant::now();
//        loop {
//            let now = Instant::now();
//            let dt = (now.duration_since(before).subsec_nanos() / 1_000_000) as u64;
//            if dt < interval {
//                thread::sleep(Duration::from_millis(interval - dt));
//                continue;
//            }
//
//            for hacker in &self.state.hackers {
//                hacker.draw(&mut self.stdout);
//            }
//
//            self.stdout.flush().unwrap();
//            let input = self.stdin.next().unwrap().unwrap();
//            use termion::event::Key::*;
//            match &input {
//                Char('a') => {
//                    write!(self.stdout, "{}", cursor::Goto(1,1)).unwrap();
//                    write!(self.stdout, "hi").unwrap();
//                },
//                Char('\n') => {},
//                Ctrl('c') => {return;}
//                _ => {}
//            };
//        }
//    }
//}

fn main() {
    let mut stdout = std::io::stdout();
    write!(stdout, "{}", clear::All).unwrap();
    let mut game = Game {
        state: GameState::new(),
        status: GameStatus::Splash,
        //stdin: io::stdin().keys(),
        stdin: termion::async_stdin().keys(),
        stdout: io::stdout().into_raw_mode().unwrap()
    };
    let mut hacker = Hacker::default();
    hacker.position = (5, 5);
    game.state.hackers.push(hacker);

    game.start();
}
