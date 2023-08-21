use crate::app::Pattern;
use clap::{command, Parser, ValueEnum};
use std::time::Duration;

#[derive(ValueEnum, Debug, Clone, Copy)]
enum ZoomLevel {
    Default,
    Wide,
    Wider,
}

impl From<ZoomLevel> for u8 {
    fn from(value: ZoomLevel) -> Self {
        match value {
            ZoomLevel::Default => 1,
            ZoomLevel::Wide => 2,
            ZoomLevel::Wider => 4,
        }
    }
}

/// An implementation of Conway's Game of Life in Rust
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Premade pattern to run
    #[arg(short, long, value_enum, default_value_t = Pattern::Random)]
    pattern: Pattern,

    /// Tick rate to run game at
    #[arg(short, long, default_value_t = 50)]
    tick: i64,

    /// Zoom level
    #[arg(short, long, value_enum, default_value_t = ZoomLevel::Default)]
    zoom: ZoomLevel,
}

pub struct TuiOptions {
    pub tick_rate: Duration,
    pub zoom: u8,
}

pub fn parse() -> (Pattern, TuiOptions) {
    let args = Args::parse();
    let zoom: u8 = args.zoom.into();
    let tick: u64 = match args.tick {
        rate if rate < 0 => 50,
        rate => rate as u64,
    };

    let tick_rate = Duration::from_millis(tick);
    (args.pattern, TuiOptions { tick_rate, zoom })
}
