use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash)]
struct Position(i32, i32);

impl From<&str> for Position {
    fn from(s: &str) -> Self {
        let mut iter = s.split(',');
        let x = iter.next().unwrap().trim().parse::<i32>().unwrap();
        let y = iter.next().unwrap().trim().parse::<i32>().unwrap();
        Position(x, y)
    }
}

impl Position {
    fn manhattan_distance(&self, other: &Position) -> i32 {
        (self.0 - other.0).abs() + (self.1 - other.1).abs()
    }
}

fn main() {
    let input = include_str!("input.txt");
    assert_eq!(0, part1(input));
}

fn part1(input: &str) -> i32 {
    let positions = input.trim().lines().map(Position::from).collect::<Vec<_>>();

    let max_x = positions.iter().max_by_key(|p| p.0).unwrap().0;
    let min_x = positions.iter().min_by_key(|p| p.0).unwrap().0;
    let max_y = positions.iter().max_by_key(|p| p.1).unwrap().1;
    let min_y = positions.iter().min_by_key(|p| p.1).unwrap().1;

    let mut map: HashMap<usize, i32> = HashMap::new();
    for x in min_x..max_x {
        for y in min_y..max_y {
            let dot = Position(x, y);
            let distances = positions
                .iter()
                .enumerate()
                .map(|(i, pos)| (i, dot.manhattan_distance(&pos)))
                .collect::<Vec<(usize, i32)>>();
            let &(idx, min_distance) = distances.iter().min_by_key(|t| t.1).unwrap();
            let num_occurences = distances
                .iter()
                .filter(|&(_, distance)| distance == &min_distance)
                .count();

            // If there is only one position, it's the closest to the origin.
            if num_occurences == 1 {
                *map.entry(idx).or_insert(0) += 1;
            }
        }
    }

    // To know if the point is in infinite space, just check the nearest distance from an infinite
    // point (or min/max boundary).
    for pos in &[
        Position(min_x, min_y),
        Position(max_x, min_y),
        Position(min_x, max_y),
        Position(max_x, max_y),
    ] {
        let idx = positions
            .iter()
            .enumerate()
            .min_by_key(|(_, pos2)| pos.manhattan_distance(pos2))
            .unwrap()
            .0;
        map.remove(&idx);
    }

    map.into_iter().max_by_key(|t| t.1).unwrap().1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let pos = Position::from("1, 2");
        assert_eq!(Position(1, 2), pos);
    }

    #[test]
    fn test_part1() {
        let input = "1, 1
1, 6
8, 3
3, 4
5, 5
8, 9";
        assert_eq!(17, part1(&input));
    }
}
