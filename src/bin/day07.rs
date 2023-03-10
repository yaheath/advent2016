use std::str::FromStr;
use std::vec::Vec;
use itertools::Itertools;
use advent_lib::read::read_input;

struct Input {
    parts: Vec<String>,
    hypernets: Vec<String>,
}

impl FromStr for Input {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut itr = s.chars();
        let mut seq = String::new();
        let mut parts = Vec::new();
        let mut hypernets = Vec::new();
        while let Some(c) = itr.next() {
            match c {
                '[' => {
                    parts.push(seq);
                    seq = String::new();
                },
                ']' => {
                    hypernets.push(seq);
                    seq = String::new();
                },
                c => { seq.push(c); },
            }
        }
        if !seq.is_empty() {
            parts.push(seq);
        }
        Ok(Input{parts, hypernets})
    }
}

impl Input {
    fn supports_tls(&self) -> bool {
        if self.hypernets.iter().any(|s| Self::has_abba(s)) {
            return false;
        }
        self.parts.iter().any(|s| Self::has_abba(s))
    }
    fn supports_ssl(&self) -> bool {
        self.parts.iter()
            .any(|s| s.chars()
                .tuple_windows()
                .any(|(a,b,c)| a == c && a != b && self.has_bab(a, b))
            )
    }
    fn has_abba(s: &str) -> bool {
        s.chars()
            .tuple_windows()
            .any(|(a,b,c,d)| a == d && b == c && a != b)
    }
    fn has_bab(&self, a: char, b: char) -> bool {
        let bab = String::from_iter([b, a, b]);
        self.hypernets.iter().any(|s| s.contains(&bab))
    }
}

fn part1(input: &Vec<Input>) -> usize {
    input.iter().filter(|row| row.supports_tls()).count()
}

fn part2(input: &Vec<Input>) -> usize {
    input.iter().filter(|row| row.supports_ssl()).count()
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
    fn day07_test() {
        let input: Vec<Input> = test_input(
            "abba[mnop]qrst\n\
             abcd[bddb]xyyx\n\
             aaaa[qwer]tyui\n\
             ioxxoj[asdfgh]zxcvbn\n"
        );
        assert_eq!(part1(&input), 2);
        let input: Vec<Input> = test_input(
            "aba[bab]xyz\n\
             xyx[xyx]xyx\n\
             aaa[kek]eke\n\
             zazbz[bzb]cdb\n"
        );
        assert_eq!(part2(&input), 3);
    }
}
