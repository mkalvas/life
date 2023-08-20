use ratatui::{
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph},
};

use crate::app::App;

pub fn render<'a>(app: &App) -> Paragraph<'a> {
    let action = if app.paused { "paused " } else { "running" };
    Paragraph::new(vec![Line::from(vec![Span::raw(format!(
        "{action}   generation: {}   alive: {}",
        app.state.generation, app.state.alive,
    ))])])
    .block(
        Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::White))
            .title("Stats")
            .border_type(BorderType::Plain),
    )
}
