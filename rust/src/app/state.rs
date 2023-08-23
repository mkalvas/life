use crate::app::Pattern;
use ratatui::{
    style::Color,
    widgets::canvas::{Painter, Shape},
};
use std::collections::HashSet;

type PointSet = HashSet<(i64, i64)>;

#[derive(Clone)]
pub struct State {
    pub alive: usize,
    pub generation: usize,
    points: PointSet,
    removals: PointSet,
    additions: PointSet,
}

impl State {
    pub fn new(pattern: &Pattern) -> Self {
        let mut ret = Self {
            alive: 0,
            generation: 1,
            points: HashSet::with_capacity(200),
            removals: HashSet::with_capacity(200),
            additions: HashSet::with_capacity(200),
        };
        ret.init_from_pattern(pattern);
        ret
    }

    pub fn init_from_pattern(&mut self, pattern: &Pattern) {
        let contents = pattern.grid();
        let half_height = (contents.lines().count() / 2) as i64;
        let half_width: i64 = match contents.lines().take(1).next() {
            Some(line) => (line.len() / 2) as i64,
            None => 0,
        };

        for (y, line) in contents.lines().enumerate() {
            for (x, char) in line.chars().enumerate() {
                if char == 'O' {
                    let px = x as i64 - half_width;
                    let py = -(y as i64 - half_height);
                    self.points.insert((px, py));
                    self.alive += 1;
                }
            }
        }
    }

    pub fn points(&self) -> &PointSet {
        &self.points
    }

    pub fn step(&mut self) {
        if self.alive != 0 {
            self.generate_delta();
            self.apply_delta();
            self.generation += 1;
        }
    }

    // store additions and removals separately to avoid branch misses
    fn apply_delta(&mut self) {
        for point in &self.removals {
            self.points.remove(point);
            self.alive -= 1;
        }

        for point in &self.additions {
            self.points.insert(*point);
            self.alive += 1;
        }
    }

    // only check living cells and neighbors of living cells
    // benchmarks show that keeping a list of seen points to not check twice is
    // actually slower for small enough game worlds
    fn generate_delta(&mut self) {
        self.additions.clear();
        self.removals.clear();
        for (x, y) in &self.points {
            for dx in -1..=1 {
                for dy in -1..=1 {
                    let pt = &(x + dx, y + dy);
                    let nbrs = self.count_neighbors(pt);
                    let on = self.points.contains(pt);
                    if !on && nbrs == 3 {
                        self.additions.insert(*pt);
                    } else if on && !((2..=3).contains(&nbrs)) {
                        self.removals.insert(*pt);
                    }
                }
            }
        }
    }

    // unrolled nested for loops for 3x3 square centered on cell (px, py)
    fn count_neighbors(&self, (px, py): &(i64, i64)) -> u8 {
        self.points.contains(&(px + -1, py + -1)) as u8
            + self.points.contains(&(px + -1, *py)) as u8
            + self.points.contains(&(px + -1, py + 1)) as u8
            + self.points.contains(&(*px, py + -1)) as u8
            + self.points.contains(&(*px, py + 1)) as u8
            + self.points.contains(&(px + 1, py + -1)) as u8
            + self.points.contains(&(px + 1, *py)) as u8
            + self.points.contains(&(px + 1, py + 1)) as u8
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
