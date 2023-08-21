use crate::app::App;
use ratatui::{
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Padding, Paragraph},
};

pub fn render<'a>(app: &App) -> Paragraph<'a> {
    let (status, color) = if app.paused {
        ("⏸  paused", Color::Red)
    } else {
        ("▶️  running", Color::LightGreen)
    };

    Paragraph::new(vec![
        Line::from(vec![Span::styled(status, Style::default().fg(color))]),
        Line::from(vec![Span::raw(format!(
            "generation: {}",
            app.state.generation
        ))]),
        Line::from(vec![Span::raw(format!("alive: {}", app.state.alive))]),
        Line::from(vec![Span::raw("1")]),
        Line::from(vec![Span::raw("2")]),
        Line::from(vec![Span::raw("3")]),
        Line::from(vec![Span::raw("4")]),
        Line::from(vec![Span::raw("5")]),
    ])
    .block(
        Block::default()
            .borders(Borders::TOP | Borders::LEFT | Borders::RIGHT)
            .style(Style::default().fg(Color::White))
            .title("Stats")
            .border_type(BorderType::Plain)
            .padding(Padding {
                left: 1,
                right: 1,
                top: 0,
                bottom: 0,
            }),
    )
}
