use regex::Regex;
use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("input.txt");
    let n = nanobots(input);
    let nanobot = n.iter().max_by_key(|bot| (bot.r)).unwrap();
    assert_eq!(674, in_range(&n, &nanobot));
    assert_eq!(0, most_bots(&n));
}

fn in_range(n: &Vec<Nanobot>, nanobot: &Nanobot) -> i32 {
    n.iter()
        .filter(|&bot| bot.distance(nanobot) <= nanobot.r)
        .count() as i32
}

fn most_bots(n: &Vec<Nanobot>) -> i32 {
    let zero_bot = Nanobot {
        x: 0,
        y: 0,
        z: 0,
        r: 0,
    };
    let mut pos = HashSet::new();
    for bot in n {
        pos.insert(bot.x - bot.r);
        pos.insert(bot.x + bot.r);
        pos.insert(bot.y - bot.r);
        pos.insert(bot.y + bot.r);
        pos.insert(bot.z - bot.r);
        pos.insert(bot.z + bot.r);
    }
    let pos = pos.into_iter().map(|x| x as i32).collect::<Vec<_>>();

    let mut counter = HashMap::new();
    for x in pos.iter() {
        for y in pos.iter() {
            for z in pos.iter() {
                for bot in n.iter() {
                    let pos = Nanobot {
                        x: *x,
                        y: *y,
                        z: *z,
                        r: 0,
                    };
                    if pos.distance(&bot) <= bot.r {
                        *counter.entry(pos).or_insert(0) += 1;
                    }
                }
            }
        }
    }

    let nearest_bot = counter
        .into_iter()
        .max_by(|a, b| match a.1.cmp(&b.1) {
            std::cmp::Ordering::Equal => b.0.distance(&zero_bot).cmp(&a.0.distance(&zero_bot)),
            other => other,
        })
        .unwrap()
        .0;
    nearest_bot.distance(&zero_bot)
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
struct Nanobot {
    x: i32,
    y: i32,
    z: i32,
    r: i32,
}

impl From<&str> for Nanobot {
    fn from(s: &str) -> Nanobot {
        let re = Regex::new(r"pos=<(-?\d+),(-?\d+),(-?\d+)>, r=(-?\d+)").unwrap();
        let cap = re.captures(s).unwrap();

        Nanobot {
            x: cap[1].parse().unwrap(),
            y: cap[2].parse().unwrap(),
            z: cap[3].parse().unwrap(),
            r: cap[4].parse().unwrap(),
        }
    }
}

impl Nanobot {
    fn distance(&self, other: &Nanobot) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs() + (self.z - other.z).abs()
    }
}

fn nanobots(input: &str) -> Vec<Nanobot> {
    input.lines().map(|l| Nanobot::from(l)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "pos=<0,0,0>, r=4
pos=<1,0,0>, r=1
pos=<4,0,0>, r=3
pos=<0,2,0>, r=1
pos=<0,5,0>, r=3
pos=<0,0,3>, r=1
pos=<1,1,1>, r=1
pos=<1,1,2>, r=1
pos=<1,3,1>, r=1";
        let n = nanobots(input);
        let nanobot = n.iter().max_by_key(|bot| (bot.r)).unwrap();
        assert_eq!(4, nanobot.r);
        assert_eq!(7, in_range(&n, &nanobot));
    }

    #[test]
    fn test_part2() {
        let input = "pos=<10,12,12>, r=2
pos=<12,14,12>, r=2
pos=<16,12,12>, r=4
pos=<14,14,14>, r=6
pos=<50,50,50>, r=200
pos=<10,10,10>, r=5";
        let n = nanobots(input);
        assert_eq!(36, most_bots(&n));
    }
}
