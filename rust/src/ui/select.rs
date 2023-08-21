use ratatui::{
    prelude::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph},
};

use crate::app::InitPattern;

/// helper function to create a centered rect using up certain percentage of the available rect `r`
pub fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_y) / 2),
            ]
            .as_ref(),
        )
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ]
            .as_ref(),
        )
        .split(popup_layout[1])[1]
}

pub fn render<'a>() -> Paragraph<'a> {
    let patterns: Vec<&str> = InitPattern::all().into_iter().map(|p| p.1).collect();
    Paragraph::new(vec![
        Line::from(vec![Span::raw(patterns.join(","))]),
        Line::from(vec![Span::styled(
            "Do you really want to select?",
            Style::default().fg(Color::Red),
        )]),
        Line::from(vec![Span::raw("")]),
        Line::from(vec![Span::raw("Press 'Enter' to quit.")]),
    ])
    .alignment(Alignment::Center)
    .block(
        Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::White).bg(Color::Reset))
            .title("Select Pattern")
            .border_type(BorderType::Plain),
    )
}
