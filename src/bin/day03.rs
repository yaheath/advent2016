use std::str::FromStr;
use std::vec::Vec;
use ya_advent_lib::read::read_input;

struct Input {
    a: u64,
    b: u64,
    c: u64,
}

impl FromStr for Input {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut splt = s.split_whitespace();
        let a = splt.next().unwrap().parse::<u64>().unwrap();
        let b = splt.next().unwrap().parse::<u64>().unwrap();
        let c = splt.next().unwrap().parse::<u64>().unwrap();
        Ok(Input{a, b, c})
    }
}

impl Input {
    fn is_triangle(&self) -> bool {
        self.a + self.b > self.c &&
            self.a + self.c > self.b &&
            self.b + self.c > self.a
    }
}

fn part1(input: &Vec<Input>) -> usize {
    input.iter().filter(|i| i.is_triangle()).count()
}

fn part2(input: &Vec<Input>) -> usize {
    input.chunks(3)
        .flat_map(|slice| [
            Input{ a: slice[0].a, b: slice[1].a, c: slice[2].a },
            Input{ a: slice[0].b, b: slice[1].b, c: slice[2].b },
            Input{ a: slice[0].c, b: slice[1].c, c: slice[2].c },
        ])
        .filter(|i| i.is_triangle())
        .count()
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
    fn day03_test() {
        let input: Vec<Input> = test_input("5 10 25");
        assert_eq!(part1(&input), 0);
        let input: Vec<Input> = test_input(
            "101 301 501\n\
             102 302 502\n\
             103 303 503\n\
             201 401 601\n\
             202 402 602\n\
             203 403 603\n"
        );
        assert_eq!(part2(&input), 6);
    }
}
