use std::vec::Vec;
use advent_lib::read::read_input;

#[derive(Clone, Copy)]
struct Node {
    elf: usize,
    gifts: usize,
    next_elf_idx: usize,
    prev_elf_idx: usize,
}

struct Ring {
    elves: Vec<Node>,
    remaining: usize,
    current_elf_idx: usize,
    opposing_elf_idx: usize,
}
impl Ring {
    fn new(num_elves: usize) -> Self {
        let mut elves: Vec<Node> = Vec::with_capacity(num_elves);
        for idx in 0..num_elves {
            elves.push( Node {
                elf: idx + 1,
                gifts: 1,
                next_elf_idx: (idx + 1) % num_elves,
                prev_elf_idx: if idx == 0 { num_elves - 1 } else { idx - 1 },
            });
        }
        Self {
            elves,
            remaining: num_elves,
            current_elf_idx: 0,
            opposing_elf_idx: num_elves / 2,
        }
    }
    fn current_elf(&self) -> usize {
        self.elves[self.current_elf_idx].elf
    }
    fn advance(&mut self) {
        self.current_elf_idx = self.elves[self.current_elf_idx].next_elf_idx;
        self.opposing_elf_idx = self.elves[self.opposing_elf_idx].next_elf_idx;
    }
    fn next_elf_index(&self) -> usize {
        self.elves[self.current_elf_idx].next_elf_idx
    }
    fn opposing_elf_index(&self) -> usize {
        self.opposing_elf_idx
    }
    fn steal_from(&mut self, target: usize) {
        self.elves.get_mut(self.current_elf_idx).unwrap().gifts += self.elves[target].gifts;
        self.remove_elf_at_index(target);
    }
    fn remove_elf_at_index(&mut self, target: usize) {
        let prev_idx = self.elves[target].prev_elf_idx;
        let next_idx = self.elves[target].next_elf_idx;
        self.elves.get_mut(prev_idx).unwrap().next_elf_idx = next_idx;
        self.elves.get_mut(next_idx).unwrap().prev_elf_idx = prev_idx;
        /*
        let t = self.elves.get_mut(target).unwrap();
        (*t).next_elf_idx = usize::MAX;
        (*t).prev_elf_idx = usize::MAX;
        (*t).gifts = 0;
        */
        if target == self.opposing_elf_idx {
            self.opposing_elf_idx = if self.remaining % 2 == 0 {
                prev_idx
            } else {
                next_idx
            };
        }
        self.remaining -= 1;
    }
}

fn solve(num_elves: usize, is_part2: bool) -> usize {
    let mut ring = Ring::new(num_elves);
    while ring.remaining > 1 {
        let target = if is_part2 {
            ring.opposing_elf_index()
        } else {
            ring.next_elf_index()
        };
        ring.steal_from(target);
        ring.advance();
    }
    ring.current_elf()
}

fn part1(input: &Vec<usize>) -> usize {
    solve(input[0], false)
}

fn part2(input: &Vec<usize>) -> usize {
    solve(input[0], true)
}

fn main() {
    let input: Vec<usize> = read_input();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;
    use advent_lib::read::test_input;

    #[test]
    fn day19_test() {
        let input: Vec<usize> = test_input("5");
        assert_eq!(part1(&input), 3);
        assert_eq!(part2(&input), 2);
    }
}
