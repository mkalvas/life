use crate::{app::App, ui::menu::MenuItem};
use ratatui::{
    layout::Constraint,
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Padding, Row, Table},
};

pub fn render<'a>(app: &App) -> Table<'a> {
    let enter = match app.tab {
        MenuItem::Quit => "confirm quit",
        MenuItem::Select => "select pattern",
        MenuItem::Game => {
            if app.paused {
                "unpause"
            } else {
                "pause"
            }
        }
    };

    let mut keybinds = vec![
        Row::new(vec!["␛", "pattern selector"]),
        Row::new(vec!["↵", enter]),
        Row::new(vec!["r", "restart"]),
        Row::new(vec!["d", "dot style"]),
        Row::new(vec!["q", "quit"]),
    ];

    if app.paused && app.tab == MenuItem::Game {
        keybinds.push(Row::new(vec!["→", "step once"]))
    }

    if app.tab == MenuItem::Select {
        keybinds.push(Row::new(vec!["ꜛ", "change selection"]));
        keybinds.push(Row::new(vec!["ꜜ", "change selection"]));
    }

    Table::new(keybinds)
        .column_spacing(2)
        .widths(&[Constraint::Min(3), Constraint::Min(16)])
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::White))
                .title("Keybinds")
                .border_type(BorderType::Plain)
                .padding(Padding {
                    left: 1,
                    right: 1,
                    top: 0,
                    bottom: 0,
                }),
        )
}
