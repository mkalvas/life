use crate::{app::state::App, ui::menu::MenuItem};
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

pub fn run(mut app: App) -> anyhow::Result<()> {
    let mut terminal = setup_terminal()?;

    let tick_rate = Duration::from_millis(10);
    let mut last_tick = Instant::now();
    loop {
        if render(&mut terminal, &mut app).is_err() {
            break;
        }

        if tick(&mut app, tick_rate, &mut last_tick).is_err() {
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

// fn lines(strings: &Vec<String>) -> Vec<Line> {
//     let strings = strings.clone();
//     let mut lines: Vec<Line> = Vec::new();
//     for s in strings.into_iter() {
//         lines.push(Line::from(s))
//     }
//     lines
// }

fn render(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>, app: &App) -> anyhow::Result<()> {
    terminal.draw(|rect| {
        let size = rect.size();
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(3), Constraint::Min(2)].as_ref())
            .split(size);

        // Draw screen
        rect.render_widget(crate::ui::menu::render(app), chunks[0]);

        match app.tab {
            MenuItem::Quit => rect.render_widget(crate::ui::quit::render(), chunks[1]),
            MenuItem::Help => rect.render_widget(crate::ui::help::render(), chunks[1]),
            MenuItem::Game => rect.render_widget(crate::ui::quit::render(), chunks[1]),
        };
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
