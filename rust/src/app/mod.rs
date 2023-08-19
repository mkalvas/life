use crate::ui::menu::MenuItem;
use anyhow::anyhow;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::symbols::Marker;

mod state;
mod tui;

pub mod cli;
pub use state::{InitPattern, State};
pub use tui::{run, setup_panic_hook};

pub struct App {
    pub tab: MenuItem,
    pub state: State,
    pub marker: Marker,
    pub paused: bool,
}

impl App {
    pub fn new(pattern: InitPattern) -> Self {
        Self {
            paused: true,
            state: State::new(pattern),
            tab: MenuItem::Game,
            marker: Marker::Bar,
        }
    }

    pub fn on_tick(&mut self) {
        if !self.paused {
            self.state.step();
        }
    }

    pub fn handle_input(&mut self, key: KeyEvent) -> anyhow::Result<()> {
        match key.code {
            KeyCode::Char('g') | KeyCode::Char('1') => self.goto(MenuItem::Game),
            KeyCode::Char('h') | KeyCode::Char('2') => self.goto(MenuItem::Help),
            KeyCode::Char('q') | KeyCode::Char('3') => match self.tab {
                MenuItem::Quit => return Err(anyhow!("quitting")),
                _ => self.goto(MenuItem::Quit),
            },
            KeyCode::Char('m') => {
                if self.tab == MenuItem::Game {
                    self.marker = match self.marker {
                        Marker::Bar => Marker::Block,
                        Marker::Block => Marker::Braille,
                        Marker::Braille => Marker::Dot,
                        Marker::Dot => Marker::Bar,
                    };
                }
            }
            KeyCode::Left => self.goto(MenuItem::from(usize::from(self.tab) + 2)), // enums auto modulo!
            KeyCode::Right => {
                if self.tab == MenuItem::Game && self.paused {
                    self.state.step();
                } else {
                    self.goto(MenuItem::from(usize::from(self.tab) + 1))
                }
            }
            KeyCode::Enter => match self.tab {
                MenuItem::Help => self.goto(MenuItem::Game),
                MenuItem::Game => self.paused = !self.paused,
                MenuItem::Quit => return Err(anyhow!("quitting")),
            },
            KeyCode::Char('c') => {
                if key.modifiers == KeyModifiers::CONTROL {
                    return Err(anyhow!("quitting"));
                }
            }
            _ => {} // all other keys unbound
        };
        Ok(())
    }

    fn goto(&mut self, tab: MenuItem) {
        self.tab = tab;
        self.paused = tab != MenuItem::Game;
    }
}
