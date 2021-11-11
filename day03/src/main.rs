use regex::Regex;
use std::collections::HashMap;

type Position = (i32, i32);

#[derive(Debug, Eq, PartialEq)]
struct Fabric {
    id: i32,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
}

fn main() {
    let input = include_str!("./input.txt");
    assert_eq!(116140, part1(&input));
    assert_eq!(574, part2(&input));
}

fn parse_fabric(input: &str) -> HashMap<i32, Fabric> {
    let mut fabric: HashMap<i32, Fabric> = HashMap::new();

    let input = input.trim();
    for line in input.lines() {
        let part = parse(line);
        fabric.insert(part.id, part);
    }

    fabric
}

fn claims(fabric: &HashMap<i32, Fabric>) -> HashMap<Position, Vec<i32>> {
    let mut claims: HashMap<Position, Vec<i32>> = HashMap::new();

    for (_, part) in fabric {
        for x in part.x..part.x + part.width {
            for y in part.y..part.y + part.height {
                let pos = (x, y);
                let arr = claims.entry(pos).or_insert_with(Vec::new);
                arr.push(part.id);
            }
        }
    }

    claims
}

fn part1(input: &str) -> i32 {
    let fabric = parse_fabric(input);
    let claims = claims(&fabric);
    let mut total = 0;
    for (_, ids) in claims {
        if ids.len() > 1 {
            total += 1;
        }
    }

    total
}

fn part2(input: &str) -> i32 {
    let fabric = parse_fabric(input);
    let claims = claims(&fabric);

    let mut claimed = HashMap::new();
    for (_, ids) in claims {
        if ids.len() == 1 {
            for id in ids {
                let count = claimed.entry(id).or_insert(0);
                *count += 1;
            }
        }
    }

    for (id, count) in claimed {
        let part = fabric.get(&id).unwrap();
        if count == part.width * part.height {
            return id;
        }
    }

    panic!("not santa's fabric")
}

fn parse(input: &str) -> Fabric {
    let input = input.trim();

    let re = Regex::new(r"^#(\d+)\s+@\s+(\d+),(\d+):\s+(\d+)x(\d+)").unwrap();
    let cap = re.captures(input).unwrap();
    let id = &cap[1];
    let x = &cap[2];
    let y = &cap[3];
    let width = &cap[4];
    let height = &cap[5];

    Fabric {
        id: id.parse::<i32>().unwrap(),
        x: x.parse::<i32>().unwrap(),
        y: y.parse::<i32>().unwrap(),
        width: width.parse::<i32>().unwrap(),
        height: height.parse::<i32>().unwrap(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let fabric = parse("#123 @ 3,2: 5x4");
        assert_eq!(
            Fabric {
                id: 123,
                x: 3,
                y: 2,
                width: 5,
                height: 4,
            },
            fabric
        );
    }
}
