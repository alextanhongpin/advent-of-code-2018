use std::collections::HashSet;

fn main() {
    let input = include_str!("./input.txt");
    assert_eq!(406, part1(&input));
    assert_eq!(312, part2(&input));
}

fn part1(input: &str) -> i32 {
    let input = input.trim();
    input
        .lines()
        .map(|line| line.parse::<i32>().unwrap())
        .sum::<i32>()
}

fn part2(input: &str) -> i32 {
    let input = input.trim();
    let numbers = input.lines().map(|line| line.parse::<i32>().unwrap());

    let mut cache = HashSet::new();
    let mut curr = 0;
    cache.insert(curr);

    for n in numbers.cycle() {
        curr += n;
        if cache.contains(&curr) {
            return curr;
        }
        cache.insert(curr);
    }

    panic!("No duplicate found!");
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
