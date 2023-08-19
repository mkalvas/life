use ratatui::{
    layout::Constraint,
    style::{Color, Modifier, Style},
    widgets::{Block, BorderType, Borders, Row, Table},
};

pub fn render<'a>() -> Table<'a> {
    Table::new(vec![
        Row::new(vec!["global", "focus game tab", "g, 1"]),
        Row::new(vec!["global", "focus help tab", "h, 2"]),
        Row::new(vec!["global", "focus quit tab", "q, 3"]),
        Row::new(vec!["global", "change tab", "left, right"]),
        Row::new(vec!["game tab (unpaused)", "pause game", "enter"]),
        Row::new(vec!["game tab (paused)", "unpause game", "enter"]),
        Row::new(vec!["game tab (paused)", "step game", "right"]),
        Row::new(vec!["help tab", "focus game tab", "enter"]),
        Row::new(vec!["quit tab", "exit app", "enter, q"]),
    ])
    .header(
        Row::new(vec!["Context", "Action", "Keys"]).style(
            Style::default()
                .fg(Color::LightGreen)
                .add_modifier(Modifier::BOLD),
        ),
    )
    .column_spacing(2)
    .widths(&[
        Constraint::Min(19),
        Constraint::Min(15),
        Constraint::Min(11),
    ])
    .block(
        Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::White))
            .title("Keybinds")
            .border_type(BorderType::Plain),
    )
}
