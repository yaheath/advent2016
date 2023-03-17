use std::str::FromStr;
use std::vec::Vec;
use lazy_static::lazy_static;
use regex::Regex;
use advent_lib::read::read_input;

#[derive(Clone)]
struct Input {
    disk: u64,
    positions: u64,
    initial: u64,
}

impl FromStr for Input {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(
                r"#(\d+) has (\d+) .* position (\d+)"
            ).unwrap();
        }
        if let Some(caps) = RE.captures(s) {
            let disk:u64 = caps.get(1).unwrap().as_str().parse::<u64>().unwrap();
            let positions:u64 = caps.get(2).unwrap().as_str().parse::<u64>().unwrap();
            let initial:u64 = caps.get(3).unwrap().as_str().parse::<u64>().unwrap();
            Ok(Input {disk, positions, initial})
        }
        else {
            Err(())
        }
    }
}

fn solve(input: &Vec<Input>) -> u64 {
    for t in 0.. {
        if input.iter().all(|d|
            (d.disk + t + d.initial) % d.positions == 0
        ) {
            return t;
        }
    }
    panic!()
}

fn part1(input: &Vec<Input>) -> u64 {
    solve(input)
}

fn part2(input: &Vec<Input>) -> u64 {
    let mut input = input.clone();
    input.push(Input{disk: input.len() as u64 + 1, initial: 0, positions: 11});
    solve(&input)
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
    fn day15_test() {
        let input: Vec<Input> = test_input(
            "Disc #1 has 5 positions; at time=0, it is at position 4.\n\
             Disc #2 has 2 positions; at time=0, it is at position 1.\n"
        );
        assert_eq!(part1(&input), 5);
    }
}
