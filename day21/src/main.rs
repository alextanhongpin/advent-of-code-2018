use std::collections::HashSet;
mod opcode;
use opcode::*;

fn main() {
    let input = include_str!("input.txt");
    let mut program = Program::new(input);
    program.run();
    println!("{:?}", program.register);
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
        let mut seen = HashSet::new();
        let mut iter = 0;
        while self.ip < self.instructions.len() {
            iter += 1;
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

            // This is the register that will cause the program to terminate.
            // eqrr 3 0 5
            if self.register[self.bound_ip] == 28 {
                if seen.contains(&self.register[args[0]]) {
                    println!("part2: {:?}", seen.iter().max().unwrap());
                    break;
                }
                seen.insert(self.register[args[0]]);
                //println!("part 1: {:?}", self.register[3] );
                //break;
            }
            //println!("{:?} {:?} {:?} {:?}", opcode, args, self.ip, self.register);
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
