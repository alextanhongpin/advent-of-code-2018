pub fn addr(instruction: &[usize; 3], input: &[usize; 6]) -> [usize; 6] {
    let mut input = *input;
    input[instruction[2]] = input[instruction[0]] + input[instruction[1]];
    input
}

pub fn addi(instruction: &[usize; 3], input: &[usize; 6]) -> [usize; 6] {
    let mut input = *input;
    input[instruction[2]] = input[instruction[0]] + instruction[1];
    input
}

pub fn mulr(instruction: &[usize; 3], input: &[usize; 6]) -> [usize; 6] {
    let mut input = *input;
    input[instruction[2]] = input[instruction[0]] * input[instruction[1]];
    input
}

pub fn muli(instruction: &[usize; 3], input: &[usize; 6]) -> [usize; 6] {
    let mut input = *input;
    input[instruction[2]] = input[instruction[0]] * instruction[1];
    input
}

pub fn banr(instruction: &[usize; 3], input: &[usize; 6]) -> [usize; 6] {
    let mut input = *input;
    input[instruction[2]] = input[instruction[0]] & input[instruction[1]];
    input
}

pub fn bani(instruction: &[usize; 3], input: &[usize; 6]) -> [usize; 6] {
    let mut input = *input;
    input[instruction[2]] = input[instruction[0]] & instruction[1];
    input
}

pub fn borr(instruction: &[usize; 3], input: &[usize; 6]) -> [usize; 6] {
    let mut input = *input;
    input[instruction[2]] = input[instruction[0]] | input[instruction[1]];
    input
}

pub fn bori(instruction: &[usize; 3], input: &[usize; 6]) -> [usize; 6] {
    let mut input = *input;
    input[instruction[2]] = input[instruction[0]] | instruction[1];
    input
}

pub fn setr(instruction: &[usize; 3], input: &[usize; 6]) -> [usize; 6] {
    let mut input = *input;
    input[instruction[2]] = input[instruction[0]];
    input
}

pub fn seti(instruction: &[usize; 3], input: &[usize; 6]) -> [usize; 6] {
    let mut input = *input;
    input[instruction[2]] = instruction[0];
    input
}

pub fn gtir(instruction: &[usize; 3], input: &[usize; 6]) -> [usize; 6] {
    let mut input = *input;
    input[instruction[2]] = if instruction[0] > input[instruction[1]] {
        1
    } else {
        0
    };
    input
}

pub fn gtri(instruction: &[usize; 3], input: &[usize; 6]) -> [usize; 6] {
    let mut input = *input;
    input[instruction[2]] = if input[instruction[0]] > instruction[1] {
        1
    } else {
        0
    };
    input
}

pub fn gtrr(instruction: &[usize; 3], input: &[usize; 6]) -> [usize; 6] {
    let mut input = *input;
    input[instruction[2]] = if input[instruction[0]] > input[instruction[1]] {
        1
    } else {
        0
    };
    input
}

pub fn eqir(instruction: &[usize; 3], input: &[usize; 6]) -> [usize; 6] {
    let mut input = *input;
    input[instruction[2]] = if instruction[0] == input[instruction[1]] {
        1
    } else {
        0
    };
    input
}

pub fn eqri(instruction: &[usize; 3], input: &[usize; 6]) -> [usize; 6] {
    let mut input = *input;
    input[instruction[2]] = if input[instruction[0]] == instruction[1] {
        1
    } else {
        0
    };
    input
}

pub fn eqrr(instruction: &[usize; 3], input: &[usize; 6]) -> [usize; 6] {
    let mut input = *input;
    input[instruction[2]] = if input[instruction[0]] == input[instruction[1]] {
        1
    } else {
        0
    };
    input
}
