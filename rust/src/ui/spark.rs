use ratatui::{
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Padding, RenderDirection, Sparkline},
};

use crate::app::App;

pub fn render<'a>(app: &'a App) -> Sparkline<'a> {
    Sparkline::default()
        .data(&app.alive_history)
        .direction(RenderDirection::LeftToRight)
        .style(Style::default().fg(Color::LightGreen))
        .block(
            Block::default()
                .style(Style::default().fg(Color::White))
                .borders(Borders::LEFT | Borders::RIGHT | Borders::BOTTOM)
                .border_type(BorderType::Plain)
                .padding(Padding {
                    left: 1,
                    right: 1,
                    top: 0,
                    bottom: 0,
                }),
        )
}
