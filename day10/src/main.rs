use regex::Regex;

type Position = (i32, i32);
type Velocity = (i32, i32);

fn main() {
    let input = include_str!("input.txt");
    draw(&input, 100000, 1);
    // 10813
}

fn parse(input: &str) -> Vec<(Position, Velocity)> {
    input
        .trim()
        .lines()
        .map(parse_row)
        .collect::<Vec<(Position, Velocity)>>()
}

fn parse_row(input: &str) -> (Position, Velocity) {
    let re = Regex::new(r"position=<(\s*\-?\d+),(\s*\-?\d+)> velocity=<(\s*\-?\d+),(\s*\-?\d+)>")
        .unwrap();
    let cap = re.captures(input).unwrap();

    let pos_x = cap[1].trim();
    let pos_y = cap[2].trim();
    let vel_x = cap[3].trim();
    let vel_y = cap[4].trim();

    (
        (pos_x.parse::<i32>().unwrap(), pos_y.parse::<i32>().unwrap()),
        (vel_x.parse::<i32>().unwrap(), vel_y.parse::<i32>().unwrap()),
    )
}

fn draw(input: &str, frames: i32, duration: u64) {
    let points = parse(input);

    for i in 1..=frames {
        let coordinates = points
            .iter()
            .map(|((x, y), (vx, vy))| (x + i * vx, y + i * vy))
            .collect::<Vec<Position>>();
        let min_x = coordinates.iter().map(|(x, _)| x).min().unwrap().clone();
        let min_y = coordinates.iter().map(|(_, y)| y).min().unwrap().clone();
        if min_x < 0 || min_y < 0 {
            continue;
        }

        let max_x = coordinates.iter().map(|(x, _)| x).max().unwrap().clone();
        let max_y = coordinates.iter().map(|(_, y)| y).max().unwrap().clone();
        let mut grid = vec![vec!['.'; (max_x + 1) as usize]; (max_y + 1) as usize];

        println!("seconds: {}", i);
        for (x, y) in coordinates {
            if x >= 0 && x <= max_x && y >= 0 && y <= max_y {
                grid[y as usize][x as usize] = '#';
            }
        }
        let map = grid
            .iter()
            .map(|row| row.iter().collect::<String>())
            .collect::<Vec<String>>()
            .join("\n");
        println!("{}\n", map);
        std::thread::sleep(std::time::Duration::from_millis(duration));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "position=< 9,  1> velocity=< 0,  2>
position=< 7,  0> velocity=<-1,  0>
position=< 3, -2> velocity=<-1,  1>
position=< 6, 10> velocity=<-2, -1>
position=< 2, -4> velocity=< 2,  2>
position=<-6, 10> velocity=< 2, -2>
position=< 1,  8> velocity=< 1, -1>
position=< 1,  7> velocity=< 1,  0>
position=<-3, 11> velocity=< 1, -2>
position=< 7,  6> velocity=<-1, -1>
position=<-2,  3> velocity=< 1,  0>
position=<-4,  3> velocity=< 2,  0>
position=<10, -3> velocity=<-1,  1>
position=< 5, 11> velocity=< 1, -2>
position=< 4,  7> velocity=< 0, -1>
position=< 8, -2> velocity=< 0,  1>
position=<15,  0> velocity=<-2,  0>
position=< 1,  6> velocity=< 1,  0>
position=< 8,  9> velocity=< 0, -1>
position=< 3,  3> velocity=<-1,  1>
position=< 0,  5> velocity=< 0, -1>
position=<-2,  2> velocity=< 2,  0>
position=< 5, -2> velocity=< 1,  2>
position=< 1,  4> velocity=< 2,  1>
position=<-2,  7> velocity=< 2, -2>
position=< 3,  6> velocity=<-1, -1>
position=< 5,  0> velocity=< 1,  0>
position=<-6,  0> velocity=< 2,  0>
position=< 5,  9> velocity=< 1, -2>
position=<14,  7> velocity=<-2,  0>
position=<-3,  6> velocity=< 2, -1>";
        draw(&input, 3, 0);
        let result = parse_row("position=< 9,  1> velocity=< 0,  2>");
        assert_eq!(((9, 1), (0, 2)), result);

        assert_eq!(true, false);
    }
}
