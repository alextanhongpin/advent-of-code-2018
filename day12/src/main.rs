use std::collections::{HashMap, VecDeque};

fn main() {
    let input = include_str!("input.txt");
    assert_eq!(3258, plant(&input, 20));
    assert_eq!(3600000002022, plant(&input, 50000000000));
}

fn plant(input: &str, generation: usize) -> i64 {
    let mut input = input.trim().lines();
    let mut state = input
        .next()
        .unwrap()
        .replace("initial state: ", "")
        .trim()
        .chars()
        .collect::<VecDeque<char>>();

    let mut rules = HashMap::new();
    for pattern in input {
        if pattern.trim().len() == 0 {
            continue;
        }
        let parts = pattern
            .split(" => ")
            .map(ToString::to_string)
            .collect::<Vec<String>>();
        rules.insert(bit(&parts[0]), parts[1].trim().chars().next().unwrap());
    }

    let mut delta = 0;
    let mut last_count: i64 = 0;
    for gen in 1..=generation {
        state.push_front('.');
        state.push_front('.');
        state.push_front('.');
        state.push_front('.');
        state.push_back('.');
        state.push_back('.');
        state.push_back('.');
        state.push_back('.');

        let mut new_state = state.clone();
        for i in 2..state.len() - 2 {
            let rule = bit(&state.range(i - 2..=i + 2).copied().collect::<String>());
            new_state[i] = *rules.get(&rule).unwrap_or(&'.');
        }
        state = new_state;
        let new_count = state
            .iter()
            .enumerate()
            .map(|(i, &c)| {
                if c == '#' {
                    (i as i64) - (gen as i64 * 4)
                } else {
                    0
                }
            })
            .sum::<i64>();
        let new_delta = new_count - last_count;
        if delta == new_delta {
            return state
                .iter()
                .enumerate()
                .map(|(i, &c)| {
                    if c == '#' {
                        (i as i64) - (gen as i64 * 4)
                    } else {
                        0
                    }
                })
                .sum::<i64>()
                + delta * (generation as i64 - gen as i64);
        }
        delta = new_delta;
        last_count = new_count;
    }

    state
        .iter()
        .enumerate()
        .map(|(i, &c)| {
            if c == '#' {
                (i as i64) - (generation as i64 * 4)
            } else {
                0
            }
        })
        .sum()
}

fn bit(pattern: &str) -> usize {
    pattern.chars().enumerate().fold(0, |acc, (i, c)| {
        acc | match c {
            '#' => 1 << i,
            _ => 0,
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(24, bit("...##"));
        assert_eq!(4, bit("..#.."));
    }

    #[test]
    fn test_plant() {
        let input = "initial state: #..#.#..##......###...###

...## => #
..#.. => #
.#... => #
.#.#. => #
.#.## => #
.##.. => #
.#### => #
#.#.# => #
#.### => #
##.#. => #
##.## => #
###.. => #
###.# => #
####. => #";
        assert_eq!(325, plant(input, 20));
    }
}
