use std::ops::Range;
use std::str::FromStr;
use std::vec::Vec;
use itertools::Itertools;
use advent_lib::read::read_input;
use advent_lib::range::{merge_ranges, range_from_str};

struct Input {
    range: Range<u64>,
}

impl FromStr for Input {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Input { range: range_from_str(s, true)? })
    }
}

fn part1(input: &Vec<Input>) -> u64 {
    let mut iter = merge_ranges(
        input.iter().map(|i| i.range.clone()).sorted_by_key(|k| k.start)
    );
    iter.next().unwrap().end
}

fn part2(input: &Vec<Input>) -> u64 {
    let mut last_end = 0_u64;
    let mut count = 0;
    for r in merge_ranges(
        input.iter().map(|i| i.range.clone()).sorted_by_key(|k| k.start)
    ) {
        count += r.start - last_end;
        last_end = r.end;
    }
    if last_end < u32::MAX as u64 {
        count += u32::MAX as u64 - last_end;
    }
    count
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
    fn day20_test() {
        let input: Vec<Input> = test_input("5-8\n0-2\n4-7\n");
        assert_eq!(part1(&input), 3);
        assert_eq!(part2(&input), u32::MAX as u64 - 8);
    }
}
