use ratatui::{
    layout::Alignment,
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph},
};

pub fn render<'a>() -> Paragraph<'a> {
    Paragraph::new(vec![
        Line::from(vec![Span::raw("")]),
        Line::from(vec![Span::styled(
            "Do you really want to quit?",
            Style::default().fg(Color::Red),
        )]),
        Line::from(vec![Span::raw("")]),
        Line::from(vec![Span::raw("Press 'Enter' to quit.")]),
    ])
    .alignment(Alignment::Center)
    .block(
        Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::White))
            .title("Quit")
            .border_type(BorderType::Plain),
    )
}
