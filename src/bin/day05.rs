use std::collections::HashMap;
use std::num::NonZeroUsize;
use std::sync::mpsc;
use std::thread;
use std::thread::available_parallelism;
use std::vec::Vec;
use ya_advent_lib::read::read_input;

fn bothparts(input: &str) -> (String, String) {
    let (data_tx, data_rx) = mpsc::channel();
    let nthreads = available_parallelism().unwrap_or(NonZeroUsize::new(2).unwrap());
    let nthreads = nthreads.get();
    //println!("{nthreads} threads");
    let mut quit_txes = Vec::with_capacity(nthreads);
    let mut threads = Vec::with_capacity(nthreads);

    for n in 0..nthreads {
        let input = String::from(input);
        let data_tx = data_tx.clone();
        let (quit_tx, quit_rx) = mpsc::channel();
        quit_txes.push(quit_tx);
        threads.push(
            thread::spawn(move || {
                for idx in (n..).step_by(nthreads) {
                    if (idx - n) % (nthreads * 100) == 0 {
                        data_tx.send((idx - n, None)).unwrap();
                    }
                    let s = format!("{input}{idx}");
                    let hash = format!("{:x}", md5::compute(s.as_bytes()));
                    if hash.starts_with("00000") {
                        data_tx.send((idx, Some(hash))).unwrap();
                    }
                    if quit_rx.try_recv().is_ok() {
                        break;
                    }
                }
            })
        );
    }

    let mut out1 = String::new();
    let mut out2 = ['_'; 8];
    let mut inbound: Vec<(usize, String)> = Vec::new();
    let mut checkpoints: HashMap<usize, usize> = HashMap::new();
    let mut current_checkpoint = 0;
    'outer: loop {
        if let Ok((idx, hash)) = data_rx.recv() {
            if let Some(h) = hash {
                inbound.push((idx, h));
            }
            else {
                checkpoints.entry(idx).and_modify(|e| *e += 1).or_insert(1);
                if current_checkpoint == 0 {
                    current_checkpoint = idx;
                }
            }
        }
        if current_checkpoint == 0 || checkpoints[&current_checkpoint] < nthreads {
            continue;
        }
        checkpoints.remove(&current_checkpoint);
        inbound.sort_unstable_by_key(|r| r.0);
        for row in inbound.iter().filter(|x| x.0 <= current_checkpoint) {
            let hash = &row.1;
            let mut hc = hash.chars().skip(5);
            let char6 = hc.next().unwrap();
            let char7 = hc.next().unwrap();
            if out1.len() < 8 {
                out1.push(char6);
            }
            if ('0'..='7').contains(&char6) {
                let i = (char6 as u8 - b'0') as usize;
                if out2[i] == '_' {
                    out2[i] = char7;
                }
            }
            if out1.len() == 8 && out2.iter().all(|c| *c != '_') {
                break 'outer;
            }
            println!("{out1} {}", out2.iter().collect::<String>());
        }
        inbound.retain(|x| x.0 > current_checkpoint);
        current_checkpoint = checkpoints.keys().copied().min().unwrap_or(0);
    }

    for tx in quit_txes {
        tx.send(true).unwrap();
    }
    for t in threads {
        t.join().unwrap();
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
