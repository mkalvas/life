use super::pattern::Pattern;
use ratatui::widgets::ListState;

#[derive(Clone)]
pub struct PatternList {
    pub items: Vec<Pattern>,
    pub state: ListState,
}

impl PatternList {
    pub fn new(pattern: &Pattern) -> Self {
        let items = Pattern::all();
        let mut state = ListState::default();

        for (i, p) in items.iter().enumerate() {
            if pattern == p {
                state.select(Some(i));
            }
        }

        Self { items, state }
    }

    pub fn get_items(&self) -> &Vec<Pattern> {
        &self.items
    }

    pub fn get_state(&mut self) -> &mut ListState {
        &mut self.state
    }

    pub fn select(&mut self, i: usize) {
        self.state.select(Some(i % self.items.len()));
    }

    pub fn selected(&self) -> Pattern {
        match self.state.selected() {
            Some(i) => self.items[i],
            None => self.items[0],
        }
    }

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => (i + 1) % self.items.len(),
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn prev(&mut self) {
        let i = match self.state.selected() {
            Some(i) => (i + self.items.len() - 1) % self.items.len(),
            None => 0,
        };
        self.state.select(Some(i));
    }
}
