use std::str::FromStr;
use std::vec::Vec;
use lazy_static::lazy_static;
use regex::Regex;
use ya_advent_lib::grid::Grid;
use ya_advent_lib::read::read_input;

enum Input {
    Rect(i64,i64),
    RotateRow(i64,i64),
    RotateCol(i64,i64),
}

impl FromStr for Input {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RECT: Regex = Regex::new(
                r"rect (\d+)x(\d+)"
            ).unwrap();
        }
        lazy_static! {
            static ref ROT: Regex = Regex::new(
                r"rotate (\w+) .=(\d+) by (\d+)"
            ).unwrap();
        }
        if let Some(caps) = RECT.captures(s) {
            let w:i64 = caps.get(1).unwrap().as_str().parse::<i64>().unwrap();
            let h:i64 = caps.get(2).unwrap().as_str().parse::<i64>().unwrap();
            Ok(Input::Rect(w, h))
        }
        else if let Some(caps) = ROT.captures(s) {
            let dir:&str = caps.get(1).unwrap().as_str();
            let rc:i64 = caps.get(2).unwrap().as_str().parse::<i64>().unwrap();
            let n:i64 = caps.get(3).unwrap().as_str().parse::<i64>().unwrap();
            match dir {
                "row" => Ok(Input::RotateRow(rc, n)),
                "column" => Ok(Input::RotateCol(rc, n)),
                _ => Err(()),
            }
        }
        else {
            Err(())
        }
    }
}

fn process(input: &Vec<Input>, width: i64, height: i64) -> Grid<bool> {
    let mut grid = Grid::new(0, 0, width - 1, height - 1, false);
    for i in input {
        match i {
            Input::Rect(w, h) => {
                for y in 0..*h {
                    for x in 0..*w {
                        grid.set(x, y, true);
                    }
                }
            },
            Input::RotateRow(row, n) => { grid.roll_row(*row, *n); },
            Input::RotateCol(row, n) => { grid.roll_col(*row, *n); },
        }
    }
    grid
}

fn main() {
    let input: Vec<Input> = read_input();
    let grid = process(&input, 50, 6);
    println!("Part 1: {}", grid.iter().filter(|c| **c).count());
    println!("Part 2:");
    grid.print_str(|c| if c { "\u{2588}".into() } else { " ".into() });
}

#[cfg(test)]
mod tests {
    use super::*;
    use ya_advent_lib::read::test_input;

    #[test]
    fn day08_test() {
        let input: Vec<Input> = test_input(
            "rect 3x2\n\
             rotate column x=1 by 1\n\
             rotate row y=0 by 4\n\
             rotate column x=1 by 1\n"
        );
        let grid = process(&input, 7, 3);
        assert_eq!(
            grid.format_str(|c| if c { "#".into() } else { ".".into() }),
            ".#..#.#\n\
             #.#....\n\
             .#.....\n".to_string()
        );
    }
}
