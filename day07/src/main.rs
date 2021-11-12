use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("input.txt");
    assert_eq!("IBJTUWGFKDNVEYAHOMPCQRLSZX".to_string(), part1(input));
}

fn part2(input: &str) -> i32 {
    0
}

fn part1(input: &str) -> String {
    let mut ltr = HashMap::new();
    let mut rtl = HashMap::new();
    for line in input.trim().lines() {
        let (head, tail) = parse(line);
        ltr.entry(head).or_insert_with(HashSet::new).insert(tail);
        *rtl.entry(tail).or_insert(0) += 1;
    }

    let left_keys: HashSet<char> = HashSet::from_iter(ltr.keys().cloned().collect::<Vec<char>>());
    let right_keys = HashSet::from_iter(rtl.keys().cloned());
    let mut queue: Vec<char> = left_keys.difference(&right_keys).cloned().collect();
    for &key in queue.iter() {
        *rtl.entry(key).or_insert(0) += 1;
    }
    let mut done: Vec<char> = vec![];
    loop {
        if queue.is_empty() {
            break;
        }
        queue.sort();

        let head = queue.remove(0);
        if !rtl.contains_key(&head) {
            continue;
        }
        *rtl.entry(head).or_insert(0) -= 1;
        if rtl[&head] != 0 {
            continue;
        }
        done.push(head);
        rtl.remove(&head);

        match ltr.get(&head) {
            Some(keys) => {
                let mut keys: Vec<char> = keys.iter().map(|&k| k).collect();
                queue.append(&mut keys);
            }
            None => {}
        }
    }

    done.iter().collect::<String>()
}

fn parse(input: &str) -> (char, char) {
    let mut chars = input.chars();
    let head = chars.nth(5).unwrap();
    let tail = chars.nth(30).unwrap();
    (head, tail)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let input = "Step C must be finished before step A can begin.";
        assert_eq!(('C', 'A'), parse(input));
    }

    #[test]
    fn test_part1() {
        let input = "Step C must be finished before step A can begin.
Step C must be finished before step F can begin.
Step A must be finished before step B can begin.
Step A must be finished before step D can begin.
Step B must be finished before step E can begin.
Step D must be finished before step E can begin.
Step F must be finished before step E can begin.";

        assert_eq!("CABDFE", part1(input));
    }
}
