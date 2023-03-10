use std::vec::Vec;
use advent_lib::read::read_input;

fn decompress(s: &str, ver: usize) -> usize {
    let mut iter = s.chars();
    let mut len = 0usize;
    while let Some(c) = (&mut iter).next() {
        match c {
            '(' => {
                let mut ss = String::new();
                let mut slen = 0;
                let mut reps = 0;
                while let Some(p) = (&mut iter).next() {
                    match p {
                        'x' => {
                            slen = ss.parse::<usize>().unwrap();
                            ss = String::new();
                        },
                        ')' => {
                            reps = ss.parse::<usize>().unwrap();
                            break;
                        },
                        _ => {ss.push(p);}
                    }
                }
                if ver == 2 {
                    let r = (&mut iter).as_str();
                    let n = decompress(r.get(0..slen).unwrap(), 2);
                    len += n * reps;
                }
                else {
                    len += slen * reps;
                }
                for _ in 0..slen {
                    (&mut iter).next();
                }
            },
            _ => {
                len += 1;
            }
        }
    }
    len
}

fn part1(input: &Vec<String>) -> usize {
    decompress(&input[0], 1)
}

fn part2(input: &Vec<String>) -> usize {
    decompress(&input[0], 2)
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
    fn day09_test() {
        assert_eq!(decompress("ADVENT", 1), 6);
        assert_eq!(decompress("A(1x5)BC", 1), 7);
        assert_eq!(decompress("(3x3)XYZ", 1), 9);
        assert_eq!(decompress("A(2x2)BCD(2x2)EFG", 1), 11);
        assert_eq!(decompress("(6x1)(1x3)A", 1), 6);
        assert_eq!(decompress("X(8x2)(3x3)ABCY", 1), 18);

        assert_eq!(decompress("(3x3)XYZ", 2), 9);
        assert_eq!(decompress("X(8x2)(3x3)ABCY", 2), 20);
        assert_eq!(decompress("(27x12)(20x12)(13x14)(7x10)(1x12)A", 2), 241920);
        assert_eq!(decompress("(25x3)(3x3)ABC(2x3)XY(5x2)PQRSTX(18x9)(3x2)TWO(5x7)SEVEN", 2), 445);
    }
}
