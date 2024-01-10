use std::collections::HashMap;
use std::vec::Vec;
use ya_advent_lib::read::read_input;

fn bothparts(input: &[String]) -> (String, String) {
    let mut hists: Vec<HashMap<char, usize>> = Vec::with_capacity(input[0].len());
    for _ in 0..input[0].len() {
        hists.push(HashMap::new());
    }
    for row in input {
        row.chars()
            .enumerate()
            .for_each(|(idx, c)| {
                hists[idx].entry(c).and_modify(|count| *count += 1).or_insert(1);
            });
    }
    (
        hists.iter()
        .map(|h| h.iter()
            .max_by_key(|(_,v)| *v)
            .map(|(k,_)| k)
            .unwrap())
        .collect(),
        hists.iter()
        .map(|h| h.iter()
            .min_by_key(|(_,v)| *v)
            .map(|(k,_)| k)
            .unwrap())
        .collect()
    )
}

fn main() {
    let input: Vec<String> = read_input();
    let (part1, part2) = bothparts(&input);
    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
}

#[cfg(test)]
mod tests {
    use super::*;
    use ya_advent_lib::read::test_input;

    #[test]
    fn day06_test() {
        let input: Vec<String> = test_input(include_str!("day06.testinput"));
        let (part1, part2) = bothparts(&input);
        assert_eq!(part1, "easter".to_string());
        assert_eq!(part2, "advent".to_string());
    }
}
