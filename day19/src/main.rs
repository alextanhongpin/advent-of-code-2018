mod opcode;
use opcode::*;

fn main() {
    let input = include_str!("input.txt");
    let mut program = Program::new(input);
    program.run();
    assert_eq!(1228, program.register[0]);
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
        let ip = lines
            .get(0)
            .unwrap()
            .replace("#ip", "")
            .trim()
            .parse::<usize>()
            .unwrap();
        Program {
            ip: 0,
            bound_ip: ip,
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
            let mut register = self.register;
            register[self.bound_ip] = self.ip;
            let (opcode, args) = self.instructions.get(self.ip).unwrap();

            let new_register = match opcode.as_str() {
                "addr" => addr(args, &register),
                "addi" => addi(args, &register),
                "mulr" => mulr(args, &register),
                "muli" => muli(args, &register),
                "banr" => banr(args, &register),
                "bani" => bani(args, &register),
                "borr" => borr(args, &register),
                "bori" => bori(args, &register),
                "setr" => setr(args, &register),
                "seti" => seti(args, &register),
                "gtir" => gtir(args, &register),
                "gtri" => gtri(args, &register),
                "gtrr" => gtrr(args, &register),
                "eqir" => eqir(args, &register),
                "eqri" => eqri(args, &register),
                "eqrr" => eqrr(args, &register),
                _ => panic!("Unknown opcode: {}", opcode),
            };
            self.register = new_register;
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
