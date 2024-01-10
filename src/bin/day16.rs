use std::vec::Vec;
use itertools::Itertools;
use ya_advent_lib::read::read_input;

fn solve(input: &str, len: usize) -> String {
    let mut s = input.to_string();
    while s.len() < len {
        let rev:String = s.chars().rev().map(|c| if c == '0' { '1' } else { '0' }).collect();
        s = format!("{s}0{rev}");
    }
    s.truncate(len);
    while s.len() & 1 == 0 {
        s = s.chars().batching(|itr|
                match itr.next() {
                    None => None,
                    Some(a) => match itr.next() {
                        None => panic!(),
                        Some(b) => Some(if a == b { '1' } else { '0' }),
                    },
                }
            ).collect();
    }
    s
}

fn part1(input: &[String]) -> String {
    solve(&input[0], 272)
}

fn part2(input: &[String]) -> String {
    solve(&input[0], 35651584)
}

fn main() {
    let input: Vec<String> = read_input();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day16_test() {
        assert_eq!(solve("10000", 20), "01100".to_string());
    }
}
