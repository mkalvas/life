use std::time::Duration;

use clap::{command, Parser, ValueEnum};

use super::InitPattern;

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
    #[arg(short, long)]
    pattern: Option<String>,

    /// Tick rate to run game at
    #[arg(short, long)]
    tick: Option<i64>,

    /// Zoom level
    #[arg(short, long, value_enum, default_value_t = ZoomLevel::Default)]
    zoom: ZoomLevel,
}

pub struct TuiOptions {
    pub tick_rate: Duration,
    pub zoom: u8,
}

pub fn parse() -> (InitPattern, TuiOptions) {
    let args = Args::parse();

    let zoom: u8 = args.zoom.into();

    let pattern = match args.pattern {
        None => InitPattern::Random,
        Some(pattern) => InitPattern::Pattern(pattern),
    };

    let tick: u64 = match args.tick {
        None => 50,
        Some(rate) if rate < 0 => 50,
        Some(rate) => rate as u64,
    };

    let tick_rate = Duration::from_millis(tick);
    (pattern, TuiOptions { tick_rate, zoom })
}
