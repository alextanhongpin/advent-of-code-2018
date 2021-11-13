use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("input.txt");
    assert_eq!("IBJTUWGFKDNVEYAHOMPCQRLSZX".to_string(), part1(input));
    assert_eq!(
        ("ITUWBJGFKDNVYAEHQOMPCRLSZX".to_string(), 0),
        part2(input, 5, 60)
    );
}

fn char_position(c: char) -> usize {
    (c as usize - 'A' as usize) + 1
}

fn part2(input: &str, workers: usize, offset: i32) -> (String, i32) {
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
    let mut timer = HashMap::new();

    for &c in left_keys.union(&right_keys) {
        timer.insert(c, char_position(c) as i32 + offset);
    }
    let mut workers: Vec<Option<char>> = vec![None; workers as usize];

    let mut seconds = 0;
    let mut done: Vec<char> = vec![];
    loop {
        if timer.is_empty() {
            break;
        }

        for i in 0..workers.len() {
            if workers[i].is_none() {
                queue.sort();
                if queue.is_empty() {
                    break;
                }
                let key = queue.remove(0);
                workers[i] = Some(key);
            }
        }

        seconds += 1;

        for i in 0..workers.len() {
            if workers[i].is_none() {
                continue;
            }
            let head = workers[i].unwrap();
            let time = timer.entry(head).or_insert(0);
            *time -= 1;
            if *time > 0 {
                continue;
            }

            timer.remove(&head);
            done.push(head);
            workers[i] = None;

            match ltr.get(&head) {
                Some(keys) => {
                    for key in keys {
                        if rtl[&key] == 1 {
                            queue.push(*key);
                            continue;
                        }
                        *rtl.entry(*key).or_insert(0) -= 1;
                    }
                }
                None => {}
            }
        }
    }

    (done.iter().collect::<String>(), seconds)
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
        done.push(head);

        match ltr.get(&head) {
            Some(keys) => {
                for key in keys {
                    if rtl[&key] == 1 {
                        queue.push(*key);
                        continue;
                    }
                    *rtl.entry(*key).or_insert(0) -= 1;
                }
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

    #[test]
    fn test_part2() {
        let input = "Step C must be finished before step A can begin.
Step C must be finished before step F can begin.
Step A must be finished before step B can begin.
Step A must be finished before step D can begin.
Step B must be finished before step E can begin.
Step D must be finished before step E can begin.
Step F must be finished before step E can begin.";

        assert_eq!(("CABFDE".to_string(), 15), part2(input, 2, 0));
    }
}
