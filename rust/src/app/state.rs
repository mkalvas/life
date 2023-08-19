use std::{collections::HashSet, fs};

use ratatui::{
    style::Color,
    widgets::canvas::{Painter, Shape},
};

pub struct State {
    points: HashSet<(i64, i64)>,
}

impl State {
    pub fn new() -> Self {
        Self {
            points: HashSet::from(read_state_file()),
        }
    }
}

impl Shape for State {
    fn draw(&self, painter: &mut Painter) {
        for (x, y) in &self.points {
            if let Some((x, y)) = painter.get_point(*x as f64, *y as f64) {
                painter.paint(x, y, Color::LightGreen);
            }
        }
    }
}

fn read_state_file() -> HashSet<(i64, i64)> {
    match fs::read_to_string("../start.txt") {
        Err(_) => HashSet::new(),
        Ok(contents) => {
            let mut set = HashSet::new();
            let mut half_width: i64 = 0;
            let half_height: i64 = (contents.lines().count() / 2) as i64;

            for (y, line) in contents.lines().enumerate() {
                if half_width == 0 {
                    half_width = (line.len() / 2) as i64;
                }

                for (x, char) in line.chars().enumerate() {
                    if char == 'o' {
                        set.insert((x as i64 - half_width, y as i64 - half_height));
                    }
                }
            }
            set
        }
    }
}
