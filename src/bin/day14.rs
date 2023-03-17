use std::collections::VecDeque;
use std::vec::Vec;
use itertools::Itertools;
use advent_lib::read::read_input;

fn search(salt: &str, stretch: usize) -> usize {
    let mut triples: VecDeque<(usize, char)> = VecDeque::new();
    let mut keys: Vec<(usize, char)> = Vec::new();
    let mut fives: VecDeque<(usize, char)> =VecDeque::new();

    for idx in 0.. {
        let s = format!("{salt}{idx}");
        let mut hash = format!("{:x}", md5::compute(s.as_bytes()));
        for _ in 0..stretch {
            hash = format!("{:x}", md5::compute(hash.as_bytes()));
        }
        if let Some(trip_c) = hash.chars()
            .tuple_windows()
            .find(|(a,b,c)| a == b && b == c)
            .map(|(a,_,_)| a)
        {
            triples.push_back((idx, trip_c));
            if let Some(five_c) = hash.chars()
                .tuple_windows()
                .find(|(a,b,c,d,e)| a == b && b == c && c == d && d == e)
                .map(|(a,_,_,_,_)| a)
            {
                fives.push_back((idx, five_c));

                while !triples.is_empty() {
                    let (start, c) = triples[0];
                    let mtch = fives.iter().find(|(n, cc)| {
                        *n > start && *n <= start + 1000 && *cc == c
                    });
                    if let Some(_) = mtch {
                        keys.push((start, c));
                        triples.pop_front();
                        while fives.len() > 0 && fives[0].0 < start {
                            fives.pop_front();
                        }
                    }
                    else if idx > start + 1000 {
                        triples.pop_front();
                    }
                    else {
                        break;
                    }
                }
            }
        }

        if keys.len() >= 64 {
            break;
        }
    }
    keys[63].0
}

fn part1(input: &str) -> usize {
    search(input, 0)
}

fn part2(input: &str) -> usize {
    search(input, 2016)
}

fn main() {
    let input: Vec<String> = read_input();
    println!("Part 1: {}", part1(&input[0]));
    println!("Part 2: {}", part2(&input[0]));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day14_test() {
        assert_eq!(part1("abc"), 22728);
        assert_eq!(part2("abc"), 22551);
    }
}
