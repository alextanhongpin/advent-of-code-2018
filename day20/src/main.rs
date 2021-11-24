use std::collections::HashMap;

fn main() {
    let input = include_str!("input.txt");
    assert_eq!(3465, walk(input));
    assert_eq!(0, part2(input));
}

#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
struct Node {
    x: i32,
    y: i32,
}

fn parse(input: &str) -> HashMap<Node, i32> {
    let mut map: HashMap<Node, i32> = HashMap::new();
    let mut curr = (Node { x: 0, y: 0 }, 0);
    let mut stack = vec![];

    for c in input.trim().chars() {
        match c {
            '(' => {
                stack.push(curr);
            }
            '|' => {
                curr = stack.last().unwrap().clone();
            }
            ')' => {
                curr = stack.pop().unwrap();
            }
            'N' | 'W' | 'S' | 'E' => {
                let dx = (c == 'E') as i32 - (c == 'W') as i32;
                let dy = (c == 'S') as i32 - (c == 'N') as i32;
                curr.0.x += dx;
                curr.0.y += dy;
                curr.1 += 1;
                if map.get(&curr.0).map(|&d| d > curr.1).unwrap_or(true) {
                    map.insert(curr.0.clone(), curr.1);
                }
            }
            '^' | '$' => {}
            _ => panic!("unexpected character: {}", c),
        }
    }
    map
}

fn walk(input: &str) -> i32 {
    let map = parse(input);
    *map.values().max().unwrap()
}

fn part2(input: &str) -> i32 {
    let map = parse(input);
    map.values().filter(|&d| d >= &1000).count() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(3, walk("^WNE$"));
        assert_eq!(10, walk("^ENWWW(NEEE|SSE(EE|N))$"));
        assert_eq!(18, walk("^ENNWSWW(NEWS|)SSSEEN(WNSE|)EE(SWEN|)NNN$"));
        assert_eq!(
            23,
            walk("^ESSWWN(E|NNENN(EESS(WNSE|)SSS|WWWSSSSE(SW|NNNE)))$")
        );
        assert_eq!(
            31,
            walk("^WSSEESWWWNW(S|NENNEEEENN(ESSSSW(NWSW|SSEN)|WSWWN(E|WWS(E|SS))))$")
        );
    }
}
