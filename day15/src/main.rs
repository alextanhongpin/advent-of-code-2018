use std::collections::HashSet;
type Position = (i32, i32);

fn main() {
    let input = include_str!("input.txt");
    let tiles = parse(&input);
    let score = play(&tiles);
    assert_eq!(215168, score);
    //assert_eq!(52122);
}

fn play(tiles: &[(Position, char)]) -> i32 {
    // Exclude all walls.
    let mut players = tiles
        .iter()
        .filter(|&(_, ch)| *ch != '#')
        .map(|&(pos, ch)| (pos, ch, 200))
        .collect::<Vec<(Position, char, i32)>>();

    // Include only walls.
    let walls: HashSet<Position> = HashSet::from_iter(
        tiles
            .iter()
            .filter(|&(_, ch)| *ch == '#')
            .map(|&(pos, _)| pos),
    );

    let mut round = 0;
    loop {
        // Top to bottom, left to right.
        players.sort_by_key(|(pos, _, _)| (pos.1, pos.0));
        for i in 0..players.len() {
            let (position, player, health) = players[i];

            // If the player is dead, skip.
            if health <= 0 {
                continue;
            }
            let mut other_players = players.clone();
            other_players.remove(i); // Don't include yourself.
            other_players.retain(|(_, _, health)| *health > 0); // Remove dead players.

            let mut obstacles = walls.clone();
            for &(other_position, _, _) in other_players.iter() {
                obstacles.insert(other_position); // Other units are walls.
            }

            // Only target enemies.
            other_players.retain(|&(_, target, _)| target != player);
            if other_players.is_empty() {
                let total_hit_points = players
                    .into_iter()
                    .map(|(_, _, health)| health)
                    .filter(|health| *health > 0)
                    .sum::<i32>();
                println!(
                    "{} wins in {} rounds with total HP {} left",
                    player, round, total_hit_points
                );
                return round * total_hit_points;
            }

            let in_range = other_players
                .iter()
                .flat_map(|&(enemy_position, _, health)| {
                    match best_move(&obstacles, &position, &enemy_position) {
                        Some((best_move, distance)) => {
                            Some((distance, enemy_position, best_move, health))
                        }
                        None => None,
                    }
                })
                .min_by_key(|&(distance, _, position, health)| {
                    (distance, health, position.1, position.0)
                });

            match in_range {
                Some((distance, enemy_position, best_move, _)) => {
                    // Move.
                    if distance > 0 {
                        players[i].0 = best_move;
                    }
                    //println!(
                    //"Player {} moves from {:?} to {:?} through {:?} with distance {}",
                    //player, position, enemy_position, best_move, distance,
                    //);

                    // Attack.
                    if distance == 1 || distance == 2 {
                        //println!("attacking player");
                        let j = players
                            .iter()
                            .position(|&(pos, _, _)| pos == enemy_position)
                            .unwrap();
                        players[j].2 -= 3;
                    }
                }
                None => {}
            }
        }
        players.retain(|(_, _, health)| *health > 0);
        round += 1;
        //println!("\nround {}", round);
        //let mut all_tiles = players.clone();
        //for &wall in walls.iter() {
        //all_tiles.push((wall, '#', 999));
        //}
        //draw(&all_tiles);
        //if round > 2 {
        ////panic!("too high");
        //}
    }
}

fn manhattan_distance(a: &Position, b: &Position) -> i32 {
    (a.0 - b.0).abs() + (a.1 - b.1).abs()
}

fn draw(tiles: &[(Position, char, i32)]) {
    let max_x = tiles.iter().map(|(p, _, _)| p.0).max().unwrap();
    let max_y = tiles.iter().map(|(p, _, _)| p.1).max().unwrap();

    let mut grid = vec![vec!['.'; max_x as usize + 1]; max_y as usize + 1];

    for (p, c, health) in tiles {
        grid[p.1 as usize][p.0 as usize] = *c;
        if *c != '#' {
            println!("{} at {:?} (HP: {})", c, p, health);
        }
    }

    for row in grid {
        println!("{}", row.iter().collect::<String>());
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

fn best_move(
    obstacles: &HashSet<Position>,
    source: &Position,
    target: &Position,
) -> Option<(Position, i32)> {
    if manhattan_distance(source, target) == 1 {
        return Some((*source, 1));
    }

    // Find all the possible in range moves.
    let in_range = &[(0, 1), (0, -1), (1, 0), (-1, 0)]
        .into_iter()
        .map(|(dx, dy)| (target.0 + dx, target.1 + dy))
        .filter(|position| !obstacles.contains(position))
        .collect::<Vec<(i32, i32)>>();
    if in_range.is_empty() {
        return None;
    }

    let best_moves = in_range
        .into_iter()
        .map(|target| {
            // Find the distance to the closest in range move.
            let mut queue: Vec<Vec<(Position, i32)>> = vec![vec![(*source, 0)]];
            let mut visited = HashSet::new();
            visited.insert(*source);

            while !queue.is_empty() {
                let to_process = queue.clone();
                queue.clear();

                let mut found = false;
                for path in to_process {
                    let (position, distance) = path.last().unwrap().clone();
                    let moves = vec![
                        (position.0 + 1, position.1),
                        (position.0 - 1, position.1),
                        (position.0, position.1 + 1),
                        (position.0, position.1 - 1),
                    ];

                    for mv in moves {
                        if obstacles.contains(&mv) {
                            continue;
                        }
                        if visited.contains(&mv) {
                            continue;
                        }
                        queue.push(
                            path.iter()
                                .cloned()
                                .chain(vec![(mv, distance + 1)])
                                .collect(),
                        );
                        if mv == *target {
                            found = true;
                            break;
                        }
                        visited.insert(mv);
                    }
                }
                if found {
                    break;
                }
            }

            queue.retain(|paths| paths.last().unwrap().0 == *target);
            if queue.is_empty() {
                return None;
            }

            let best = queue
                .iter()
                .min_by_key(|paths| {
                    let position = paths.iter().nth(1).unwrap().0;
                    let distance = paths.iter().last().unwrap().1;
                    (distance, position.1, position.0)
                })
                .unwrap();
            Some((
                best.clone().into_iter().nth(1).map(|(p, _)| p).unwrap(),
                best.last().unwrap().1 + 1,
            ))
        })
        .flatten();
    best_moves.min_by_key(|&(position, distance)| (distance, position.1, position.0))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_best_move() {
        let input = "#######
#E..G.#
#...#.#
#.G.#G#
#######";
        let tiles = parse(&input);
        let players = tiles
            .iter()
            .filter(|&(_, ch)| *ch != '#')
            .map(|&(pos, ch)| (pos, ch, 200))
            .collect::<Vec<(Position, char, i32)>>();

        // Include only walls.
        let mut walls: HashSet<Position> = HashSet::from_iter(
            tiles
                .iter()
                .filter(|&(_, ch)| *ch == '#')
                .map(|&(pos, _)| pos),
        );

        let mut all_tiles = players.clone();
        for &wall in walls.iter() {
            all_tiles.push((wall, '#', 999));
        }
        draw(&all_tiles);

        for (pos, _, _) in players {
            walls.insert(pos.clone());
        }
        assert_eq!(Some(((2, 1), 3)), best_move(&walls, &(1, 1), &(4, 1)));
        assert_eq!(Some(((2, 1), 3)), best_move(&walls, &(1, 1), &(2, 3)));
        assert_eq!(None, best_move(&walls, &(1, 1), &(5, 3)));
    }

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
        let tiles = parse(&input);
        let score = play(&tiles);
        assert_eq!(27828, score);
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
        let tiles = parse(&input);
        let score = play(&tiles);
        assert_eq!(27730, score);

        let input = "#######
#G..#E#
#E#E.E#
#G.##.#
#...#E#
#...E.#
#######";

        let tiles = parse(&input);
        let score = play(&tiles);
        assert_eq!(36334, score);

        let input = "#######
#E..EG#
#.#G.E#
#E.##E#
#G..#.#
#..E#.#
#######";

        let tiles = parse(&input);
        let score = play(&tiles);
        assert_eq!(39514, score);

        let input = "
#######
#E.G#.#
#.#G..#
#G.#.G#
#G..#.#
#...E.#
#######";

        let tiles = parse(&input);
        let score = play(&tiles);
        assert_eq!(27755, score);

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

        let tiles = parse(&input);
        let score = play(&tiles);
        assert_eq!(18740, score);
    }
}
