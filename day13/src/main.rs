use std::collections::{HashMap, HashSet};

type Position = (i32, i32);

type Track = char;
type Map = (Position, Track);

fn main() {
    let input = include_str!("input.txt");
    let map = parse(&input);
    assert_eq!((53, 133), find_crash_point(&map, false));
    assert_eq!((111, 68), find_crash_point(&map, true));
}

fn draw(carts: &Vec<(Position, Track, usize)>, paths: &HashMap<Position, Track>) {
    let positions = paths.iter().map(|(pos, _)| pos).collect::<Vec<&Position>>();
    let max_x = positions.iter().map(|&(x, _)| x).max().unwrap();
    let max_y = positions.iter().map(|&(_, y)| y).max().unwrap();
    let mut map = vec![vec![' '; *max_x as usize + 1]; *max_y as usize + 1];
    for (pos, track) in paths {
        map[pos.1 as usize][pos.0 as usize] = *track;
    }
    for (pos, cart, _) in carts {
        map[pos.1 as usize][pos.0 as usize] = *cart;
    }
    for row in map {
        println!("{}", row.iter().collect::<String>());
    }
}

fn find_crash_point(map: &Vec<Map>, remove_cart_on_crashed: bool) -> Position {
    let mut carts: Vec<(Position, Track, usize)> = Vec::new();
    let mut paths: HashMap<Position, Track> = HashMap::new();
    for &(pos, track) in map {
        match track {
            '^' | 'v' | '<' | '>' => carts.push((pos, track, 0)),
            _ => {
                paths.insert(pos, track);
            }
        }
    }

    loop {
        let mut carts_to_move = carts.clone();
        carts_to_move.sort_by_key(|(pos, _, _)| (pos.1, pos.0));
        carts.clear();

        let mut crash_point: HashSet<Position> = HashSet::new();
        for (pos, cart, count) in carts_to_move.clone() {
            if crash_point.contains(&pos) {
                continue;
            }
            let new_pos = match cart {
                '^' => (pos.0, pos.1 - 1),
                'v' => (pos.0, pos.1 + 1),
                '<' => (pos.0 - 1, pos.1),
                '>' => (pos.0 + 1, pos.1),
                _ => panic!("Unknown cart type: {:?}", cart),
            };

            if crash_point.contains(&new_pos) {
                if remove_cart_on_crashed {
                    carts.retain(|&(pos, _, _)| pos != new_pos);
                    continue;
                } else {
                    return new_pos;
                }
            }

            let track = paths.get(&new_pos).unwrap();
            let new_dir = match (cart, track) {
                ('^', '/') => '>',
                ('^', '\\') => '<',
                ('^', '|') => '^',
                ('v', '/') => '<',
                ('v', '\\') => '>',
                ('v', '|') => 'v',
                ('<', '-') => '<',
                ('<', '/') => 'v',
                ('<', '\\') => '^',
                ('>', '-') => '>',
                ('>', '/') => '^',
                ('>', '\\') => 'v',
                (c, '+') => {
                    match count % 3 {
                        // Turn left
                        0 => match c {
                            '<' => 'v',
                            '>' => '^',
                            '^' => '<',
                            'v' => '>',
                            _ => panic!("Unknown cart type: {:?}", c),
                        },
                        // Straight
                        1 => c,
                        // Turn right
                        2 => match c {
                            '<' => '^',
                            '>' => 'v',
                            '^' => '>',
                            'v' => '<',
                            _ => panic!("Unknown cart type: {:?}", c),
                        },
                        _ => panic!("Unknown turn: {:?}", count),
                    }
                }
                other => panic!("Unknown track combination: {:?} {}", other, count),
            };
            let count = match track {
                '+' => count + 1,
                _ => count,
            };

            carts.push((new_pos, new_dir, count));
            crash_point.insert(new_pos);
        }
        if carts.len() == 1 {
            return carts[0].0;
        }
    }
}

fn parse(input: &str) -> Vec<Map> {
    let mut tracks: Vec<Map> = Vec::new();
    for (y, line) in input.lines().enumerate() {
        for (x, track) in line.chars().enumerate() {
            if track.is_whitespace() {
                continue;
            }
            let track: Track = track.into();
            match track {
                '^' | 'v' => {
                    tracks.push(((x as i32, y as i32), '|'));
                }
                '<' | '>' => {
                    tracks.push(((x as i32, y as i32), '-'));
                }
                _ => {}
            }
            tracks.push(((x as i32, y as i32), track));
        }
    }
    tracks
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let input = r"/->-\        
|   |  /----\
| /-+--+-\  |
| | |  | v  |
\-+-/  \-+--/
  \------/   ";

        assert_eq!(50, parse(&input).len());
    }

    #[test]
    fn test_crash() {
        let input = r"/->-\        
|   |  /----\
| /-+--+-\  |
| | |  | v  |
\-+-/  \-+--/
  \------/   ";
        assert_eq!((7, 3), find_crash_point(&parse(input), false));
    }

    #[test]
    fn test_last_cart() {
        let input = r"/>-<\  
|   |  
| /<+-\
| | | v
\>+</ |
  |   ^
  \<->/";
        assert_eq!((6, 4), find_crash_point(&parse(input), true));
    }
}
