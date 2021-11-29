use regex::Regex;
use std::collections::HashMap;

fn main() {
    let input = include_str!("input.txt");
    let n = nanobots(input);
    let nanobot = n.iter().max_by_key(|bot| (bot.r)).unwrap();
    assert_eq!(674, in_range(&n, &nanobot));
    assert_eq!(129444177, most_bots(&n));
}

fn in_range(n: &Vec<Nanobot>, nanobot: &Nanobot) -> i32 {
    n.iter()
        .filter(|&bot| bot.distance(nanobot) <= nanobot.r)
        .count() as i32
}

fn scale_down(n: &Vec<Nanobot>, factor: i32) -> Vec<Nanobot> {
    n.into_iter()
        .map(|bot| {
            let mut bot = bot.clone();
            bot.x /= factor;
            bot.y /= factor;
            bot.z /= factor;
            bot.r /= factor;
            bot
        })
        .collect()
}

fn most_bots(bots: &Vec<Nanobot>) -> i32 {
    let mut factor = 1e7 as i32;

    let mut min_x = bots.iter().map(|bot| bot.x).min().unwrap() / factor;
    let mut max_x = bots.iter().map(|bot| bot.x).max().unwrap() / factor;
    let mut min_y = bots.iter().map(|bot| bot.y).min().unwrap() / factor;
    let mut max_y = bots.iter().map(|bot| bot.y).max().unwrap() / factor;
    let mut min_z = bots.iter().map(|bot| bot.z).min().unwrap() / factor;
    let mut max_z = bots.iter().map(|bot| bot.z).max().unwrap() / factor;
    let mut nearest_bot = (
        Nanobot {
            x: 0,
            y: 0,
            z: 0,
            r: 0,
        },
        0,
    );

    while factor != 0 {
        let n = &scale_down(bots, factor);
        let mut counter = HashMap::new();

        for x in min_x..=max_x {
            for y in min_y..=max_y {
                for z in min_z..=max_z {
                    for bot in n.iter() {
                        let pos = Nanobot { x, y, z, r: 0 };
                        if bot.distance(&pos) <= bot.r {
                            *counter.entry(pos).or_insert(0) += 1;
                        }
                    }
                }
            }
        }

        nearest_bot = counter
            .into_iter()
            .max_by(|a, b| match a.1.cmp(&b.1) {
                std::cmp::Ordering::Equal => a.0.from_origin().cmp(&b.0.from_origin()),
                other => other,
            })
            .unwrap()
            .clone();
        println!("factor: {}, nearest_bot: {:?}", factor, nearest_bot);
        min_x = (nearest_bot.0.x - 1) * 10;
        max_x = (nearest_bot.0.x + 1) * 10;
        min_y = (nearest_bot.0.y - 1) * 10;
        max_y = (nearest_bot.0.y + 1) * 10;
        min_z = (nearest_bot.0.z - 1) * 10;
        max_z = (nearest_bot.0.z + 1) * 10;
        factor /= 10;
    }

    nearest_bot.0.from_origin()
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

    fn from_origin(&self) -> i32 {
        self.x.abs() + self.y.abs() + self.z.abs()
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
