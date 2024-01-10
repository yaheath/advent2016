use std::collections::{HashMap, VecDeque};
use std::num::NonZeroUsize;
use std::sync::mpsc;
use std::thread;
use std::thread::available_parallelism;
use std::vec::Vec;
use itertools::Itertools;
use ya_advent_lib::read::read_input;


fn search(salt: &str, stretch: usize) -> usize {
    let (data_tx, data_rx) = mpsc::channel();
    let nthreads = available_parallelism().unwrap_or(NonZeroUsize::new(2).unwrap());
    let nthreads = nthreads.get();
    //println!("{nthreads} threads");
    let mut quit_txes = Vec::with_capacity(nthreads);
    let mut threads = Vec::with_capacity(nthreads);

    for n in 0..nthreads {
        let salt = String::from(salt);
        let data_tx = data_tx.clone();
        let (quit_tx, quit_rx) = mpsc::channel();
        quit_txes.push(quit_tx);
        threads.push(
            thread::spawn(move || {
                for idx in (n..).step_by(nthreads) {
                    if (idx - n) % (nthreads * 100) == 0 {
                        data_tx.send((n, 0, idx - n, ' ')).unwrap();
                    }
                    let s = format!("{salt}{idx}");
                    let mut hash = format!("{:x}", md5::compute(s.as_bytes()));
                    for _ in 0..stretch {
                        hash = format!("{:x}", md5::compute(hash.as_bytes()));
                    }
                    if let Some(trip_c) = hash.chars()
                        .tuple_windows()
                        .find(|(a,b,c)| a == b && b == c)
                        .map(|(a,_,_)| a)
                    {
                        data_tx.send((n, 3, idx, trip_c)).unwrap();
                        if let Some(five_c) = hash.chars()
                            .tuple_windows()
                            .find(|(a,b,c,d,e)| a == b && b == c && c == d && d == e)
                            .map(|(a,_,_,_,_)| a)
                        {
                            data_tx.send((n, 5, idx, five_c)).unwrap();
                        }
                    }
                    if quit_rx.try_recv().is_ok() {
                        break;
                    }
                }
            })
        );
    }

    let mut inbound: Vec<(usize, usize, usize, char)> = Vec::new();
    let mut triples: VecDeque<(usize, char)> = VecDeque::new();
    let mut keys: Vec<(usize, char)> = Vec::new();
    let mut fives: VecDeque<(usize, char)> =VecDeque::new();
    let mut checkpoints: HashMap<usize, usize> = HashMap::new();
    let mut current_checkpoint = 0;

    loop {
        if let Ok((thrd, n, idx, ch)) = data_rx.recv() {
            match n {
                0 => {
                    checkpoints.entry(idx).and_modify(|e| *e += 1).or_insert(1);
                    if current_checkpoint == 0 {
                        current_checkpoint = idx;
                    }
                },
                3 | 5 => { inbound.push((thrd, n, idx, ch)); }
                _ => panic!(),
            }

            if current_checkpoint == 0 || checkpoints[&current_checkpoint] < nthreads {
                continue;
            }
            checkpoints.remove(&current_checkpoint);
            inbound.sort_unstable_by_key(|r| r.2);
            for row in inbound.iter().filter(|x| x.2 <= current_checkpoint) {
                let (_, n, idx, ch) = *row;
                match n {
                    3 => { triples.push_back((idx, ch)); },
                    5 => { fives.push_back((idx, ch)); },
                    0 => { },
                    _ => panic!(),
                }
                while !triples.is_empty() {
                    let (start, c) = triples[0];
                    let mtch = fives.iter().find(|(n, cc)| {
                        *n > start && *n <= start + 1000 && *cc == c
                    });
                    if let Some(_) = mtch {
                        keys.push((start, c));
                        triples.pop_front();
                        while fives.len() > 0 && fives[0].0 < start {
                            fives.pop_front();
                        }
                    }
                    else if idx > start + 1000 {
                        triples.pop_front();
                    }
                    else {
                        break;
                    }
                }
            }
            inbound.retain(|x| x.2 > current_checkpoint);
            current_checkpoint = checkpoints.keys().copied().min().unwrap_or(0);
        }
        if keys.len() >= 64 {
            break;
        }
    }
    for tx in quit_txes {
        tx.send(true).unwrap();
    }
    for t in threads {
        t.join().unwrap();
    }
    keys[63].0
}

fn part1(input: &str) -> usize {
    search(input, 0)
}

fn part2(input: &str) -> usize {
    search(input, 2016)
}

fn main() {
    let input: Vec<String> = read_input();
    println!("Part 1: {}", part1(&input[0]));
    println!("Part 2: {}", part2(&input[0]));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day14_test() {
        assert_eq!(part1("abc"), 22728);
        assert_eq!(part2("abc"), 22551);
    }
}
