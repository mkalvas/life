use clap::{command, Parser};

use super::InitPattern;

/// An implementation of Conway's Game of Life in Rust
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Premade pattern to run
    #[arg(short, long)]
    pub pattern: Option<String>,

    /// Tick rate to run game at
    #[arg(short, long)]
    pub tick: Option<i64>,
}

pub fn parse() -> (u64, InitPattern) {
    let args = Args::parse();

    let tick: u64 = match args.tick {
        None => 50,
        Some(rate) if rate < 0 => 50,
        Some(rate) => rate as u64,
    };

    let pattern = match args.pattern {
        None => InitPattern::Random,
        Some(pattern) => InitPattern::Pattern(pattern),
    };

    (tick, pattern)
}
