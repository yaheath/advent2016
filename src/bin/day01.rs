use std::collections::HashSet;
use std::str::FromStr;
use std::vec::Vec;
use ya_advent_lib::read::read_input;
use ya_advent_lib::coords::{CDir, Coord2D};

#[derive(Clone, Copy)]
enum Mov {
    Left(i64),
    Right(i64),
}
impl FromStr for Mov {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (chr, num) = s.split_at(1);
        let num = num.parse::<i64>().unwrap();
        match chr {
            "L" => Ok(Mov::Left(num)),
            "R" => Ok(Mov::Right(num)),
            _ => panic!(),
        }
    }
}

struct Input {
    moves: Vec<Mov>,
}

impl FromStr for Input {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let moves: Vec<Mov> = s.split(", ")
            .map(|ss| ss.parse::<Mov>().unwrap())
            .collect();
        Ok(Input{moves})
    }
}

fn part1(input: &Input) -> i64 {
    let mut dir = CDir::N;
    let mut pos = Coord2D::new(0, 0);
    for mov in input.moves.iter() {
        let (newdir, steps) = match mov {
            Mov::Left(n) => (dir.left(), n),
            Mov::Right(n) => (dir.right(), n),
        };
        dir = newdir;
        pos += <CDir as Into<Coord2D>>::into(dir) * *steps;
    }
    pos.x.abs() + pos.y.abs()
}

fn part2(input: &Input) -> i64 {
    let mut dir = CDir::N;
    let mut pos = Coord2D::new(0, 0);
    let mut all_locs: HashSet<Coord2D> = HashSet::new();
    'outer:
    for mov in input.moves.iter() {
        let (newdir, steps) = match mov {
            Mov::Left(n) => (dir.left(), n),
            Mov::Right(n) => (dir.right(), n),
        };
        dir = newdir;
        for _ in 0..*steps {
            pos += dir;
            if all_locs.contains(&pos) {
                break 'outer;
            }
            all_locs.insert(pos);
        }
    }
    pos.x.abs() + pos.y.abs()
}

fn main() {
    let input: Vec<Input> = read_input();
    println!("Part 1: {}", part1(&input[0]));
    println!("Part 2: {}", part2(&input[0]));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day01_test() {
        let input = "R2, L3".parse::<Input>().unwrap();
        assert_eq!(part1(&input), 5);
        let input = "R2, R2, R2".parse::<Input>().unwrap();
        assert_eq!(part1(&input), 2);
        let input = "R5, L5, R5, R3".parse::<Input>().unwrap();
        assert_eq!(part1(&input), 12);
        let input = "R8, R4, R4, R8".parse::<Input>().unwrap();
        assert_eq!(part2(&input), 4);
    }
}
