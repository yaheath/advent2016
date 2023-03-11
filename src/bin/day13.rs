use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};
use std::vec::Vec;
use advent_lib::read::read_input;
use advent_lib::bits::one_bits_u64;
use advent_lib::coords::Coord2D;

struct DynMap {
    cache: HashMap<Coord2D, bool>,
    seed: u64,
}
impl DynMap {
    fn new(seed: u64) -> Self {
        Self { cache: HashMap::new(), seed }
    }

    fn get(&mut self, c: Coord2D) -> bool {
        if !self.cache.contains_key(&c) {
            let x = c.x as u64;
            let y = c.y as u64;
            let v = x*x + 3*x + 2*x*y + y + y*y + self.seed;
            let v = one_bits_u64(v);
            self.cache.insert(c, v & 1 == 1);
        }
        self.cache[&c]
    }
}

fn dist_to(seed: u64, target: Coord2D) -> usize {
    let mut map = DynMap::new(seed);
    let mut queue: BinaryHeap<(Reverse<usize>,Coord2D)> = BinaryHeap::new();
    let mut visited: HashMap<Coord2D,usize> = HashMap::new();
    let initial = Coord2D::new(1, 1);
    visited.insert(initial, 0);
    queue.push((Reverse(initial.mdist_to(&target) as usize), initial));
    while let Some(node) = queue.pop() {
        let current = node.1;
        let dist = visited[&current];
        if current == target {
            return dist;
        }
        for n in current.neighbors4() {
            if n.x >= 0 && n.y >= 0 && !map.get(n) &&
                (!visited.contains_key(&n) || dist + 1 < visited[&n])
            {
                visited.insert(n, dist + 1);
                queue.push((Reverse(dist + n.mdist_to(&current) as usize), n));
            }
        }
    }
    panic!();
}

fn part1(input: u64) -> usize {
    dist_to(input, Coord2D::new(31, 39))
}

fn part2(input: u64) -> usize {
    let mut map = DynMap::new(input);
    let mut visited: HashMap<Coord2D,usize> = HashMap::new();
    let initial = Coord2D::new(1, 1);
    visited.insert(initial, 0);
    let mut queue: Vec<Coord2D> = Vec::new();
    queue.push(initial);
    while let Some(node) = queue.pop() {
        let dist = visited[&node];
        if dist == 50 { continue; }
        for n in node.neighbors4() {
            if n.x >= 0 && n.y >= 0 && !map.get(n) &&
                (!visited.contains_key(&n) || dist + 1 < visited[&n])
            {
                visited.insert(n, dist + 1);
                queue.push(n);
            }
        }
    }
    visited.len()
}

fn main() {
    let input: Vec<u64> = read_input();
    println!("Part 1: {}", part1(input[0]));
    println!("Part 2: {}", part2(input[0]));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day13_test() {
        assert_eq!(dist_to(10, Coord2D::new(7, 4)), 11);
    }
}
