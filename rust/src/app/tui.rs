use crate::{app::App, ui::menu::MenuItem};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{prelude::*, Terminal};
use std::{
    io, panic,
    time::{Duration, Instant},
};

pub fn setup_panic_hook() {
    let original_hook = std::panic::take_hook();
    panic::set_hook(Box::new(move |panic| {
        restore_terminal().unwrap();
        original_hook(panic)
    }));
}

pub fn run(mut app: App, tick_rate: u64) -> anyhow::Result<()> {
    let mut terminal = setup_terminal()?;

    let tick_rate_duration = Duration::from_millis(tick_rate);
    let mut last_tick = Instant::now();
    loop {
        if render(&mut terminal, &app).is_err() {
            break;
        }

        if tick(&mut app, tick_rate_duration, &mut last_tick).is_err() {
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

fn render(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>, app: &App) -> anyhow::Result<()> {
    terminal.draw(|rect| {
        let size = rect.size();
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Length(3),
                    Constraint::Min(2),
                    Constraint::Length(3),
                ]
                .as_ref(),
            )
            .split(size);

        // Draw screen
        rect.render_widget(crate::ui::menu::render(app), chunks[0]);

        match app.tab {
            MenuItem::Quit => rect.render_widget(crate::ui::quit::render(), chunks[1]),
            MenuItem::Help => rect.render_widget(crate::ui::help::render(), chunks[1]),
            MenuItem::Game => rect.render_widget(
                crate::ui::game::render(chunks[1], &app.state, app.marker),
                chunks[1],
            ),
        };

        rect.render_widget(crate::ui::stats::render(app), chunks[2]);
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
