use std::collections::{HashMap, HashSet};
fn main() {
    let input = include_str!("input.txt");
    let sections = input.trim().split("\n\n\n").collect::<Vec<&str>>();
    assert_eq!(
        614,
        sections[0]
            .trim()
            .split("\n\n")
            .map(|input| get_possible_opcode(input).len())
            .filter(|&n| n >= 3)
            .count()
    );

    let mut possible_opcodes = HashMap::new();
    let mut unions = HashMap::new();
    for line in sections[0].trim().split("\n\n") {
        let possible_opcode = get_possible_opcode(line);
        for (i, opcode) in possible_opcode {
            let mut curr = HashSet::new();
            curr.insert(i);

            let union_opcodes = unions.entry(opcode).or_insert_with(HashSet::new);
            *union_opcodes = union_opcodes.union(&curr).cloned().collect();
        }
    }
    // Create a new map to hold only unique opcodes.
    while possible_opcodes.len() != 16 {
        for opcode in 0..16 {
            if !unions.contains_key(&opcode) {
                continue;
            }
            let indices = unions[&opcode].clone();
            for idx in indices.iter() {
                if possible_opcodes.contains_key(idx) {
                    unions.get_mut(&opcode).unwrap().remove(idx);
                }
            }
            let indices = unions[&opcode].clone();
            if indices.len() == 1 {
                possible_opcodes.insert(indices.into_iter().last().unwrap(), opcode);
                unions.remove(&opcode);
            }
        }
    }
    let mut index_by_opcode = HashMap::new();
    for (k, v) in possible_opcodes.iter() {
        index_by_opcode.insert(v, k);
    }
    let ops = [
        addr, addi, mulr, muli, banr, bani, borr, bori, setr, seti, gtir, gtri, gtrr, eqir, eqri,
        eqrr,
    ];
    let mut registers = [0; 4];

    for line in sections[1].trim().lines() {
        let instruction: [usize; 4] = line
            .trim()
            .split_whitespace()
            .map(|n| n.trim().parse::<usize>().unwrap())
            .collect::<Vec<usize>>()
            .try_into()
            .unwrap();
        let opcode = instruction[0];
        let op = ops[*index_by_opcode[&opcode]];
        registers = op(&instruction, &registers);
    }
    assert_eq!(656, registers[0]);
}

fn get_possible_opcode(input: &str) -> Vec<(usize, usize)> {
    let program = parse(input);
    [
        is_addr, is_addi, is_mulr, is_muli, is_banr, is_bani, is_borr, is_bori, is_setr, is_seti,
        is_gtir, is_gtri, is_gtrr, is_eqir, is_eqri, is_eqrr,
    ]
    .into_iter()
    .enumerate()
    .map(|(i, f)| {
        if f(&program.0, &program.1, &program.2) {
            Some((i, program.0[0]))
        } else {
            None
        }
    })
    .flatten()
    .collect()
}

fn parse(input: &str) -> ([usize; 4], [usize; 4], [usize; 4]) {
    let mut input = input.trim().lines();
    let before = input
        .next()
        .unwrap()
        .trim()
        .replace("Before: [", "")
        .replace("]", "")
        .split(',')
        .map(|n| n.trim().parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    let instruction = input
        .next()
        .unwrap()
        .trim()
        .split_whitespace()
        .map(|n| n.trim().parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    let after = input
        .next()
        .unwrap()
        .trim()
        .replace("After:  [", "")
        .replace("]", "")
        .split(',')
        .map(|n| n.trim().parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    (
        instruction.try_into().unwrap(),
        before.try_into().unwrap(),
        after.try_into().unwrap(),
    )
}

fn is_addr(instruction: &[usize; 4], input: &[usize; 4], output: &[usize; 4]) -> bool {
    input[instruction[1]] + input[instruction[2]] == output[instruction[3]]
}

fn is_addi(instruction: &[usize; 4], input: &[usize; 4], output: &[usize; 4]) -> bool {
    input[instruction[1]] + instruction[2] == output[instruction[3]]
}

fn is_mulr(instruction: &[usize; 4], input: &[usize; 4], output: &[usize; 4]) -> bool {
    input[instruction[1]] * input[instruction[2]] == output[instruction[3]]
}

fn is_muli(instruction: &[usize; 4], input: &[usize; 4], output: &[usize; 4]) -> bool {
    input[instruction[1]] * instruction[2] == output[instruction[3]]
}

fn is_banr(instruction: &[usize; 4], input: &[usize; 4], output: &[usize; 4]) -> bool {
    input[instruction[1]] & input[instruction[2]] == output[instruction[3]]
}

fn is_bani(instruction: &[usize; 4], input: &[usize; 4], output: &[usize; 4]) -> bool {
    input[instruction[1]] & instruction[2] == output[instruction[3]]
}

fn is_borr(instruction: &[usize; 4], input: &[usize; 4], output: &[usize; 4]) -> bool {
    input[instruction[1]] | input[instruction[2]] == output[instruction[3]]
}

fn is_bori(instruction: &[usize; 4], input: &[usize; 4], output: &[usize; 4]) -> bool {
    input[instruction[1]] | instruction[2] == output[instruction[3]]
}

fn is_setr(instruction: &[usize; 4], input: &[usize; 4], output: &[usize; 4]) -> bool {
    input[instruction[1]] == output[instruction[3]]
}

fn is_seti(instruction: &[usize; 4], input: &[usize; 4], output: &[usize; 4]) -> bool {
    instruction[1] == output[instruction[3]]
}

fn is_gtir(instruction: &[usize; 4], input: &[usize; 4], output: &[usize; 4]) -> bool {
    (instruction[1] > input[instruction[2]]) && (output[instruction[3]] == 1)
        || ((instruction[1] <= input[instruction[2]]) && (output[instruction[3]] == 0))
}

fn is_gtri(instruction: &[usize; 4], input: &[usize; 4], output: &[usize; 4]) -> bool {
    (input[instruction[1]] > instruction[2]) && (output[instruction[3]] == 1)
        || ((input[instruction[1]] <= instruction[2]) && (output[instruction[3]] == 0))
}

fn is_gtrr(instruction: &[usize; 4], input: &[usize; 4], output: &[usize; 4]) -> bool {
    (input[instruction[1]] > input[instruction[2]]) && (output[instruction[3]] == 1)
        || ((input[instruction[1]] <= input[instruction[2]]) && (output[instruction[3]] == 0))
}

fn is_eqir(instruction: &[usize; 4], input: &[usize; 4], output: &[usize; 4]) -> bool {
    (instruction[1] == input[instruction[2]]) && (output[instruction[3]] == 1)
        || ((instruction[1] != input[instruction[2]]) && (output[instruction[3]] == 0))
}

fn is_eqri(instruction: &[usize; 4], input: &[usize; 4], output: &[usize; 4]) -> bool {
    (input[instruction[1]] == instruction[2]) && (output[instruction[3]] == 1)
        || ((input[instruction[1]] != instruction[2]) && (output[instruction[3]] == 0))
}

fn is_eqrr(instruction: &[usize; 4], input: &[usize; 4], output: &[usize; 4]) -> bool {
    (input[instruction[1]] == input[instruction[2]]) && (output[instruction[3]] == 1)
        || ((input[instruction[1]] != input[instruction[2]]) && (output[instruction[3]] == 0))
}

fn addr(instruction: &[usize; 4], input: &[usize; 4]) -> [usize; 4] {
    let mut input = *input;
    input[instruction[3]] = input[instruction[1]] + input[instruction[2]];
    input
}

fn addi(instruction: &[usize; 4], input: &[usize; 4]) -> [usize; 4] {
    let mut input = *input;
    input[instruction[3]] = input[instruction[1]] + instruction[2];
    input
}

fn mulr(instruction: &[usize; 4], input: &[usize; 4]) -> [usize; 4] {
    let mut input = *input;
    input[instruction[3]] = input[instruction[1]] * input[instruction[2]];
    input
}

fn muli(instruction: &[usize; 4], input: &[usize; 4]) -> [usize; 4] {
    let mut input = *input;
    input[instruction[3]] = input[instruction[1]] * instruction[2];
    input
}

fn banr(instruction: &[usize; 4], input: &[usize; 4]) -> [usize; 4] {
    let mut input = *input;
    input[instruction[3]] = input[instruction[1]] & input[instruction[2]];
    input
}

fn bani(instruction: &[usize; 4], input: &[usize; 4]) -> [usize; 4] {
    let mut input = *input;
    input[instruction[3]] = input[instruction[1]] & instruction[2];
    input
}

fn borr(instruction: &[usize; 4], input: &[usize; 4]) -> [usize; 4] {
    let mut input = *input;
    input[instruction[3]] = input[instruction[1]] | input[instruction[2]];
    input
}

fn bori(instruction: &[usize; 4], input: &[usize; 4]) -> [usize; 4] {
    let mut input = *input;
    input[instruction[3]] = input[instruction[1]] | instruction[2];
    input
}

fn setr(instruction: &[usize; 4], input: &[usize; 4]) -> [usize; 4] {
    let mut input = *input;
    input[instruction[3]] = input[instruction[1]];
    input
}

fn seti(instruction: &[usize; 4], input: &[usize; 4]) -> [usize; 4] {
    let mut input = *input;
    input[instruction[3]] = instruction[1];
    input
}

fn gtir(instruction: &[usize; 4], input: &[usize; 4]) -> [usize; 4] {
    let mut input = *input;
    input[instruction[3]] = if instruction[1] > input[instruction[2]] {
        1
    } else {
        0
    };
    input
}

fn gtri(instruction: &[usize; 4], input: &[usize; 4]) -> [usize; 4] {
    let mut input = *input;
    input[instruction[3]] = if input[instruction[1]] > instruction[2] {
        1
    } else {
        0
    };
    input
}

fn gtrr(instruction: &[usize; 4], input: &[usize; 4]) -> [usize; 4] {
    let mut input = *input;
    input[instruction[3]] = if input[instruction[1]] > input[instruction[2]] {
        1
    } else {
        0
    };
    input
}

fn eqir(instruction: &[usize; 4], input: &[usize; 4]) -> [usize; 4] {
    let mut input = *input;
    input[instruction[3]] = if instruction[1] == input[instruction[2]] {
        1
    } else {
        0
    };
    input
}

fn eqri(instruction: &[usize; 4], input: &[usize; 4]) -> [usize; 4] {
    let mut input = *input;
    input[instruction[3]] = if input[instruction[1]] == instruction[2] {
        1
    } else {
        0
    };
    input
}

fn eqrr(instruction: &[usize; 4], input: &[usize; 4]) -> [usize; 4] {
    let mut input = *input;
    input[instruction[3]] = if input[instruction[1]] == input[instruction[2]] {
        1
    } else {
        0
    };
    input
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "Before: [3, 2, 1, 1]
9 2 1 2
After:  [3, 2, 2, 1]";
        let result = parse(&input);
        assert_eq!(([9, 2, 1, 2], [3, 2, 1, 1], [3, 2, 2, 1]), result);
        assert_eq!(true, is_addi(&result.0, &result.1, &result.2));
        assert_eq!(true, is_mulr(&result.0, &result.1, &result.2));
        assert_eq!(true, is_seti(&result.0, &result.1, &result.2));
        assert_eq!(3, get_possible_opcode(input))
    }
}
