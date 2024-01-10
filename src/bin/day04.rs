use std::str::FromStr;
use std::vec::Vec;
use itertools::Itertools;
use ya_advent_lib::read::read_input;

#[derive(Clone, Debug)]
struct Input {
    name: String,
    sector_id: u64,
    checksum: String,
}

impl FromStr for Input {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut sp1 = s.split('[');
        let nc = sp1.next().unwrap();
        let mut sp2 = nc.rsplitn(2, '-');
        let sector_id = sp2.next().unwrap().parse::<u64>().unwrap();
        let name = sp2.next().unwrap().to_string();
        let checksum = sp1.next().unwrap().strip_suffix(']').unwrap().to_string();
        Ok(Input{name, sector_id, checksum})
    }
}

impl Input {
    fn is_real(&self) -> bool {
        let histo = self.name.chars().filter(|c| *c != '-').counts();
        let cksum: String = histo.iter()
            .sorted_by(|a, b| b.1.cmp(a.1).then_with(|| a.0.cmp(b.0)))
            .map(|(k,_)| *k)
            .take(5)
            .collect();
        cksum == self.checksum
    }
    fn decrypt(&self) -> String {
        self.name.chars()
            .map(|c| {
                if c == '-' {
                    ' '
                }
                else {
                    (
                        (((c as u8 - b'a') as u64 + self.sector_id) % 26)
                        as u8 + b'a'
                    ) as char
                }
            })
            .collect()
    }
}

fn part1(input: &[Input]) -> u64 {
    input.iter()
        .filter(|i| i.is_real())
        .map(|i| i.sector_id)
        .sum()
}

fn part2(input: &[Input]) -> u64 {
    input.iter()
        .filter(|i| i.is_real())
        .map(|i| (i, i.decrypt()))
        .find(|(_, s)| s == "northpole object storage")
        .map(|(i, _)| i.sector_id)
        .unwrap()
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
    fn day04_test() {
        let input: Vec<Input> = test_input(
            "aaaaa-bbb-z-y-x-123[abxyz]\n\
             a-b-c-d-e-f-g-h-987[abcde]\n\
             not-a-real-room-404[oarel]\n\
             totally-real-room-200[decoy]\n"
        );
        assert_eq!(part1(&input), 1514);
        //assert_eq!(part2(&input), 0);
        let enc = "qzmt-zixmtkozy-ivhz-343[a]".parse::<Input>().unwrap();
        assert_eq!(enc.decrypt(), "very encrypted name");
    }
}
