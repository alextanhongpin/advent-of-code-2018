fn main() {
    let input = include_str!("input.txt");
    let sections = input.trim().split("\n\n\n").collect::<Vec<&str>>();
    assert_eq!(
        0,
        sections[0]
            .trim()
            .split("\n\n")
            .map(|input| count_opcode(&input))
            .filter(|&n| n >= 3)
            .count()
    );
}

fn count_opcode(input: &str) -> i32 {
    let program = parse(input);
    [
        is_addr, is_addi, is_mulr, is_muli, is_banr, is_bani, is_borr, is_bori, is_setr, is_seti,
        is_gtir, is_gtri, is_gtrr, is_eqir, is_eqri, is_eqrr,
    ]
    .into_iter()
    .map(|f| f(&program.0, &program.1, &program.2) as i32)
    .sum::<i32>()
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
        assert_eq!(3, count_opcode(input))
    }
}
