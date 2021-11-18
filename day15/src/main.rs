use std::collections::{HashMap, HashSet, VecDeque};
type Position = (i32, i32);

fn main() {}

fn play(tiles: &[(Position, char)]) -> i32 {
    let mut players = tiles
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
    let mut round = 0;
    loop {
        for (i, (position, player, health)) in players.clone().iter().enumerate() {
            if *health <= 0 {
                continue;
            }
            let mut other_players = players.clone();
            other_players.remove(i);
            let other_players_excluding_you = other_players.clone();
            let mut obstacles = walls.clone();
            for other_player in other_players_excluding_you {
                obstacles.insert(other_player.0);
            }

            other_players.retain(|&(_, other, _)| other != *player);
            if other_players.is_empty() {
                return round
                    * players
                        .into_iter()
                        .map(|(_, _, health)| health)
                        .sum::<i32>();
            }

            other_players.sort_by_key(|&(other_position, _, _)| {
                let mut obstacles = obstacles.clone();
                obstacles.remove(&other_position);
                let distance = match shortest_path(&obstacles, position, &other_position) {
                    Some((distance, _)) => distance,
                    None => 9999,
                };
                (distance, other_position.1, other_position.0)
            });
            for &(other_position, _, _) in other_players.iter() {
                if manhattan_distance(&position, &other_position) == 1 {
                    let j = players
                        .iter()
                        .position(|&(pos, _, _)| pos == other_position)
                        .unwrap();
                    players[j].2 -= 3;
                    break;
                }
                obstacles.remove(&other_position);
                match shortest_path(&obstacles, position, &other_position) {
                    Some((_, best_path)) => {
                        players[i].0 = *best_path.iter().nth(1).unwrap();
                    }
                    _ => {}
                }
                break;
            }
        }
        players = players
            .into_iter()
            .filter(|(_, _, health)| *health >= 0)
            .collect();
        let mut all_tiles = players.clone();
        for &wall in walls.iter() {
            all_tiles.push((wall, '#', 999));
        }
        println!("\nbround: {}", round);
        draw(&all_tiles);
        round += 1;
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
        for (x, ch) in line.chars().enumerate() {
            if ch == '.' {
                continue;
            }
            result.push(((x as i32, y as i32), ch));
        }
    }
    result
}

fn shortest_path(
    obstacles: &HashSet<Position>,
    player: &Position,
    other: &Position,
) -> Option<(i32, Vec<Position>)> {
    if manhattan_distance(player, other) == 1 {
        return Some((0, vec![*other]));
    }
    let mut queue: VecDeque<(Position, i32)> = VecDeque::new();
    let mut visited = HashMap::new();
    queue.push_back((player.clone(), 0));
    visited.insert(player.clone(), 0);

    'find: loop {
        if queue.is_empty() {
            break;
        }
        let (position, distance) = queue.pop_front().unwrap();
        let moves = vec![
            (position.0, position.1 - 1),
            (position.0 + 1, position.1),
            (position.0 - 1, position.1),
            (position.0, position.1 + 1),
        ];
        for mv in moves {
            if obstacles.contains(&mv) {
                continue;
            }

            if visited.contains_key(&mv) {
                continue;
            }
            visited.insert(mv, distance + 1);
            if mv == *other {
                break 'find;
            }

            queue.push_back((mv, distance + 1));
        }
    }

    if visited.get(other).is_none() {
        return None;
    }

    let mut steps = visited.get(other).unwrap().clone();
    let mut path = VecDeque::new();
    path.push_back(other.clone());

    loop {
        let position = path.back().unwrap();
        let moves = vec![
            (position.0, position.1 - 1),
            (position.0 + 1, position.1),
            (position.0 - 1, position.1),
            (position.0, position.1 + 1),
        ];
        for mv in moves {
            if !visited.contains_key(&mv) {
                continue;
            }
            if *visited.get(&mv).unwrap() == steps - 1 {
                path.push_back(mv);
                steps -= 1;
                break;
            }
        }
        if steps == 0 {
            break;
        }
    }

    Some((
        *visited.get(other).unwrap(),
        path.into_iter().rev().collect(),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        //let input = "#########
        //#G..G..G#
        //#.......#
        //#.......#
        //#G..E..G#
        //#.......#
        //#.......#
        //#G..G..G#
        //#########";

        //let tiles = parse(&input);
        //draw(&tiles);
        //play(&tiles);
        let result = 2 + 2;
        assert_eq!(result, 4);
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
    }
}
