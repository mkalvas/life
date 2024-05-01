use crate::ui::menu::MenuItem;
use anyhow::anyhow;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::symbols::Marker;
use slice_deque::SliceDeque;
use std::ops::Deref;

mod patterns;
mod state;
mod tui;

pub mod cli;
pub use patterns::{Pattern, PatternList};
pub use state::State;
pub use tui::{run, setup_panic_hook};

pub struct App {
    pub tab: MenuItem,
    pub state: State,
    pub marker: Marker,
    pub paused: bool,
    pub alive_history: FixedVec<u64>,
    pub patterns: PatternList,
}

impl App {
    pub fn new(pattern: &Pattern) -> Self {
        Self {
            paused: true,
            state: State::new(pattern),
            tab: MenuItem::Game,
            marker: Marker::Bar,
            alive_history: FixedVec::new(500),
            patterns: PatternList::new(pattern),
        }
    }

    pub fn step(&mut self) {
        self.state.step();
        self.alive_history.prepend(self.state.alive as u64);
    }

    pub fn on_tick(&mut self) {
        if !self.paused {
            self.step();
        }
    }

    pub fn handle_input(&mut self, key: KeyEvent) -> anyhow::Result<()> {
        match key.code {
            KeyCode::Char('q') => match self.tab {
                MenuItem::Quit => return Err(anyhow!("quitting")),
                _ => self.goto(MenuItem::Quit),
            },
            KeyCode::Char('d') => {
                if self.tab == MenuItem::Game {
                    self.marker = match self.marker {
                        Marker::Bar => Marker::Block,
                        Marker::Block => Marker::Braille,
                        Marker::Braille => Marker::Dot,
                        Marker::Dot => Marker::Bar,
                    };
                }
            }
            KeyCode::Right => {
                if self.tab == MenuItem::Game && self.paused {
                    self.step();
                }
            }
            KeyCode::Char('r') => {
                self.paused = true;
                self.state = State::new(&self.patterns.selected());
            }
            KeyCode::Up | KeyCode::Char('k') => {
                if self.tab == MenuItem::Select {
                    self.patterns.prev()
                }
            }
            KeyCode::Down | KeyCode::Char('j') => {
                if self.tab == MenuItem::Select {
                    self.patterns.next()
                }
            }
            KeyCode::Esc => match self.tab {
                MenuItem::Quit => self.goto(MenuItem::Game),
                MenuItem::Select => self.goto(MenuItem::Game),
                MenuItem::Game => self.goto(MenuItem::Select),
            },
            KeyCode::Enter => match self.tab {
                MenuItem::Game => self.paused = !self.paused,
                MenuItem::Quit => return Err(anyhow!("quitting")),
                MenuItem::Select => {
                    self.state = State::new(&self.patterns.selected());
                    self.tab = MenuItem::Game // don't unpause right away
                }
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

pub struct FixedVec<T> {
    capacity: usize,
    inner: SliceDeque<T>,
}

impl<T> FixedVec<T> {
    pub fn new(capacity: usize) -> Self {
        Self {
            capacity,
            inner: SliceDeque::with_capacity(capacity),
        }
    }

    pub fn prepend(&mut self, value: T) {
        self.inner.push_front(value);
        if self.inner.len() > self.capacity {
            self.inner.pop_back();
        }
    }
}

impl<T> Deref for FixedVec<T> {
    type Target = [T];
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
