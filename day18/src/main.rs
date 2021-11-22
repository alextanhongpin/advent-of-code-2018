use std::collections::HashMap;

fn main() {
    let input = include_str!("input.txt");
    let mut area = CollectionArea::new(input);
    area.draw();
    area.simulate(10);
    area.draw();
    let resources = area.resources();
    let result = resources[&'|'] * resources[&'#'];
    assert_eq!(506160, result);

    let mut area = CollectionArea::new(input);
    area.draw();
    area.simulate(1000);
    area.draw();
    let resources = area.resources();
    let result = resources[&'|'] * resources[&'#'];
    assert_eq!(0, result);
}

type Position = (i32, i32);

#[derive(Debug)]
struct CollectionArea {
    acres: HashMap<Position, char>,
}

impl CollectionArea {
    fn new(input: &str) -> CollectionArea {
        CollectionArea {
            acres: input
                .trim()
                .lines()
                .enumerate()
                .fold(HashMap::new(), |acres, (y, line)| {
                    line.chars()
                        .enumerate()
                        .fold(acres, |mut acres, (x, acre)| {
                            acres.insert((x as i32, y as i32), acre);
                            acres
                        })
                }),
        }
    }

    fn draw(&self) {
        let (_, max_x) = self.bound_x();
        let (_, max_y) = self.bound_y();

        for y in 0..=max_y {
            for x in 0..=max_x {
                print!("{}", self.acres.get(&(x, y)).unwrap_or(&'.'));
            }
            println!();
        }
        println!();
    }

    fn bound_x(&self) -> Position {
        let min_x = self.acres.keys().map(|(x, _)| x).min().unwrap();
        let max_x = self.acres.keys().map(|(x, _)| x).max().unwrap();
        (*min_x, *max_x)
    }

    fn bound_y(&self) -> Position {
        let min_y = self.acres.keys().map(|(_, y)| y).min().unwrap();
        let may_y = self.acres.keys().map(|(_, y)| y).max().unwrap();
        (*min_y, *may_y)
    }

    fn simulate(&mut self, minutes: usize) {
        let mut minutes = minutes;
        let (min_x, max_x) = self.bound_x();
        let (min_y, max_y) = self.bound_y();
        while minutes > 0 {
            let mut new_acres = self.acres.clone();
            minutes -= 1;
            for y in min_y..=max_y {
                for x in min_x..=max_x {
                    let acre = self.acres.get(&(x, y));
                    let adjacent_tiles = self.adjacent_acres(&(x, y));

                    let trees = adjacent_tiles.get(&'|').cloned().unwrap_or_default();
                    let lumberyards = adjacent_tiles.get(&'#').cloned().unwrap_or_default();

                    let new_acre = match *acre.unwrap() {
                        '.' if trees >= 3 => '|',
                        '|' if lumberyards >= 3 => '#',
                        '#' => {
                            if lumberyards >= 1 && trees >= 1 {
                                '#'
                            } else {
                                '.'
                            }
                        }
                        other => other,
                    };
                    new_acres.insert((x, y), new_acre);
                }
            }
            self.acres = new_acres;
        }
    }

    fn adjacent_acres(&self, position: &Position) -> HashMap<char, usize> {
        vec![
            (position.0 - 1, position.1),
            (position.0 - 1, position.1 - 1),
            (position.0 - 1, position.1 + 1),
            (position.0 + 1, position.1),
            (position.0 + 1, position.1 - 1),
            (position.0 + 1, position.1 + 1),
            (position.0, position.1 - 1),
            (position.0, position.1 + 1),
        ]
        .into_iter()
        .flat_map(|pos| self.acres.get(&pos))
        .fold(HashMap::new(), |mut hash, acre| {
            *hash.entry(*acre).or_insert(0) += 1;
            hash
        })
    }

    fn resources(&self) -> HashMap<char, usize> {
        self.acres.values().fold(HashMap::new(), |mut hash, acre| {
            *hash.entry(*acre).or_insert(0) += 1;
            hash
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = ".#.#...|#.
.....#|##|
.|..|...#.
..|#.....#
#.#|||#|#|
...#.||...
.|....|...
||...#|.#|
|.||||..|.
...#.|..|.";
        let mut area = CollectionArea::new(input);
        area.draw();
        area.simulate(10);
        area.draw();
        let resources = area.resources();
        let result = resources[&'|'] * resources[&'#'];
        assert_eq!(result, 1147);
    }
}
