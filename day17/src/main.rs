use std::collections::HashSet;

type Position = (i32, i32);

fn main() {
    let input = include_str!("input.txt");
    let mut game = Game::new(&input);
    game.simulate();
    game.draw();

    assert_eq!(0, game.drops.union(&game.water).count());
}

#[derive(Debug)]
struct Game {
    tiles: HashSet<Position>,
    drops: HashSet<Position>,
    water: HashSet<Position>,
    last_drops: Vec<Position>,
}

impl Game {
    fn new(input: &str) -> Self {
        let mut tiles = HashSet::new();
        for line in input.trim().lines() {
            let mut parts = line.trim().split(',').map(str::trim).collect::<Vec<&str>>();
            parts.sort();
            let x_part = parts.iter().next().unwrap().split('=').last().unwrap();
            let y_part = parts.iter().last().unwrap().split('=').last().unwrap();

            if x_part.contains("..") {
                let x_range = x_part
                    .split("..")
                    .map(|s| s.parse::<i32>().unwrap())
                    .collect::<Vec<i32>>();
                let x_start = x_range[0];
                let x_end = x_range[1];
                let y = y_part.parse::<i32>().unwrap();
                for x in x_start..=x_end {
                    tiles.insert((x, y));
                }
                continue;
            }
            let x = x_part.parse::<i32>().unwrap();
            if y_part.contains("..") {
                let y_range = y_part
                    .split("..")
                    .map(|s| s.parse::<i32>().unwrap())
                    .collect::<Vec<i32>>();
                let y_start = y_range[0];
                let y_end = y_range[1];
                for y in y_start..=y_end {
                    tiles.insert((x, y));
                }
                continue;
            }
            let y = y_part.parse::<i32>().unwrap();
            tiles.insert((x, y));
        }

        Game {
            tiles: tiles,
            drops: HashSet::new(),
            water: HashSet::new(),
            last_drops: vec![(500, 0)],
        }
    }

    fn draw(&self) {
        let min_x = self.tiles.iter().map(|(x, _)| *x).min().unwrap();
        let max_x = self.tiles.iter().map(|(x, _)| *x).max().unwrap();
        let max_y = self.tiles.iter().map(|(_, y)| *y).max().unwrap();
        for y in 0..=max_y {
            for x in min_x..=max_x {
                if x == 500 && y == 0 {
                    print!("+");
                } else if self.tiles.contains(&(x, y)) {
                    print!("#");
                } else if self.water.contains(&(x, y)) {
                    print!("~");
                } else if self.drops.contains(&(x, y)) {
                    print!("|");
                } else {
                    print!(".");
                }
            }
            println!();
        }
        println!();
    }

    fn simulate(&mut self) {
        let max_y = self.tiles.iter().map(|(_, y)| *y).max().unwrap();
        while !self.last_drops.is_empty() {
            let drop = self.last_drops.pop().unwrap();
            if drop.1 > max_y {
                self.drops.remove(&drop);
                continue;
            }
            if self.can_drip(&drop) {
                self.drip(&drop);
                continue;
            }
            if self.can_fill(&drop) {
                self.fill(&drop);
            } else {
                self.overflow(&drop);
            }
        }
    }

    fn can_drip(&self, drop: &Position) -> bool {
        let next_drop = (drop.0, drop.1 + 1);
        if self.tiles.contains(&next_drop) || self.water.contains(&next_drop) {
            false
        } else {
            true
        }
    }

    fn drip(&mut self, drop: &Position) {
        let next_drop = (drop.0, drop.1 + 1);
        self.drops.insert(next_drop);
        self.last_drops.push(next_drop);
    }

    fn can_fill(&mut self, drop: &Position) -> bool {
        let mut left = drop.clone();
        loop {
            left.0 -= 1;
            if self.tiles.contains(&left) {
                break;
            }
            let bottom = (left.0, left.1 + 1);
            if !(self.tiles.contains(&bottom) || self.water.contains(&bottom)) {
                return false;
            }
        }
        let mut right = drop.clone();
        loop {
            right.0 += 1;
            if self.tiles.contains(&right) {
                return true;
            }
            let bottom = (right.0, right.1 + 1);
            if !(self.tiles.contains(&bottom) || self.water.contains(&bottom)) {
                return false;
            }
        }
    }

    fn fill(&mut self, drop: &Position) {
        self.last_drops.push((drop.0, drop.1 - 1));
        self.water.insert(drop.clone());
        let mut left = drop.clone();
        loop {
            left.0 -= 1;
            if self.tiles.contains(&left) {
                break;
            }
            self.water.insert(left);
        }
        let mut right = drop.clone();
        loop {
            right.0 += 1;
            if self.tiles.contains(&right) {
                break;
            }
            self.water.insert(right);
        }
    }

    fn overflow(&mut self, drop: &Position) {
        let mut left = drop.clone();
        self.drops.insert(drop.clone());
        loop {
            left.0 -= 1;
            if self.tiles.contains(&left) {
                break;
            }
            self.drops.insert(left);
            let bottom = (left.0, left.1 + 1);
            if !(self.tiles.contains(&bottom) || self.water.contains(&bottom)) {
                self.last_drops.push(left);
                break;
            }
        }

        let mut right = drop.clone();
        loop {
            right.0 += 1;
            if self.tiles.contains(&right) {
                break;
            }
            self.drops.insert(right);
            let bottom = (right.0, right.1 + 1);
            if !(self.tiles.contains(&bottom) || self.water.contains(&bottom)) {
                self.last_drops.push(right);
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "x=495, y=2..7
y=7, x=495..501
x=501, y=3..7
x=498, y=2..4
x=506, y=1..2
x=498, y=10..13
x=504, y=10..13
y=13, x=498..504";

        let mut game = Game::new(input);
        game.simulate();
        game.draw();
        assert_eq!(57, game.drops.union(&game.water).count());
    }
}
