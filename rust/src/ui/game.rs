use crate::app::State;
use ratatui::{
    prelude::Rect,
    symbols::Marker,
    widgets::{
        canvas::{Canvas, Context},
        Block, Borders,
    },
};

pub fn render<'a>(
    frame: Rect,
    state: &State,
    marker: Marker,
) -> Canvas<impl Fn(&mut Context<'_>) + '_> {
    Canvas::default()
        .block(Block::default().borders(Borders::ALL).title("Game of Life"))
        .marker(marker)
        .paint(|ctx| {
            ctx.draw(state);
        })
        .x_bounds(bounds(frame.width))
        .y_bounds(bounds(frame.height))
}

fn bounds(span: u16) -> [f64; 2] {
    let half = (span / 2) as f64;
    [-half, if span % 2 == 0 { half } else { half + 1 as f64 }]
}
