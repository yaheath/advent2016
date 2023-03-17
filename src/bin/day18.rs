use std::str::FromStr;
use std::vec::Vec;
use itertools::Itertools;
use advent_lib::read::read_input;

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

fn nextrow(tiles: &Vec<bool>) -> Vec<bool> {
    std::iter::once(&false)
        .chain(tiles.iter())
        .chain(std::iter::once(&false))
        .tuple_windows()
        .map(|tuple| match tuple {
            (&true, &true, &false) => true,
            (&false, &true, &true) => true,
            (&true, &false, &false) => true,
            (&false, &false, &true) => true,
            _ => false,
        })
        .collect()
}

fn num_safe(tiles: &Vec<bool>, rows: usize) -> usize {
    let mut tiles = tiles.clone();
    let mut sum = tiles.iter().filter(|c| !**c).count();
    for _ in 0..(rows-1) {
        tiles = nextrow(&tiles);
        sum += tiles.iter().filter(|c| !**c).count();
    }
    sum
}

fn part1(input: &Vec<Input>) -> usize {
    num_safe(&input[0].tiles, 40)
}

fn part2(input: &Vec<Input>) -> usize {
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
    use advent_lib::read::test_input;

    #[test]
    fn day18_test() {
        let input: Vec<Input> = test_input(".^^.^.^^^^");
        assert_eq!(num_safe(&input[0].tiles, 10), 38);
        assert_eq!(part2(&input), 0);
    }
}
