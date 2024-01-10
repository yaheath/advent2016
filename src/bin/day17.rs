use std::collections::VecDeque;
use std::vec::Vec;
use ya_advent_lib::read::read_input;
use ya_advent_lib::coords::{CDir, Coord2D};


fn solve(passcode: &str, find_longest: bool) -> Option<String> {
    let initial = Coord2D::new(0, 0);
    let target = Coord2D::new(3, 3);
    let mut longest: Option<String> = None;
    let mut queue: VecDeque<(Coord2D, String)> = VecDeque::new();
    queue.push_back((initial, String::new()));
    while let Some((loc, path)) = queue.pop_front() {
        if loc == target {
            if find_longest {
                if !longest.is_some() || longest.as_ref().unwrap().len() < path.len() {
                    longest = Some(path.clone());
                }
                continue;
            }
            else {
                return Some(path);
            }
        }
        let key = format!("{passcode}{path}");
        let hash = format!("{:x}", md5::compute(key.as_bytes()));
        hash.chars()
            .take(4)
            .zip([CDir::N, CDir::S, CDir::W, CDir::E])
            .flat_map(|(c,d)| match c {
                'b'..='f' => Some((loc + d, d)),
                _ => None,
            })
            .filter(|(c,_)| (0..4).contains(&c.x) && (0..4).contains(&c.y))
            .for_each(|(c,d)| {
                let mut newpath = path.clone();
                newpath.push(match d {
                    CDir::N => 'U',
                    CDir::S => 'D',
                    CDir::W => 'L',
                    CDir::E => 'R',
                });
                queue.push_back((c, newpath));
            });
    }
    longest
}

fn part1(input: &Vec<String>) -> String {
    solve(&input[0], false).unwrap()
}

fn part2(input: &Vec<String>) -> usize {
    solve(&input[0], true).unwrap().len()
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
    fn day17_test() {
        assert_eq!(solve("hijkl", false), None);
        assert_eq!(solve("ihgpwlah", false), Some("DDRRRD".to_string()));
        assert_eq!(solve("kglvqrro", false), Some("DDUDRLRRUDRD".to_string()));
        assert_eq!(solve("ulqzkmiv", false), Some("DRURDRUDDLLDLUURRDULRLDUUDDDRR".to_string()));

        let p = solve("ihgpwlah", true).map(|p| p.len());
        assert_eq!(p, Some(370));
        let p = solve("kglvqrro", true).map(|p| p.len());
        assert_eq!(p, Some(492));
        let p = solve("ulqzkmiv", true).map(|p| p.len());
        assert_eq!(p, Some(830));
    }
}
