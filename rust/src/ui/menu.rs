use crate::app::state::App;
use ratatui::{
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Tabs},
};

#[derive(Copy, Clone, Debug)]
pub enum MenuItem {
    Quit,
    Help,
    Game,
}

impl From<usize> for MenuItem {
    fn from(input: usize) -> MenuItem {
        match input {
            0 => MenuItem::Game,
            1 => MenuItem::Help,
            2 => MenuItem::Quit,
            3 => MenuItem::Game,
            _ => MenuItem::Help,
        }
    }
}

impl From<MenuItem> for usize {
    fn from(input: MenuItem) -> usize {
        match input {
            MenuItem::Game => 0,
            MenuItem::Help => 1,
            MenuItem::Quit => 2,
        }
    }
}

const MENU_TITLES: &[&str] = &["Game", "Help", "Quit"];

pub fn render<'a>(app: &App) -> Tabs<'a> {
    let menu = MENU_TITLES
        .iter()
        .map(|t| {
            let (first, rest) = t.split_at(1);
            Line::from(vec![
                Span::styled(
                    first,
                    Style::default()
                        .fg(Color::LightGreen)
                        .add_modifier(Modifier::UNDERLINED),
                ),
                Span::styled(rest, Style::default().fg(Color::White)),
            ])
        })
        .collect();

    Tabs::new(menu)
        .select(app.tab.into())
        .block(Block::default().title("Menu").borders(Borders::ALL))
        .style(Style::default().fg(Color::White))
        .highlight_style(
            Style::default()
                .fg(Color::LightGreen)
                .add_modifier(Modifier::BOLD),
        )
        .divider(Span::raw("|"))
}
