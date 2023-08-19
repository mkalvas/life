use std::{collections::HashSet, fs};

use ratatui::{
    style::Color,
    widgets::canvas::{Painter, Shape},
};

pub enum InitPattern {
    Pattern(String),
    Random,
}

pub struct State {
    pub alive: usize,
    pub generation: usize,
    pub points: HashSet<(i64, i64)>,
    pub delta: HashSet<(i64, i64)>,
    pub seen: HashSet<(i64, i64)>,
    stabilized: bool,
    stable_gens: u8,
    xs: [i64; 2],
    ys: [i64; 2],
}

impl State {
    pub fn new(pattern: InitPattern) -> Self {
        let mut ret = Self {
            xs: [0, 0],
            ys: [0, 0],
            stabilized: false,
            stable_gens: 0,
            alive: 0,
            generation: 1,
            delta: HashSet::new(),
            seen: HashSet::new(),
            points: HashSet::new(),
        };

        match pattern {
            InitPattern::Random => ret.read_state_file("random".to_string()),
            InitPattern::Pattern(pattern) => ret.read_state_file(pattern),
        }

        ret
    }

    fn read_state_file(&mut self, pattern: String) {
        match fs::read_to_string(format!("../patterns/{pattern}.txt")) {
            Err(_) => (),
            Ok(contents) => {
                let mut set = HashSet::new();
                let mut half_width: i64 = 0;
                let half_height: i64 = (contents.lines().count() / 2) as i64;

                for (y, line) in contents.lines().enumerate() {
                    if half_width == 0 {
                        half_width = (line.len() / 2) as i64;
                    }

                    for (x, char) in line.chars().enumerate() {
                        if char == 'O' {
                            let px = x as i64 - half_width;
                            let py = -(y as i64 - half_height);

                            if px < self.xs[0] {
                                self.xs[0] = px
                            } else if px > self.xs[1] {
                                self.xs[1] = px
                            };

                            if py < self.ys[0] {
                                self.ys[0] = py
                            } else if py > self.ys[1] {
                                self.ys[1] = py
                            };

                            set.insert((px, py));
                            self.alive += 1;
                        }
                    }
                }
                self.points = set;
            }
        }
    }

    pub fn step(&mut self) {
        if !self.stabilized {
            self.generate_delta();
            self.apply_delta();
            if !self.stabilized {
                self.generation += 1;
            }
        }
    }

    fn apply_delta(&mut self) {
        let prev_alive = self.alive;
        for point in &self.delta {
            if self.points.contains(point) {
                self.points.remove(point);
                self.alive -= 1;
            } else {
                self.points.insert(*point);
                self.alive += 1;
            }
        }

        // extinction is permanently stable
        if self.alive == 0 {
            self.stabilized = true;
        }

        if self.alive == prev_alive {
            self.stable_gens += 1;
            // arbitrary cutoff, could be false too if we have gliders on
            // collision paths with other living cells or gliders
            self.stabilized = self.stable_gens > 5;
        } else {
            self.stable_gens = 0;
        }
    }

    fn generate_delta(&mut self) {
        self.delta = HashSet::new();
        self.seen = HashSet::new();
        // only check on points and neighbors of on points
        for (x, y) in &self.points {
            for dx in -1..=1 {
                for dy in -1..=1 {
                    let pt = &(x + dx, y + dy);
                    if self.seen.contains(pt) {
                        continue;
                    }

                    let nbrs = self.count_neighbors(pt);

                    let on = self.points.contains(pt);
                    if (!on && nbrs == 3) || (on && !((2..=3).contains(&nbrs))) {
                        self.delta.insert(*pt);
                    }
                }
            }
        }
    }

    fn count_neighbors(&self, (px, py): &(i64, i64)) -> u8 {
        let mut nbrs = 0;
        for dx in -1..=1 {
            for dy in -1..=1 {
                if !(dx == 0 && dy == 0) {
                    nbrs += self.points.contains(&(px + dx, py + dy)) as u8;
                }
            }
        }
        nbrs
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
