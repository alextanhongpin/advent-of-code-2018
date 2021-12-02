use std::collections::HashMap;

fn main() {
    let input = include_str!("input.txt");
    assert_eq!(0, part1(input));
}

type Coordinates = (i32, i32, i32, i32);

fn distance(a: Coordinates, b: Coordinates) -> i32 {
    let (x1, y1, z1, t1) = a;
    let (x2, y2, z2, t2) = b;
    (x1 - x2).abs() + (y1 - y2).abs() + (z1 - z2).abs() + (t1 - t2).abs()
}

fn parse(input: &str) -> Vec<Coordinates> {
    input
        .trim()
        .lines()
        .map(|line| {
            let mut parts = line.trim().split(',');
            let a = parts.next().unwrap().parse().unwrap();
            let b = parts.next().unwrap().parse().unwrap();
            let c = parts.next().unwrap().parse().unwrap();
            let d = parts.next().unwrap().parse().unwrap();
            (a, b, c, d)
        })
        .collect()
}

fn part1(input: &str) -> i32 {
    let mut coordinates = parse(input);
    let mut stack = vec![];
    let mut count = 0;

    while !coordinates.is_empty() {
        if stack.is_empty() {
            stack.push(coordinates.remove(0));
            count += 1;
        } else {
            let curr = stack.pop().unwrap();
            let nearby = coordinates
                .clone()
                .into_iter()
                .filter(|coord| distance(*coord, curr) <= 3);
            for n in nearby {
                stack.push(n);
            }
            coordinates = coordinates
                .clone()
                .into_iter()
                .filter(|coord| distance(*coord, curr) > 3)
                .collect();
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = " 0,0,0,0
 3,0,0,0
 0,3,0,0
 0,0,3,0
 0,0,0,3
 0,0,0,6
 9,0,0,0
12,0,0,0";
        assert_eq!(2, part1(input));

        let input = "-1,2,2,0
0,0,2,-2
0,0,0,-2
-1,2,0,0
-2,-2,-2,2
3,0,2,-1
-1,3,2,2
-1,0,-1,0
0,2,1,-2
3,0,0,0";
        assert_eq!(4, part1(input));

        let input = "1,-1,0,1
2,0,-1,0
3,2,-1,0
0,0,3,1
0,0,-1,-1
2,3,-2,0
-2,2,0,0
2,-2,0,-1
1,-1,0,-1
3,2,0,2";
        assert_eq!(3, part1(input));

        let input = "1,-1,-1,-2
-2,-2,0,1
0,2,1,3
-2,3,-2,1
0,2,3,-2
-1,-1,1,-2
0,-2,-1,0
-2,2,3,-1
1,2,2,0
-1,-2,0,-2";
        assert_eq!(8, part1(input));
    }
}
