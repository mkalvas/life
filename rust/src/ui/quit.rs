use ratatui::{
    layout::Alignment,
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Padding, Paragraph},
};

pub fn render<'a>() -> Paragraph<'a> {
    Paragraph::new(vec![
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
            .title("Quit")
            .borders(Borders::ALL)
            .border_type(BorderType::Plain)
            .style(Style::default().fg(Color::White))
            .padding(Padding {
                top: 2,
                bottom: 2,
                left: 1,
                right: 1,
            }),
    )
}
