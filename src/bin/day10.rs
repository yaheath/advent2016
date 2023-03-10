use std::collections::HashMap;
use std::str::FromStr;
use std::vec::Vec;
use advent_lib::read::read_input;

type BotNum = usize;
type ChipNum = usize;

#[derive(Copy,Clone)]
enum Dest {
    Output(usize),
    Bot(BotNum),
}
impl Dest {
    fn new(what: &str, val: &str) -> Self {
        let val = val.parse::<usize>().unwrap();
        match what {
            "bot" => Self::Bot(val),
            "output" => Self::Output(val),
            _ => panic!(),
        }
    }
}

enum Input {
    Bot(BotNum,Dest,Dest),
    Value(BotNum,ChipNum),
}

impl FromStr for Input {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let w: Vec<&str> = s.split_whitespace().collect();
        let bot = w[1].parse::<usize>().unwrap();
        match w[0] {
            "bot" => {
                let low = Dest::new(w[5], w[6]);
                let hi = Dest::new(w[10], w[11]);
                Ok(Input::Bot(bot,low,hi))
            },
            "value" => {
                let bot = w[5].parse::<usize>().unwrap();
                let dest = w[1].parse::<usize>().unwrap();
                Ok(Input::Value(bot,dest))
            }
            _ => Err(()),
        }
    }
}

struct Bot {
    item1: Option<ChipNum>,
    item2: Option<ChipNum>,
}
impl Bot {
    fn new(item: Option<ChipNum>) -> Self {
        Self{item1: item, item2: None}
    }
    fn is_full(&self) -> bool {
        self.item1.is_some() && self.item2.is_some()
    }
    fn take(&mut self, chip: ChipNum) -> bool {
        assert!(!self.is_full());
        if self.item1.is_none() {
            self.item1 = Some(chip);
            true
        }
        else if self.item2.is_none() {
            self.item2 = Some(chip);
            true
        }
        else {
            false
        }
    }
    fn drop_chips(&mut self) -> (ChipNum, ChipNum) {
        assert!(self.is_full());
        if *(self.item1.as_ref().unwrap()) < *(self.item2.as_ref().unwrap()) {
            (self.item1.take().unwrap(), self.item2.take().unwrap())
        }
        else {
            (self.item2.take().unwrap(), self.item1.take().unwrap())
        }
    }
}

enum StepResult {
    Ok,
    Break(BotNum),
    Deadlock,
}

struct BotField {
    bots: HashMap<BotNum, Bot>,
    rules: HashMap<BotNum, (Dest, Dest)>,
    outputs: HashMap<usize, Vec<ChipNum>>,
    breakpoint: Option<(ChipNum, ChipNum)>,
}
impl BotField {
    fn new(input: &Vec<Input>) -> Self {
        let mut bots: HashMap<BotNum, Bot> = HashMap::new();
        let mut rules: HashMap<BotNum, (Dest, Dest)> = HashMap::new();
        let outputs = HashMap::new();
        for i in input {
            match i {
                Input::Bot(bot, lowdest, hidest) => {
                    bots.entry(*bot)
                        .or_insert(Bot::new(None));
                    rules.insert(*bot, (*lowdest, *hidest));
                },
                Input::Value(bot, chip) => {
                    bots.entry(*bot)
                        .and_modify(|b| {b.take(*chip); })
                        .or_insert(Bot::new(Some(*chip)));
                },
            }
        }
        Self {
            bots,
            rules,
            outputs,
            breakpoint: None,
        }
    }
    fn set_breakpoint(&mut self, item1: ChipNum, item2: ChipNum) {
        self.breakpoint = Some((item1, item2));
    }
    fn step(&mut self) -> StepResult {
        let queued: Vec<BotNum> = self.bots.iter().filter(|(_,b)| b.is_full()).map(|(k,_)| *k).collect();
        if queued.len() == 0 {
            return StepResult::Deadlock;
        }
        let mut bp_bot: Option<BotNum> = None;
        for bot in queued {
            if let Some((bp_item1, bp_item2)) = self.breakpoint.as_ref() {
                let bot_item1 = *(self.bots[&bot].item1.as_ref().unwrap());
                let bot_item2 = *(self.bots[&bot].item2.as_ref().unwrap());
                if *bp_item1 == bot_item1 && *bp_item2 == bot_item2
                    || *bp_item1 == bot_item2 && *bp_item2 == bot_item1 {
                        bp_bot = Some(bot);
                }
            }
            let rule = self.rules[&bot];
            let (lowchip, highchip) = self.bots.get_mut(&bot).unwrap().drop_chips();
            match rule.0 {
                Dest::Bot(d) => {
                    self.bots.get_mut(&d).unwrap().take(lowchip);
                },
                Dest::Output(o) => {
                    self.outputs.entry(o)
                        .and_modify(|e| (*e).push(lowchip))
                        .or_insert(vec![lowchip]);
                },
            }
            match rule.1 {
                Dest::Bot(d) => {
                    self.bots.get_mut(&d).unwrap().take(highchip);
                },
                Dest::Output(o) => {
                    self.outputs.entry(o)
                        .and_modify(|e| (*e).push(highchip))
                        .or_insert(vec![highchip]);
                },
            }
        }
        if let Some(bp) = bp_bot {
            StepResult::Break(bp)
        }
        else {
            StepResult::Ok
        }
    }
}

fn process(input: &Vec<Input>, item1: ChipNum, item2: ChipNum) -> usize {
    let mut bots = BotField::new(input);
    bots.set_breakpoint(item1, item2);
    loop {
        match bots.step() {
            StepResult::Break(bot) => { return bot; },
            StepResult::Ok => {},
            StepResult::Deadlock => panic!(),
        }
    }
}

fn part1(input: &Vec<Input>) -> usize {
    process(input, 17, 61)
}

fn part2(input: &Vec<Input>) -> usize {
    let mut bots = BotField::new(input);
    loop {
        match bots.step() {
            StepResult::Deadlock => {break;},
            _ => {},
        }
    }
    bots.outputs[&0][0] *
    bots.outputs[&1][0] *
    bots.outputs[&2][0]
}

fn main() {
    let input: Vec<Input> = read_input();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;
    use advent_lib::read::test_input;

    #[test]
    fn day10_test() {
        let input: Vec<Input> = test_input(
            "value 5 goes to bot 2\n\
             bot 2 gives low to bot 1 and high to bot 0\n\
             value 3 goes to bot 1\n\
             bot 1 gives low to output 1 and high to bot 0\n\
             bot 0 gives low to output 2 and high to output 0\n\
             value 2 goes to bot 2\n"
        );
        assert_eq!(process(&input, 2, 5), 2);
    }
}
