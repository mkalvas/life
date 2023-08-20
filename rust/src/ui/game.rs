use crate::app::State;
use ratatui::{
    prelude::Rect,
    symbols::Marker,
    widgets::{
        canvas::{Canvas, Context},
        Block, Borders,
    },
};

pub fn render(
    frame: Rect,
    state: &State,
    marker: Marker,
    zoom: u8,
) -> Canvas<impl Fn(&mut Context<'_>) + '_> {
    Canvas::default()
        .block(Block::default().borders(Borders::ALL).title("Game of Life"))
        .marker(marker)
        .paint(|ctx| {
            ctx.draw(state);
        })
        .x_bounds(bounds(frame.width, zoom))
        .y_bounds(bounds(frame.height, zoom))
}

fn bounds(span: u16, zoom: u8) -> [f64; 2] {
    let half = (span * (zoom as u16) / 2) as f64;
    // let half = (span * 2) as f64;
    [-half, if span % 2 == 0 { half } else { half + 1_f64 }]
}
