use std::collections::HashMap;
use std::str::FromStr;
use std::vec::Vec;
use advent_lib::read::read_input;

#[derive(Clone, Copy, Debug)]
enum RI {
    Reg(char),
    Imm(i64),
}

impl FromStr for RI {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let f = s.chars().next().unwrap();
        match f {
            'a' ..= 'z' => Ok(RI::Reg(f)),
            _ => Ok(RI::Imm(s.parse::<i64>().unwrap())),
        }
    }
}

#[derive(Debug)]
enum Instruction {
    Cpy(RI,char),
    Inc(char),
    Dec(char),
    Jnz(RI,RI),
}

impl FromStr for Instruction {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split(' ');
        let opcode = iter.next().ok_or(())?;
        let x = iter.next().ok_or(())?;
        let a = x.parse::<RI>()?;
        let x = x.chars().next().ok_or(())?;
        let y = iter.next();
        let b = y.map(|v| v.parse::<RI>().unwrap());
        let y = y.map(|v| v.chars().next().unwrap());
        match opcode {
            "cpy" => Ok(Instruction::Cpy(a, y.unwrap())),
            "inc" => Ok(Instruction::Inc(x)),
            "dec" => Ok(Instruction::Dec(x)),
            "jnz" => Ok(Instruction::Jnz(a, b.unwrap())),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Copy)]
enum RunResult {
    Ok,
    Halt,
}

struct VM<'a> {
    registers: HashMap<char, i64>,
    program: &'a Vec<Instruction>,
    pc: i64,
}
impl<'a> VM<'a> {
    fn new(program: &'a Vec<Instruction>, c: i64) -> Self {
        let registers = HashMap::from_iter([('c', c)]);
        Self { registers, program, pc: 0 }
    }
    fn step(&mut self) -> RunResult {
        if self.pc < 0 || self.pc >= self.program.len() as i64 {
            return RunResult::Halt;
        }
        let inst = &self.program[self.pc as usize];
        match inst {
            Instruction::Cpy(x, y) => {
                *self.registers.entry(*y).or_insert(0) = self.resolve(*x);
            },
            Instruction::Inc(x) => {
                self.registers.entry(*x).and_modify(|v| *v += 1).or_insert(1);
            },
            Instruction::Dec(x) => {
                self.registers.entry(*x).and_modify(|v| *v -= 1).or_insert(-1);
            },
            Instruction::Jnz(x, y) => {
                if self.resolve(*x) != 0 {
                    self.pc += self.resolve(*y) - 1;
                }
            },
        }
        self.pc += 1;
        if self.pc < 0 || self.pc >= self.program.len() as i64 {
            RunResult::Halt
        }
        else {
            RunResult::Ok
        }
    }
    fn resolve(&self, ri: RI) -> i64 {
        match ri {
            RI::Imm(x) => x,
            RI::Reg(r) => *self.registers.get(&r).unwrap_or(&0),
        }
    }
    fn run(&mut self) -> RunResult {
        loop {
            let r = self.step();
            match r {
                RunResult::Ok => {},
                _ => return r,
            }
        }
    }
}

fn part1(input: &Vec<Instruction>) -> i64 {
    let mut vm = VM::new(input, 0);
    vm.run();
    vm.registers[&'a']
}

fn part2(input: &Vec<Instruction>) -> i64 {
    let mut vm = VM::new(input, 1);
    vm.run();
    vm.registers[&'a']
}

fn main() {
    let input: Vec<Instruction> = read_input();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;
    use advent_lib::read::test_input;

    #[test]
    fn day12_test() {
        let input: Vec<Instruction> = test_input(
            "cpy 41 a\n\
             inc a\n\
             inc a\n\
             dec a\n\
             jnz a 2\n\
             dec a\n"
        );
        assert_eq!(part1(&input), 42);
    }
}
