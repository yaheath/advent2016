use std::str::FromStr;
use std::vec::Vec;
use ya_advent_lib::read::read_input;
use ya_advent_lib::vm_shell::{CPU, VM, VMShell, InstructionResult};

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

#[derive(Debug, Clone)]
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

struct AssembunnyCPU { }
impl CPU<char, i64, Instruction> for AssembunnyCPU {
    fn execute_instruction(&self, vm: &mut VM<char, i64, Instruction>, i: &Instruction) -> InstructionResult {
        let resolve = |ri| {
            match ri {
                RI::Imm(x) => x,
                RI::Reg(r) => vm.get_reg(r),
            }
        };
        match i {
            Instruction::Cpy(x, y) => {
                vm.set_reg(*y, resolve(*x));
            },
            Instruction::Inc(x) => {
                let n = vm.get_reg(*x) + 1;
                vm.set_reg(*x, n);
            },
            Instruction::Dec(x) => {
                let n = vm.get_reg(*x) - 1;
                vm.set_reg(*x, n);
            },
            Instruction::Jnz(x, y) => {
                if resolve(*x) != 0 {
                    let jump = resolve(*y);
                    if jump < 0 {
                        return InstructionResult::JumpBck(jump.unsigned_abs() as usize);
                    }
                    else {
                        return InstructionResult::JumpFwd(jump as usize);
                    }
                }
            },
        }
        InstructionResult::Ok
    }
}

struct AssembunnyVM {
    cpu: AssembunnyCPU,
    shell: VMShell<char, i64, Instruction>,
}

impl AssembunnyVM {
    fn new(program: &[Instruction], c: i64) -> Self {
        let cpu = AssembunnyCPU{};
        let mut shell = VMShell::new(program.to_owned(), 0);
        shell.vm.set_reg('c', c);
        Self { cpu, shell }
    }
    fn run(&mut self) {
        self.shell.run(&self.cpu);
    }
}

fn part1(input: &[Instruction]) -> i64 {
    let mut vm = AssembunnyVM::new(input, 0);
    vm.run();
    vm.shell.vm.get_reg('a')
}

fn part2(input: &[Instruction]) -> i64 {
    let mut vm = AssembunnyVM::new(input, 1);
    vm.run();
    vm.shell.vm.get_reg('a')
}

fn main() {
    let input: Vec<Instruction> = read_input();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;
    use ya_advent_lib::read::test_input;

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
