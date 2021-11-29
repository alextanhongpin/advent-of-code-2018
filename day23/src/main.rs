use regex::Regex;
use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("input.txt");
    let n = nanobots(input);
    let nanobot = n.iter().max_by_key(|bot| (bot.r)).unwrap();
    assert_eq!(674, in_range(&n, &nanobot));
    // Too high.
    //Nanobot { x: 20443518, y: 45034833, z: 68671021}}
    assert_eq!(134149372, most_bots(&n));
}

fn in_range(n: &Vec<Nanobot>, nanobot: &Nanobot) -> i32 {
    n.iter()
        .filter(|&bot| bot.distance(nanobot) <= nanobot.r)
        .count() as i32
}

fn scale_down(n: &Vec<Nanobot>) -> Vec<Nanobot> {
    n.into_iter()
        .map(|bot| {
            let mut bot = bot.clone();
            bot.x /= 1e6 as i32;
            bot.y /= 1e6 as i32;
            bot.z /= 1e6 as i32;
            bot.r /= 1e6 as i32;
            bot
        })
        .collect()
}

fn most_bots(n: &Vec<Nanobot>) -> i32 {
    //let n = &scale_down(n);
    let zero_bot = Nanobot {
        x: 0,
        y: 0,
        z: 0,
        r: 0,
    };
    let mut xs = HashSet::new();
    let mut ys = HashSet::new();
    let mut zs = HashSet::new();
    let mut x_ranges = vec![];
    let mut y_ranges = vec![];
    let mut z_ranges = vec![];

    for bot in n {
        xs.insert(bot.x);
        xs.insert(bot.x - bot.r);
        xs.insert(bot.x + bot.r);
        x_ranges.push((bot.x - bot.r, bot.x + bot.r));

        ys.insert(bot.y);
        ys.insert(bot.y - bot.r);
        ys.insert(bot.y + bot.r);
        y_ranges.push((bot.y - bot.r, bot.y + bot.r));

        zs.insert(bot.z);
        zs.insert(bot.z - bot.r);
        zs.insert(bot.z + bot.r);
        z_ranges.push((bot.z - bot.r, bot.z + bot.r));
    }
    let mut x_count = HashMap::new();
    for x in xs.clone() {
        for (min, max) in x_ranges.iter() {
            if x >= *min && x <= *max {
                *x_count.entry(x).or_insert(0) += 1;
            }
        }
    }
    let mut y_count = HashMap::new();
    for y in ys.clone() {
        for (min, max) in y_ranges.iter() {
            if y >= *min && y <= *max {
                *y_count.entry(y).or_insert(0) += 1;
            }
        }
    }
    let mut z_count = HashMap::new();
    for z in zs.clone() {
        for (min, max) in z_ranges.iter() {
            if z >= *min && z <= *max {
                *z_count.entry(z).or_insert(0) += 1;
            }
        }
    }
    let min_x = x_count
        .iter()
        .max_by_key(|(x, count)| (*count, -*x))
        .unwrap()
        .0;
    let max_x = x_count
        .iter()
        .max_by_key(|(x, count)| (*count, *x))
        .unwrap()
        .0;
    let min_y = y_count
        .iter()
        .max_by_key(|(y, count)| (*count, -*y))
        .unwrap()
        .0;
    let max_y = y_count
        .iter()
        .max_by_key(|(y, count)| (*count, *y))
        .unwrap()
        .0;
    let min_z = z_count
        .iter()
        .max_by_key(|(z, count)| (*count, -*z))
        .unwrap()
        .0;
    let max_z = z_count
        .iter()
        .max_by_key(|(z, count)| (*count, *z))
        .unwrap()
        .0;

    let xs = xs
        .into_iter()
        .filter(|x| x >= min_x && x <= max_x)
        .collect::<Vec<i32>>();
    let ys = ys
        .into_iter()
        .filter(|y| y >= min_y && y <= max_y)
        .collect::<Vec<i32>>();
    let zs = zs
        .into_iter()
        .filter(|z| z >= min_z && z <= max_z)
        .collect::<Vec<i32>>();

    let mut counter = HashMap::new();
    for x in xs {
        for y in ys.clone() {
            for z in zs.clone() {
                for bot in n.iter() {
                    let pos = Nanobot {
                        x: x,
                        y: y,
                        z: z,
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
            std::cmp::Ordering::Equal => a.0.distance(&zero_bot).cmp(&b.0.distance(&zero_bot)),
            other => other,
        })
        .unwrap()
        .0;
    println!("{:?}", nearest_bot);
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
