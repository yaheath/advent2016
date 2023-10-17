use std::str::FromStr;
use std::vec::Vec;
use advent_lib::read::read_input;
use advent_lib::vm_shell::{CPU, VM, VMShell, InstructionResult};
use advent_lib::vm_display::{InstructionDisplay, Formatter};
use advent_lib::vm_debugger::Debugger;

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
impl InstructionDisplay<i64> for RI {
    fn fmt(&self, fmt: &mut Formatter<i64>) {
        match self {
            RI::Reg(c) => fmt.add_register(c.to_string()),
            RI::Imm(i) => fmt.add_integer(*i),
        };
    }
}

#[derive(Debug, Clone)]
enum Instruction {
    Cpy(RI,char),
    InvCpy(RI,RI),
    Inc(char),
    InvInc(RI),
    Dec(char),
    InvDec(RI),
    Jnz(RI,RI),
    Tgl(RI),
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
            "tgl" => Ok(Instruction::Tgl(a)),
            _ => Err(()),
        }
    }
}
impl InstructionDisplay<i64> for Instruction {
    fn fmt(&self, fmt: &mut Formatter<i64>) {
        match self {
            Instruction::Cpy(x,y) => {
                fmt.add_opcode("cpy".into());
                x.fmt(fmt);
                fmt.add_register(y.to_string());
            },
            Instruction::InvCpy(x,y) => {
                fmt.add_opcode("*cpy".into());
                x.fmt(fmt);
                y.fmt(fmt);
            },
            Instruction::Inc(x) => {
                fmt.add_opcode("inc".into());
                fmt.add_register(x.to_string());
            },
            Instruction::InvInc(x) => {
                fmt.add_opcode("inc".into());
                x.fmt(fmt);
            },
            Instruction::Dec(x) => {
                fmt.add_opcode("dec".into());
                fmt.add_register(x.to_string());
            },
            Instruction::InvDec(x) => {
                fmt.add_opcode("dec".into());
                x.fmt(fmt);
            },
            Instruction::Jnz(x,y) => {
                fmt.add_opcode("jnz".into());
                x.fmt(fmt);
                y.fmt(fmt);
            },
            Instruction::Tgl(x) => {
                fmt.add_opcode("tgl".into());
                x.fmt(fmt);
            },
        };
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
                        return InstructionResult::JumpBck(jump.abs() as usize);
                    }
                    else {
                        return InstructionResult::JumpFwd(jump as usize);
                    }
                }
            },
            Instruction::Tgl(x) => {
                let loc = vm.pc as i64 + resolve(*x);
                if loc >= 0 && loc < vm.program.len() as i64 {
                    let ninst = match vm.program[loc as usize] {
                        Instruction::Inc(x) => Instruction::Dec(x),
                        Instruction::InvInc(x) => Instruction::InvDec(x),
                        Instruction::Dec(x) => Instruction::Inc(x),
                        Instruction::InvDec(x) => Instruction::InvInc(x),
                        Instruction::Tgl(RI::Reg(x)) => Instruction::Inc(x),
                        Instruction::Tgl(RI::Imm(x)) => Instruction::InvInc(RI::Imm(x)),
                        Instruction::Jnz(x, RI::Reg(y)) => Instruction::Cpy(x, y),
                        Instruction::Jnz(x, RI::Imm(y)) => Instruction::InvCpy(x, RI::Imm(y)),
                        Instruction::Cpy(x, y) => Instruction::Jnz(x, RI::Reg(y)),
                        Instruction::InvCpy(x, y) => Instruction::Jnz(x, y),
                    };
                    vm.program[loc as usize] = ninst;
                }
            }
            _ => {},
        }
        InstructionResult::Ok
    }
}

struct AssembunnyVM {
    cpu: AssembunnyCPU,
    shell: VMShell<char, i64, Instruction>,
}

impl AssembunnyVM {
    fn new(program: &Vec<Instruction>, a: i64) -> Self {
        let cpu = AssembunnyCPU{};
        let mut shell = VMShell::new(program.clone(), 0);
        shell.vm.set_reg('a', a);
        Self { cpu, shell }
    }
    fn run(&mut self) {
        self.shell.run(&self.cpu);
    }
}

fn part1(input: &Vec<Instruction>) -> i64 {
    let mut vm = AssembunnyVM::new(input, 7);
    //vm.run();
    let _ = Debugger::run(&mut vm.shell, &vm.cpu);
    vm.shell.vm.get_reg('a')
}

fn part2(input: &Vec<Instruction>) -> i64 {
    let mut vm = AssembunnyVM::new(input, 12);
    //vm.run();
    let _ = Debugger::run(&mut vm.shell, &vm.cpu);
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
    use advent_lib::read::test_input;

    #[test]
    fn day23_test() {
        let input: Vec<Instruction> = test_input(
            "cpy 2 a\n\
             tgl a\n\
             tgl a\n\
             tgl a\n\
             cpy 1 a\n\
             dec a\n\
             dec a\n"
        );
        assert_eq!(part1(&input), 3);
    }
}
