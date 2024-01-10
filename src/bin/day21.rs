use std::collections::HashMap;
use std::str::FromStr;
use std::vec::Vec;
use ya_advent_lib::read::read_input;

enum Input {
    SwapPosition(usize,usize),
    SwapLetter(char,char),
    RotateLeft(usize),
    RotateRight(usize),
    RotateByPos(char),
    Reverse(usize,usize),
    Move(usize,usize),
}

impl FromStr for Input {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let words: Vec<&str> = s.split(' ').collect();
        match (words[0], words[1]) {
            ("swap", "position") => {Ok(Input::SwapPosition(
                        words[2].parse::<usize>().unwrap(),
                        words[5].parse::<usize>().unwrap(),
            ))},
            ("swap", "letter") => {Ok(Input::SwapLetter(
                        words[2].chars().next().unwrap(),
                        words[5].chars().next().unwrap(),
            ))},
            ("rotate", "left") => {Ok(Input::RotateLeft(
                        words[2].parse::<usize>().unwrap(),
            ))},
            ("rotate", "right") => {Ok(Input::RotateRight(
                        words[2].parse::<usize>().unwrap(),
            ))},
            ("rotate", "based") => {Ok(Input::RotateByPos(
                        words[6].chars().next().unwrap(),
            ))},
            ("reverse", "positions") => {Ok(Input::Reverse(
                        words[2].parse::<usize>().unwrap(),
                        words[4].parse::<usize>().unwrap(),
            ))},
            ("move", "position") => {Ok(Input::Move(
                        words[2].parse::<usize>().unwrap(),
                        words[5].parse::<usize>().unwrap(),
            ))},
            _ => Err(()),
        }
    }
}

struct Password {
    chars: Vec<char>,
    reverse: bool,
    revmap: HashMap<usize,usize>,
}
impl Password {
    fn new(s: &str, reverse: bool) -> Self {
        let revmap: HashMap<usize,usize> = if reverse {
            HashMap::from_iter(
                (0..s.len())
                .map(|x| {
                    let y = if x >= 4 { x + 2 } else { x + 1 };
                    ((x + y) % s.len(), y % s.len())
                })
            )
        }
        else {
            HashMap::new()
        };
        Self {chars: s.chars().collect(), reverse, revmap}
    }
    fn apply(&mut self, instruction: &Input) {
        match instruction {
            Input::SwapPosition(p1, p2) => {
                self.chars.swap(*p1, *p2);
            },
            Input::SwapLetter(c1, c2) => {
                for i in 0..self.chars.len() {
                    if self.chars[i] == *c1 {
                        self.chars[i] = *c2;
                    } else if self.chars[i] == *c2 {
                        self.chars[i] = *c1;
                    }
                }
            },
            Input::RotateLeft(n) => {
                if self.reverse {
                    self.chars.rotate_right(*n);
                }
                else {
                    self.chars.rotate_left(*n);
                }
            },
            Input::RotateRight(n) => {
                if self.reverse {
                    self.chars.rotate_left(*n);
                }
                else {
                    self.chars.rotate_right(*n);
                }
            },
            Input::RotateByPos(c) => {
                let mut n = self.chars.iter().position(|&x| x == *c).unwrap();
                if self.reverse {
                    self.chars.rotate_left(self.revmap[&n]);
                }
                else {
                    if n >= 4 { n += 1 }
                    n += 1;
                    n %= self.chars.len();
                    self.chars.rotate_right(n);
                }
            },
            Input::Reverse(p1, p2) => {
                self.chars[*p1..=*p2].reverse();
            },
            Input::Move(p1, p2) => {
                let (frm, to) = if self.reverse { (*p2, *p1) } else { (*p1, *p2) };
                let c = self.chars.splice(frm..=frm, []).next().unwrap();
                self.chars.insert(to, c);
            }
        }
    }
    fn to_str(&self) -> String {
        self.chars.iter().collect()
    }
}

fn solve(input: &[Input], initial: &str, reverse: bool) -> String {
    let mut pw = Password::new(initial, reverse);
    if reverse {
        input.iter().rev().for_each(|i| pw.apply(i));
    }
    else {
        input.iter().for_each(|i| pw.apply(i));
    }
    pw.to_str()
}

fn part1(input: &[Input]) -> String {
    solve(input, "abcdefgh", false)
}

fn part2(input: &[Input]) -> String {
    solve(input, "fbgdceah", true)
}

fn main() {
    let input: Vec<Input> = read_input();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;
    use ya_advent_lib::read::test_input;

    #[test]
    fn day21_test() {
        let input: Vec<Input> = test_input(include_str!("day21.testinput"));
        assert_eq!(solve(&input, "abcde", false), "decab".to_string());
        assert_eq!(solve(&input, "decab", true), "abcde".to_string());
    }
}
