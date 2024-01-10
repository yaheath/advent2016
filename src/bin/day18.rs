use std::str::FromStr;
use std::vec::Vec;
use itertools::Itertools;
use ya_advent_lib::read::read_input;

struct Input {
    tiles: Vec<bool>,
}

impl FromStr for Input {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tiles = s.chars().map(|c| c == '^').collect();
        Ok(Input{tiles})
    }
}

fn nextrow(tiles: &[bool]) -> Vec<bool> {
    std::iter::once(&false)
        .chain(tiles.iter())
        .chain(std::iter::once(&false))
        .tuple_windows()
        .map(|tuple| matches!(tuple,
            (&true, &true, &false) |
            (&false, &true, &true) |
            (&true, &false, &false) |
            (&false, &false, &true))
        )
        .collect()
}

fn num_safe(tiles: &[bool], rows: usize) -> usize {
    let mut tiles = tiles.to_owned();
    let mut sum = tiles.iter().filter(|c| !**c).count();
    for _ in 0..(rows-1) {
        tiles = nextrow(&tiles);
        sum += tiles.iter().filter(|c| !**c).count();
    }
    sum
}

fn part1(input: &[Input]) -> usize {
    num_safe(&input[0].tiles, 40)
}

fn part2(input: &[Input]) -> usize {
    num_safe(&input[0].tiles, 400000)
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
    fn day18_test() {
        let input: Vec<Input> = test_input(".^^.^.^^^^");
        assert_eq!(num_safe(&input[0].tiles, 10), 38);
    }
}
