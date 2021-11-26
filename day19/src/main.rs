fn main() {
    let input = include_str!("input.txt");
    let mut program = Program::new(input);
    program.run();
    assert_eq!(1228, program.register[0]);

    // Not sure how this relates to part 2, but it works.
    let p = 10551267;
    let result = p + (1..=p / 2).filter(|x| p % x == 0).sum::<u32>();
    assert_eq!(15285504, result);
}

#[derive(Debug)]
struct Program {
    bound_ip: usize,
    ip: usize,
    register: [usize; 6],
    instructions: Vec<(String, [usize; 3])>,
}

impl Program {
    fn new(input: &str) -> Self {
        let lines = input.trim().lines().collect::<Vec<&str>>();
        Program {
            ip: 0,
            bound_ip: lines
                .get(0)
                .unwrap()
                .replace("#ip", "")
                .trim()
                .parse::<usize>()
                .unwrap(),
            register: [0; 6],
            instructions: lines
                .into_iter()
                .skip(1)
                .map(|line| {
                    let columns = line
                        .split_whitespace()
                        .map(str::trim)
                        .collect::<Vec<&str>>();
                    (
                        columns.get(0).unwrap().to_string(),
                        columns
                            .into_iter()
                            .skip(1)
                            .map(|col| col.trim().parse::<usize>().unwrap())
                            .collect::<Vec<usize>>()
                            .try_into()
                            .unwrap(),
                    )
                })
                .collect(),
        }
    }

    fn run(&mut self) {
        while self.ip < self.instructions.len() {
            let mut reg = self.register;
            reg[self.bound_ip] = self.ip;
            let (opcode, [a, b, c]) = self.instructions.get(self.ip).unwrap().clone();

            reg[c] = match opcode.as_str() {
                "addr" => reg[a] + reg[b],
                "addi" => reg[a] + b,
                "mulr" => reg[a] * reg[b],
                "muli" => reg[a] * b,
                "banr" => reg[a] & reg[b],
                "bani" => reg[a] & b,
                "borr" => reg[a] | reg[b],
                "bori" => reg[a] | b,
                "setr" => reg[a],
                "seti" => a,
                "gtir" => (a > reg[b]) as usize,
                "gtri" => (reg[a] > b) as usize,
                "gtrr" => (reg[a] > reg[b]) as usize,
                "eqir" => (a == reg[b]) as usize,
                "eqri" => (reg[a] == b) as usize,
                "eqrr" => (reg[a] == reg[b]) as usize,
                _ => panic!("Unknown opcode: {}", opcode),
            };
            self.register = reg;
            self.ip = self.register[self.bound_ip] + 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "#ip 0
seti 5 0 1
seti 6 0 2
addi 0 1 0
addr 1 2 3
setr 1 0 0
seti 8 0 4
seti 9 0 5";
        let mut program = Program::new(input);
        program.run();
        assert_eq!(6, program.register[0]);
    }
}
