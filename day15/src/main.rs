use std::collections::{HashMap, HashSet};
type Position = (i32, i32);

fn main() {
    let input = include_str!("input.txt");
    assert_eq!(215168, Grid::new(&input).play());
    assert_eq!(52374, play_for_elf(&input));
}

fn play_for_elf(input: &str) -> i32 {
    let mut elf_power = 4;
    loop {
        match Grid::new_with_elf_power(&input, elf_power).play_for_elf() {
            Some(score) => return score,
            None => elf_power += 1,
        }
    }
}

#[derive(Debug)]
struct Grid {
    players: Vec<(Position, char, i32)>,
    walls: HashSet<Position>,
    round: i32,
    elf_power: i32,
}

impl Grid {
    fn new(input: &str) -> Self {
        let tiles = parse(&input);
        let players = tiles
            .iter()
            .filter(|&(_, ch)| *ch != '#')
            .map(|&(pos, ch)| (pos, ch, 200))
            .collect::<Vec<(Position, char, i32)>>();

        let walls: HashSet<Position> = HashSet::from_iter(
            tiles
                .iter()
                .filter(|&(_, ch)| *ch == '#')
                .map(|&(pos, _)| pos),
        );
        Grid {
            players: players,
            walls: walls,
            round: 0,
            elf_power: 3,
        }
    }

    fn new_with_elf_power(input: &str, elf_power: i32) -> Self {
        let mut grid = Grid::new(input);
        grid.elf_power = elf_power;
        grid
    }

    fn play_for_elf(&mut self) -> Option<i32> {
        let num_elves = self.players.iter().filter(|&&(_, ch, _)| ch == 'E').count();
        loop {
            match self.round() {
                Some(winner) => {
                    let remaining_elves = self
                        .players
                        .iter()
                        .filter(|&&(_, ch, health)| ch == 'E' && health > 0)
                        .count();
                    if remaining_elves != num_elves {
                        return None;
                    }
                    println!(
                        "wins at round: {} with HP {} with attack power {}",
                        self.round,
                        self.total_hit_points(),
                        self.elf_power
                    );
                    return Some(self.round * self.total_hit_points());
                }
                None => (),
            }
        }
    }

    fn play(&mut self) -> i32 {
        loop {
            match self.round() {
                Some(_) => {
                    println!(
                        "wins at round: {} with HP {}",
                        self.round,
                        self.total_hit_points(),
                    );
                    return self.round * self.total_hit_points();
                }
                None => (),
            }
        }
    }

    fn total_hit_points(&self) -> i32 {
        self.players
            .iter()
            .map(|(_, _, health)| health)
            .filter(|health| *health > &0)
            .sum::<i32>()
    }

    fn round(&mut self) -> Option<char> {
        self.players = self.filter_alive(&self.players);
        self.sort_by_read_order();

        for i in 0..self.players.len() {
            let (pos, player, health) = self.players[i];
            // Skip if player is dead during the previous turn.
            if health <= 0 {
                continue;
            }
            let alive = self.filter_alive(&self.players);
            let obstacles = self.build_obstacles(&alive);
            let enemies = self.find_targets(&alive, player);
            if enemies.is_empty() {
                return Some(player);
            }

            // Attack.
            match enemies
                .clone()
                .into_iter()
                .filter(|enemy| manhattan_distance(&pos, &enemy) == 1)
                .min_by_key(|enemy| {
                    (
                        self.find_player_health_at_position(&alive, enemy),
                        enemy.1,
                        enemy.0,
                    )
                }) {
                Some(enemy) => {
                    let j = self
                        .players
                        .iter()
                        .into_iter()
                        .position(|&(pos, _, _)| pos == enemy)
                        .unwrap();
                    self.players[j].2 -= self.get_attack(player);
                    if player == 'G' && self.players[j].2 <= 0 && self.elf_power > 3 {
                        return Some('G');
                    }
                    continue;
                }
                None => {}
            }

            let possible_moves = enemies
                .clone()
                .into_iter()
                .flat_map(|enemy| {
                    // Filter the best in-range move.
                    let in_range = self.find_in_range(&obstacles, &enemy);
                    in_range
                        .into_iter()
                        .flat_map(|in_range_pos| {
                            match self.find_shortest_distance(&obstacles, &pos, &in_range_pos) {
                                Some(dist) => Some((dist, enemy, in_range_pos)),
                                None => None,
                            }
                        })
                        .min_by_key(|&(dist, _, pos)| (dist, pos.1, pos.0))
                })
                .min_by_key(|&(dist, origin, _)| (dist, origin.1, origin.0));

            // Move.
            match possible_moves {
                Some((_, _, tgt_pos)) => {
                    let in_range = self.find_in_range(&obstacles, &pos);
                    let best_move = in_range
                        .into_iter()
                        .flat_map(|src_pos| {
                            match self.find_shortest_distance(&obstacles, &src_pos, &tgt_pos) {
                                Some(dist) => Some((dist, src_pos)),
                                None => None,
                            }
                        })
                        .min_by_key(|&(dist, pos)| (dist, pos.1, pos.0));
                    match best_move {
                        Some((_, new_move)) => {
                            self.players[i].0 = new_move;
                        }
                        _ => {}
                    }
                }
                None => {}
            }

            // Attack.
            match enemies
                .into_iter()
                .filter(|enemy| manhattan_distance(&self.players[i].0, &enemy) == 1)
                .min_by_key(|enemy| {
                    (
                        self.find_player_health_at_position(&alive, enemy),
                        enemy.1,
                        enemy.0,
                    )
                }) {
                Some(enemy) => {
                    let j = self
                        .players
                        .iter()
                        .into_iter()
                        .position(|&(pos, _, _)| pos == enemy)
                        .unwrap();
                    self.players[j].2 -= self.get_attack(player);
                    // Quick hack to exit early when the elf is dead.
                    if player == 'G' && self.players[j].2 <= 0 && self.elf_power > 3 {
                        return Some('G');
                    }
                }
                None => {}
            }
        }

        self.round += 1;
        //self.draw();
        None
    }

    fn get_attack(&self, player: char) -> i32 {
        match player {
            'E' => self.elf_power,
            _ => 3,
        }
    }

    fn draw(&self) {
        println!("\nround {}", self.round);
        let mut all_tiles = self.players.clone();
        for &wall in self.walls.iter() {
            all_tiles.push((wall, '#', 999));
        }
        draw(&all_tiles);
    }

    fn find_player_health_at_position(
        &self,
        players: &Vec<(Position, char, i32)>,
        target: &Position,
    ) -> i32 {
        players
            .iter()
            .find(|(pos, _, _)| &pos == &target)
            .unwrap()
            .2
    }

    fn find_shortest_distance(
        &self,
        obstacles: &HashSet<Position>,
        from: &Position,
        to: &Position,
    ) -> Option<i32> {
        if from == to {
            return Some(0);
        }
        if manhattan_distance(from, to) == 1 {
            return Some(1);
        }
        let mut queue = vec![(*from, 0)];
        let mut visited = HashSet::new();
        visited.insert(*from);

        while !queue.is_empty() {
            let possible_moves = queue.clone();
            queue.clear();

            for (pos, dist) in possible_moves {
                for &dir in &[(0, 1), (0, -1), (1, 0), (-1, 0)] {
                    let new_pos = (pos.0 + dir.0, pos.1 + dir.1);
                    if obstacles.contains(&new_pos) {
                        continue;
                    }
                    if visited.contains(&new_pos) {
                        continue;
                    }
                    visited.insert(new_pos);
                    if new_pos == *to {
                        return Some(dist + 1);
                    }
                    queue.push((new_pos, dist + 1));
                }
            }
        }
        None
    }

    fn find_in_range(&self, obstacles: &HashSet<Position>, pos: &Position) -> Vec<Position> {
        vec![(0, 1), (0, -1), (1, 0), (-1, 0)]
            .into_iter()
            .map(|(dx, dy)| (pos.0 + dx, pos.1 + dy))
            .filter(|&(x, y)| !obstacles.contains(&(x, y)))
            .collect()
    }

    fn sort_by_read_order(&mut self) {
        self.players.sort_by_key(|(pos, _, _)| (pos.1, pos.0))
    }

    fn filter_alive(&self, players: &Vec<(Position, char, i32)>) -> Vec<(Position, char, i32)> {
        players
            .clone()
            .into_iter()
            .filter(|(_, _, hp)| *hp > 0)
            .collect()
    }

    fn find_targets(&self, players: &Vec<(Position, char, i32)>, player: char) -> Vec<Position> {
        players
            .iter()
            .filter(|(_, ch, _)| *ch != player)
            .map(|&(pos, _, _)| pos)
            .collect()
    }

    fn build_obstacles(&self, players: &Vec<(Position, char, i32)>) -> HashSet<Position> {
        let mut obstacles = self.walls.clone();
        obstacles.extend(
            players
                .clone()
                .into_iter()
                .map(|(pos, _, _)| pos)
                .collect::<Vec<Position>>(),
        );
        obstacles
    }
}

fn manhattan_distance(a: &Position, b: &Position) -> i32 {
    (a.0 - b.0).abs() + (a.1 - b.1).abs()
}

fn draw(tiles: &[(Position, char, i32)]) {
    let max_x = tiles.iter().map(|(p, _, _)| p.0).max().unwrap();
    let max_y = tiles.iter().map(|(p, _, _)| p.1).max().unwrap();

    let mut grid = vec![vec!['.'; max_x as usize + 1]; max_y as usize + 1];
    let mut units = HashMap::new();

    for (p, c, health) in tiles {
        if *health > 0 {
            grid[p.1 as usize][p.0 as usize] = *c;
        }
        if *c != '#' && *health > 0 {
            units.entry(p.1).or_insert(Vec::new()).push((p, c, health));
        }
    }

    for (y, row) in grid.into_iter().enumerate() {
        let meta = match units.get(&(y as i32)) {
            Some(cols) => {
                let mut cols = cols.clone();
                cols.sort_by_key(|&(p, _, _)| p.0);
                let meta = cols
                    .iter()
                    .map(|&(p, c, health)| format!("{} at {:?} (HP: {})", c, p, health))
                    .collect::<Vec<String>>();
                meta.join(", ")
            }
            None => "".to_string(),
        };
        println!("{} {}", row.iter().collect::<String>(), meta);
    }
}

fn parse(input: &str) -> Vec<(Position, char)> {
    let mut result = Vec::new();
    for (y, line) in input.trim().lines().enumerate() {
        for (x, ch) in line.trim().chars().enumerate() {
            if ch.is_whitespace() {
                continue;
            }
            if ch == '.' {
                continue;
            }
            result.push(((x as i32, y as i32), ch));
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_move() {
        let input = "#########
#G..G..G#
#.......#
#.......#
#G..E..G#
#.......#
#.......#
#G..G..G#
#########";
        assert_eq!(27828, Grid::new(&input).play());
    }

    #[test]
    fn test_play() {
        let input = "#######
#.G...#
#...EG#
#.#.#G#
#..G#E#
#.....#
#######";
        assert_eq!(27730, Grid::new(&input).play());

        let input = "#######
#G..#E#
#E#E.E#
#G.##.#
#...#E#
#...E.#
#######";

        assert_eq!(36334, Grid::new(&input).play());

        let input = "#######
#E..EG#
#.#G.E#
#E.##E#
#G..#.#
#..E#.#
#######";

        assert_eq!(39514, Grid::new(&input).play());

        let input = "
#######
#E.G#.#
#.#G..#
#G.#.G#
#G..#.#
#...E.#
#######";

        assert_eq!(27755, Grid::new(&input).play());

        let input = "
#########
#G......#
#.E.#...#
#..##..G#
#...##..#
#...#...#
#.G...G.#
#.....G.#
#########";

        assert_eq!(18740, Grid::new(&input).play());
    }

    #[test]
    fn test_play_part2() {
        let input = "#######
#.G...#
#...EG#
#.#.#G#
#..G#E#
#.....#
#######";
        assert_eq!(4988, play_for_elf(&input));

        let input = "#######
        #E..EG#
        #.#G.E#
        #E.##E#
        #G..#.#
        #..E#.#
        #######";

        assert_eq!(31284, play_for_elf(&input));

        let input = "
        #########
        #G......#
        #.E.#...#
        #..##..G#
        #...##..#
        #...#...#
        #.G...G.#
        #.....G.#
        #########";

        assert_eq!(1140, play_for_elf(&input));
    }
}
