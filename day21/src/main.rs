const MAGIC_IP: usize = 28;

fn main() {
    let input = include_str!("input.txt");
    let mut program = Program::new(input);
    program.run();
    assert_eq!(16134795, program.history[0]);
    assert_eq!(Some(&14254292), program.history.last());
}

#[derive(Debug)]
struct Program {
    bound_ip: usize,
    ip: usize,
    register: [usize; 6],
    instructions: Vec<(String, [usize; 3])>,
    history: Vec<usize>,
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
            history: Vec::new(),
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
        'outer: while self.ip < self.instructions.len() {
            self.register[self.bound_ip] = self.ip;
            let (opcode, [a, b, c]) = self.instructions[self.ip].clone();

            // This is the register that will cause the program to terminate.
            // eqrr 3 0 5
            if self.ip == MAGIC_IP {
                if self.history.contains(&self.register[a]) {
                    break 'outer;
                }
                self.history.push(self.register[a]);
            }

            let reg = self.register;
            self.register[c] = match opcode.as_str() {
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
            self.ip = self.register[self.bound_ip] + 1;
        }
    }
}
