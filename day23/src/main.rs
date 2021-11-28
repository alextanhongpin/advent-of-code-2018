use regex::Regex;

fn main() {
    let input = include_str!("input.txt");
    let n = nanobots(input);
    let nanobot = n.iter().max_by_key(|bot| (bot.r)).unwrap();

    let in_range = n
        .iter()
        .filter(|&bot| bot.distance(&nanobot) <= nanobot.r)
        .count();

    assert_eq!(0, in_range);
}

#[derive(Debug)]
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
    fn it_works() {
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

        let in_range = n
            .iter()
            .filter(|&bot| bot.distance(&nanobot) <= nanobot.r)
            .count();
        assert_eq!(7, in_range);
    }
}
