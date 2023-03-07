use std::str::FromStr;
use std::vec::Vec;
use advent_lib::read::read_input;
use advent_lib::coords::CDir;

struct Input {
    list: Vec<CDir>,
}

impl FromStr for Input {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let list: Vec<CDir> = s.chars()
            .map(|c| match c {
                'U' => CDir::N,
                'D' => CDir::S,
                'R' => CDir::E,
                'L' => CDir::W,
                _ => panic!(),
            })
            .collect();
        Ok(Input{list})
    }
}

fn next_digit(d: char, dir: CDir) -> char {
    match (d, dir) {
        ('1', CDir::E) => '2',
        ('1', CDir::S) => '4',
        ('2', CDir::E) => '3',
        ('2', CDir::W) => '1',
        ('2', CDir::S) => '5',
        ('3', CDir::W) => '2',
        ('3', CDir::S) => '6',
        ('4', CDir::N) => '1',
        ('4', CDir::E) => '5',
        ('4', CDir::S) => '7',
        ('5', CDir::N) => '2',
        ('5', CDir::E) => '6',
        ('5', CDir::S) => '8',
        ('5', CDir::W) => '4',
        ('6', CDir::N) => '3',
        ('6', CDir::W) => '5',
        ('6', CDir::S) => '9',
        ('7', CDir::N) => '4',
        ('7', CDir::E) => '8',
        ('8', CDir::N) => '5',
        ('8', CDir::E) => '9',
        ('8', CDir::W) => '7',
        ('9', CDir::N) => '6',
        ('9', CDir::W) => '8',
        _ => d,
    }
}

fn part1(input: &Vec<Input>) -> String {
    let mut out = String::new();
    let mut prev_digit = '5';
    for row in input {
        let digit = row.list.iter()
            .fold(prev_digit, |d, dir| next_digit(d, *dir));
        out.push(digit);
        prev_digit = digit;
    }
    out
}

fn next_digit2(d: char, dir: CDir) -> char {
    match (d, dir) {
        ('1', CDir::S) => '3',
        ('2', CDir::E) => '3',
        ('2', CDir::S) => '6',
        ('3', CDir::N) => '1',
        ('3', CDir::E) => '4',
        ('3', CDir::S) => '7',
        ('3', CDir::W) => '2',
        ('4', CDir::W) => '3',
        ('4', CDir::S) => '8',
        ('5', CDir::E) => '6',
        ('6', CDir::N) => '2',
        ('6', CDir::E) => '7',
        ('6', CDir::S) => 'A',
        ('6', CDir::W) => '5',
        ('7', CDir::N) => '3',
        ('7', CDir::E) => '8',
        ('7', CDir::S) => 'B',
        ('7', CDir::W) => '6',
        ('8', CDir::N) => '4',
        ('8', CDir::E) => '9',
        ('8', CDir::S) => 'C',
        ('8', CDir::W) => '7',
        ('9', CDir::W) => '8',
        ('A', CDir::N) => '6',
        ('A', CDir::E) => 'B',
        ('B', CDir::N) => '7',
        ('B', CDir::E) => 'C',
        ('B', CDir::S) => 'D',
        ('B', CDir::W) => 'A',
        ('C', CDir::N) => '8',
        ('C', CDir::W) => 'B',
        ('D', CDir::N) => 'B',
        _ => d,
    }
}

fn part2(input: &Vec<Input>) -> String {
    let mut out = String::new();
    let mut prev_digit = '5';
    for row in input {
        let digit = row.list.iter()
            .fold(prev_digit, |d, dir| next_digit2(d, *dir));
        out.push(digit);
        prev_digit = digit;
    }
    out
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
    fn day02_test() {
        let input: Vec<Input> = test_input(include_str!("day02.testinput"));
        assert_eq!(part1(&input), "1985".to_string());
        assert_eq!(part2(&input), "5DB3".to_string());
    }
}
