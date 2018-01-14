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
use std::fmt;
extern crate rand;
use rand::Rng;

extern crate tui;

use tui::Terminal;
use tui::backend::{Backend, RawBackend};
use tui::buffer::{Buffer, Cell};
use tui::widgets::{Widget, Block, Borders, SelectableList, Paragraph};
use tui::layout::{Group, Size, Direction, Rect};

#[derive(Copy, Clone, Debug)]
enum HackerSitState {
    Sitting,
    Standing
}

#[derive(Copy, Clone, Debug)]
enum HackerMood {
    Happy,
    Sad
}

#[derive(Clone, Debug)]
struct Hacker {
    mood: HackerMood,
    hunger: u32,
    sit_state: HackerSitState,
    position: (u16, u16),
}

impl Default for Hacker {
    fn default() -> Hacker {
        Hacker {
            mood: HackerMood::Happy,
            hunger: 0,
            sit_state: HackerSitState::Sitting,
            position: (0,0)
        }
    }
}

impl Hacker {
    /// Get the character representing the mood of this hacker
    fn face(&self) -> char {
        use HackerMood::*;
        match self.mood {
            Happy => '🙂',
            Sad => '🙁',
        }
    }

    /// Get the character representing the chair (or lack thereof) of this hacker
    fn chair(&self) -> char {
        match self.sit_state {
            HackerSitState::Sitting => '💺',
            HackerSitState::Standing => '䷋',
        }
    }
}

const SPLASH_TEXT: &'static str = r#"
 _   _            _         _   _
| | | |          | |       | | | |
| |_| | __ _  ___| | ____ _| |_| |__   ___  _ __
|  _  |/ _` |/ __| |/ / _` | __| '_ \ / _ \| '_ \
| | | | (_| | (__|   < (_| | |_| | | | (_) | | | |
\_| |_/\__,_|\___|_|\_\__,_|\__|_| |_|\___/|_| |_|
             _____         _ _
            |_   _|       (_) |
              | |_ __ __ _ _| |
              | | '__/ _` | | |
              | | | | (_| | | |
              \_/_|  \__,_|_|_|

          Press [spacebar] to start
"#;

#[derive(Copy, Clone, Debug)]
enum GameStatus {
    Splash,
    Planning,
    Tables,
}

#[derive(Copy, Clone, Debug)]
enum WeatherState {
    Clear,
    Rain,
    Snow,
    Hail,
}

#[derive(Clone, Debug)]
struct GameState {
    pub hackers: Vec<Hacker>,
    pub time: u32,
    pub money: u32,
    pub weather: WeatherState,
}

impl Default for GameState {
    fn default() -> GameState {
        GameState {
            hackers: vec![],
            time: 0,
            money: 0,
            weather: WeatherState::Clear
        }
    }
}

struct Game<R> {
    state: GameState,
    status: GameStatus,
    stdin: R,
    term: Terminal<RawBackend>
}

impl<R> fmt::Debug for Game<R> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Game {{ state: {:?}, status: {:?} }}", self.state, self.status)
    }
}

impl<R: Iterator<Item=Result<Key, std::io::Error>>> Game<R> {
    fn start(&mut self) {
        loop {
            let input = self.stdin.next();
            if input.is_some() {
                use termion::event::Key::*;
                match self.status.clone() {
                    GameStatus::Splash => {
                        match input.unwrap().unwrap() {
                            Char(' ') => {
                                self.status = GameStatus::Planning;
                            }
                            Ctrl('c') => { return; }
                            _ => {}
                        };
                    },
                    GameStatus::Planning => {
                        match input.unwrap().unwrap() {
                            Ctrl('c') => { return; }
                            Esc => { self.status = GameStatus::Tables; }
                            _ => {}
                        };


                    },
                    GameStatus::Tables => {
                        match input.unwrap().unwrap() {
                            Char('s') => {
                                self.state.hackers[0].sit_state = HackerSitState::Standing;
                            },
                            Ctrl('c') => { return; }
                            _ => {}
                        };
                    }
                }
            }

            let area = self.term.size().unwrap();

            match &self.status {
                GameStatus::Splash => {
                    Paragraph::default()
                        .text(SPLASH_TEXT)
                        .render(&mut self.term, &area);
                },
                GameStatus::Planning => {
                    Paragraph::default()
                        .text("Press [esc] when you're done")
                        .render(&mut self.term, &area);
                },
                GameStatus::Tables => {
                    let mut hack_view = HackathonView {
                        hackers: &self.state.hackers
                    };
                    hack_view.render(&mut self.term, &area);
                }
            }

            self.term.draw();
            thread::sleep(Duration::from_millis(100));
        }
    }
}

/// A widget to display the hackathon table view
struct HackathonView<'a> {
    hackers: &'a Vec<Hacker>,
}

impl<'a> Widget for HackathonView<'a> {
    fn draw(&mut self, area: &Rect, buf: &mut Buffer) {
        for hacker in self.hackers {
            buf.get_mut(hacker.position.0, hacker.position.1).set_char(hacker.face());
            buf.get_mut(hacker.position.0, hacker.position.1+1).set_char(hacker.chair());
        }
    }
}

fn main() {
    let backend = RawBackend::new().unwrap();
    let mut terminal = Terminal::new(backend).unwrap();
    terminal.clear().unwrap();

    let mut game = Game {
        state: GameState::default(),
        status: GameStatus::Splash,
        stdin: termion::async_stdin().keys(),
        term: terminal
    };

    let mut hacker = Hacker::default();
    hacker.mood = HackerMood::Sad;
    hacker.position = (5, 5);
    game.state.hackers.push(hacker);

    game.start();
}
