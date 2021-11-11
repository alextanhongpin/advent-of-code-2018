fn main() {
    let input = include_str!("./input.txt");
    assert_eq!(7776, part1(&input));
    assert_eq!("wlkigsqyfecjqqmnxaktdrhbz".to_string(), part2(&input));
}

fn part2(input: &str) -> String {
    let input = input.trim();
    let mut lines = input.lines().collect::<Vec<_>>();
    lines.sort();
    let n = lines.len();

    for i in 0..n {
        for j in 0..n {
            if i == j {
                continue;
            }
            if let Some(pos) = difference_by_n_char(lines[i], lines[j], 1) {
                let mut line = lines[i].chars().collect::<Vec<_>>();
                line.remove(pos);
                return line.into_iter().collect::<String>();
            }
        }
    }

    "".to_string()
}

fn difference_by_n_char(src: &str, tgt: &str, n: i32) -> Option<usize> {
    let src = src.trim();
    let tgt = tgt.trim();
    if src.len() != tgt.len() {
        return None;
    }
    let mut n = n;
    let len = src.len();

    let mut src = src.chars();
    let mut tgt = tgt.chars();
    let mut pos = 0;

    for i in 0..len {
        if src.next().unwrap() != tgt.next().unwrap() {
            pos = i;
            n -= 1;
        }
    }

    if n == 0 {
        Some(pos)
    } else {
        None
    }
}

fn part1(input: &str) -> i32 {
    let input = input.trim();
    let mut twice = 0;
    let mut thrice = 0;

    for line in input.lines() {
        let repeats_twice = has_n_count(line, 2);
        let repeats_thrice = has_n_count(line, 3);
        twice += repeats_twice as i32;
        thrice += repeats_thrice as i32;
    }

    twice * thrice
}

fn has_n_count(input: &str, n: i32) -> bool {
    let chars = input.chars();

    for c in chars {
        if input.matches(c).count() == n as usize {
            return true;
        }
    }

    false
}
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
