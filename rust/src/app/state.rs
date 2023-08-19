use anyhow::anyhow;
use crossterm::event::{KeyCode, KeyEvent};

use crate::ui::menu::MenuItem;

pub struct App {
    pub tab: MenuItem,
    paused: bool,
}

impl App {
    pub fn new() -> Self {
        Self {
            tab: MenuItem::Game,
            paused: false,
        }
    }

    pub fn on_tick(&mut self) {}

    pub fn handle_input(&mut self, key: KeyEvent) -> anyhow::Result<()> {
        match key.code {
            KeyCode::Char('g') | KeyCode::Char('1') => self.tab = MenuItem::Game,
            KeyCode::Char('h') | KeyCode::Char('2') => self.tab = MenuItem::Help,
            KeyCode::Char('q') | KeyCode::Char('3') => match self.tab {
                MenuItem::Quit => return Err(anyhow!("quitting")),
                _ => self.tab = MenuItem::Quit,
            },
            KeyCode::Left => self.tab = MenuItem::from(usize::from(self.tab) + 3 - 1 % 3),
            KeyCode::Right => self.tab = MenuItem::from(usize::from(self.tab) + 1 % 3),
            KeyCode::Enter => match self.tab {
                MenuItem::Help => self.tab = MenuItem::Game,
                MenuItem::Game => self.paused = !self.paused,
                MenuItem::Quit => return Err(anyhow!("quitting")),
            },
            _ => {} // all other keys unbound
        };
        Ok(())
    }
}
