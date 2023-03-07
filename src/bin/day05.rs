use std::vec::Vec;
use advent_lib::read::read_input;

fn bothparts(input: &str) -> (String, String) {
    let mut out1 = String::new();
    let mut out2 = ['_'; 8];
    for index in 0.. {
        let s = format!("{input}{index}");
        let hash = format!("{:x}", md5::compute(s.as_bytes()));
        if hash.starts_with("00000") {
            let mut hc = hash.chars().skip(5);
            let char6 = hc.next().unwrap();
            let char7 = hc.next().unwrap();
            if out1.len() < 8 {
                out1.push(char6);
            }
            if char6 >= '0' && char6 <= '7' {
                let i = (char6 as u8 - b'0') as usize;
                if out2[i] == '_' {
                    out2[i] = char7;
                }
            }
            if out1.len() == 8 && out2.iter().all(|c| *c != '_') {
                break;
            }
            println!("{out1} {}", out2.iter().collect::<String>());
        }
    }
    (out1, out2.iter().collect())
}

fn main() {
    let input: Vec<String> = read_input();
    let (part1, part2) = bothparts(&input[0]);
    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day05_test() {
        assert_eq!(bothparts("abc"), ("18f47a30".to_string(), "05ace8e3".to_string()));
    }
}
