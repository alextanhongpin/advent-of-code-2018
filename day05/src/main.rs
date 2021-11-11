use std::collections::HashSet;
fn main() {
    let input = include_str!("input.txt");
    assert_eq!(10450, reaction(&input));
    assert_eq!(4624, smarter_reaction(&input));
}

fn smarter_reaction(input: &str) -> usize {
    let unique_units: HashSet<char> = HashSet::from_iter(input.trim().to_lowercase().chars());
    let mut min_length = input.len();
    for unit in unique_units {
        let input = input
            .replace(unit, "")
            .replace(unit.to_ascii_uppercase(), "");
        let output = reaction(&input);
        if output < min_length {
            min_length = output;
        }
    }

    min_length
}

fn reaction(input: &str) -> usize {
    let mut input = input.trim().chars().collect::<Vec<_>>();
    let mut stack: Vec<char> = Vec::new();

    loop {
        match input.pop() {
            Some(c) => match stack.pop() {
                Some(s) => {
                    if s.to_ascii_lowercase() == c.to_ascii_lowercase() && s != c {
                        continue;
                    }
                    stack.push(s);
                    stack.push(c);
                }
                None => {
                    stack.push(c);
                }
            },
            None => break,
        }
    }

    stack.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "dabAcCaCBAcCcaDA";
        let result = reaction(input);
        assert_eq!(10, result);
    }
}
