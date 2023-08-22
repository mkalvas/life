use crate::{
    app::App,
    ui::{menu::MenuItem, select::centered_rect},
};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{prelude::*, widgets::ListItem, Terminal};
use std::{
    io, panic,
    time::{Duration, Instant},
};

use super::cli::TuiOptions;

pub fn setup_panic_hook() {
    let original_hook = std::panic::take_hook();
    panic::set_hook(Box::new(move |panic| {
        restore_terminal().unwrap();
        original_hook(panic)
    }));
}

pub fn run(mut app: App, opts: TuiOptions) -> anyhow::Result<()> {
    let mut terminal = setup_terminal()?;
    let mut last_tick = Instant::now();
    loop {
        if render(&mut terminal, &mut app, opts.zoom).is_err() {
            break;
        }

        if tick(&mut app, opts.tick_rate, &mut last_tick).is_err() {
            break;
        }
    }

    terminal.show_cursor()?;
    restore_terminal()?;
    Ok(())
}

fn setup_terminal() -> anyhow::Result<Terminal<CrosstermBackend<io::Stdout>>> {
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout))?;
    terminal.hide_cursor()?;
    Ok(terminal)
}

fn restore_terminal() -> anyhow::Result<()> {
    disable_raw_mode()?;
    execute!(io::stdout(), DisableMouseCapture, LeaveAlternateScreen)?;
    Ok(())
}

fn render(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    app: &mut App,
    zoom: u8,
) -> anyhow::Result<()> {
    terminal.draw(|rect| {
        let size = rect.size();
        let sections = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(9), Constraint::Min(0)])
            .split(size);

        let infobar = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Length(25), Constraint::Min(0)])
            .split(sections[0]);

        let stats_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(4), Constraint::Length(6)])
            .split(infobar[1]);

        rect.render_widget(crate::ui::help::render(app), infobar[0]);
        rect.render_widget(crate::ui::stats::render(app), stats_layout[0]);
        rect.render_widget(crate::ui::spark::render(app), stats_layout[1]);

        match app.tab {
            MenuItem::Game => {
                rect.render_widget(
                    crate::ui::game::render(sections[1], &app.state, app.marker, zoom),
                    sections[1],
                );
            }
            MenuItem::Quit => {
                let popup_area = centered_rect(30, 40, sections[1]);
                rect.render_widget(crate::ui::game::game_block(), sections[1]);
                rect.render_widget(crate::ui::quit::render(), popup_area);
            }
            MenuItem::Select => {
                let popup_area = centered_rect(30, 40, sections[1]);
                rect.render_widget(crate::ui::game::game_block(), sections[1]);

                let items: Vec<ListItem> = app
                    .patterns
                    .get_items()
                    .iter()
                    .map(|i| ListItem::new(i.as_str()))
                    .collect();

                rect.render_stateful_widget(
                    crate::ui::select::render(items),
                    popup_area,
                    app.patterns.get_state(),
                );
            }
        };

        // rect.render_widget(crate::ui::stats::render(app), chunks[2]);
    })?;
    Ok(())
}

fn tick(app: &mut App, tick_rate: Duration, last_tick: &mut Instant) -> anyhow::Result<()> {
    let timeout = tick_rate
        .checked_sub(last_tick.elapsed())
        .unwrap_or_else(|| Duration::from_secs(0));

    if event::poll(timeout)? {
        if let Event::Key(key) = event::read()? {
            app.handle_input(key)?;
        }
    }

    if last_tick.elapsed() >= tick_rate {
        app.on_tick();
        *last_tick = Instant::now();
    }

    Ok(())
}
