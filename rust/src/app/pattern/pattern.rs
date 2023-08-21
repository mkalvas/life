use clap::ValueEnum;

use super::data;

#[derive(ValueEnum, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Pattern {
    Random,
    SimpleExtinct,
    SimpleStable,
    GosperGlider,
    RPentomino,
    Max,
    PufferShip,
    Spider,
    Hotcrystal0,
    P76PiPentominoHassler,
}

impl Pattern {
    pub fn as_str(&self) -> &'static str {
        match self {
            Pattern::Random => "Random",
            Pattern::SimpleExtinct => "Simple Extinct",
            Pattern::SimpleStable => "Simple Stable",
            Pattern::GosperGlider => "Gosper Glider",
            Pattern::RPentomino => "R-pentomino",
            Pattern::Max => "Max",
            Pattern::PufferShip => "Puffer Ship",
            Pattern::Spider => "Spider",
            Pattern::Hotcrystal0 => "Hotcrystal0",
            Pattern::P76PiPentominoHassler => "p76 pi-heptomino hassler",
        }
    }

    pub fn grid(&self) -> String {
        match self {
            Pattern::Random => data::random(),
            Pattern::SimpleExtinct => data::SIMPLE_EXTINCT.to_string(),
            Pattern::SimpleStable => data::SIMPLE_STABLE.to_string(),
            Pattern::GosperGlider => data::GOSPER_GLIDER.to_string(),
            Pattern::RPentomino => data::R_PENTOMINO.to_string(),
            Pattern::Max => data::MAX.to_string(),
            Pattern::PufferShip => data::PUFFER_SHIP.to_string(),
            Pattern::Spider => data::SPIDER.to_string(),
            Pattern::Hotcrystal0 => data::HOTCRYSTAL_0.to_string(),
            Pattern::P76PiPentominoHassler => data::P76_PI_PENTOMINO_HASSLER.to_string(),
        }
    }

    pub fn all() -> Vec<Pattern> {
        vec![
            Pattern::Random,
            Pattern::SimpleExtinct,
            Pattern::SimpleStable,
            Pattern::GosperGlider,
            Pattern::RPentomino,
            Pattern::Max,
            Pattern::PufferShip,
            Pattern::Spider,
            Pattern::Hotcrystal0,
            Pattern::P76PiPentominoHassler,
        ]
    }
}

impl TryFrom<String> for Pattern {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Pattern, Self::Error> {
        for p in Pattern::all().into_iter() {
            if p.as_str() == value {
                return Ok(p);
            }
        }
        Err("unknown pattern")
    }
}
