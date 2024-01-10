use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};
use std::str::FromStr;
use std::vec::Vec;
use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use ya_advent_lib::coords::{CDir, Coord2D};
use ya_advent_lib::read::{read_input, ParseErr};

struct Input {
    x: i64,
    y: i64,
    size: usize,
    used: usize,
}

impl FromStr for Input {
    type Err = ParseErr;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(
                r"node-x(\d+)-y(\d+)\s+(\d+)T\s+(\d+)T"
            ).unwrap();
        }
        if let Some(caps) = RE.captures(s) {
            let x = caps.get(1).unwrap().as_str().parse::<i64>().unwrap();
            let y = caps.get(2).unwrap().as_str().parse::<i64>().unwrap();
            let size = caps.get(3).unwrap().as_str().parse::<usize>().unwrap();
            let used = caps.get(4).unwrap().as_str().parse::<usize>().unwrap();
            Ok(Input {x, y, size, used})
        }
        else {
            Err(ParseErr::Skip)
        }
    }
}

impl Input {
    fn avail(&self) -> usize {
        self.size - self.used
    }
}

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
struct Node {
    size: usize,
    used: usize,
}

impl Node {
    fn new(input: &Input) -> Self {
        Self {
            size: input.size,
            used: input.used,
        }
    }
}

fn part1(input: &[Input]) -> usize {
    input.iter()
        .tuple_combinations()
        .filter(|(a, b)|
            a.used > 0 && a.size <= b.avail() ||
            b.used > 0 && b.size <= a.avail()
        )
        .count()
}

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
struct Grid {
    nodes: Vec<Node>,
    width: usize,
    height: usize,
    target_loc: Coord2D,
}
impl Grid {
    fn from_input(input: &[Input]) -> Self {
        let map: HashMap<Coord2D, Node> =
            input.iter()
                .map(|i| (Coord2D::new(i.x, i.y), Node::new(i)))
                .collect();
        let max_x = input.iter().map(|i| i.x).max().unwrap() + 1;
        let max_y = input.iter().map(|i| i.y).max().unwrap() + 1;
        let mut nodes = Vec::with_capacity((max_x * max_y) as usize);
        for y in 0..max_y {
            for x in 0..max_x {
                nodes.push(map[&Coord2D::new(x, y)].clone());
            }
        }
        Grid {
            nodes,
            width: max_x as usize,
            height: max_y as usize,
            target_loc: Coord2D::new(max_x - 1, 0),
        }
    }
    fn index_for(&self, loc: Coord2D) -> usize {
        loc.y as usize * self.width + loc.x as usize
    }

    fn get(&self, coord: Coord2D) -> &Node {
        let idx = self.index_for(coord);
        &self.nodes[idx]
    }

    #[allow(dead_code)]
    fn print(&self) {
        for y in 0..self.height as i64 {
            for x in 0..self.width as i64 {
                let coord = Coord2D::new(x, y);
                let cell = &self.nodes[self.index_for(coord)];
                let m = if coord == self.target_loc { "*" } else { "" };
                let s = format!("{}/{}{m}", cell.used, cell.size);
                print!(" {s:^8}");
            }
            println!();
        }
        println!();
    }
}

// Find the "hole" and move it to the square to the left of the target data using A*.
// Returns: number of steps taken
fn phase1(grid: &Grid) -> usize {
    let hole = (0..grid.width as i64).cartesian_product(0..grid.height as i64)
        .map(|(x,y)| Coord2D::new(x,y))
        .find(|c| grid.nodes[grid.index_for(*c)].used == 0)
        .unwrap();
    let target = grid.target_loc + CDir::W;
    let mut queue: BinaryHeap<(Reverse<i64>, Coord2D)> = BinaryHeap::new();
    let mut visited: HashMap<Coord2D,usize> = HashMap::new();
    queue.push((Reverse(hole.mdist_to(&target)), hole));
    visited.insert(hole, 1);
    while let Some((_, loc)) = queue.pop() {
        let steps = visited[&loc];
        if loc == target {
            return steps;
        }
        let locsize = grid.get(loc).size;
        for nc in [CDir::N, CDir::S, CDir::E, CDir::W]
            .iter()
            .map(|d| loc + *d)
            .filter(|nc| nc.x >= 0 && nc.x < grid.width as i64 && nc.y >= 0 && nc.y < grid.height as i64)
        {
            if grid.get(nc).used <= locsize
                && (!visited.contains_key(&nc) || visited[&nc] > steps) {
                    visited.insert(nc, steps + 1);
                    queue.push((Reverse(nc.mdist_to(&target)), nc));
            }
        }
    }
    panic!();
}

fn phase2(grid: &Grid) -> usize {
    let hole = grid.target_loc + CDir::W;
    (hole.x * 5) as usize
}

fn part2(input: &[Input]) -> usize {
    let grid = Grid::from_input(input);
    phase1(&grid) + phase2(&grid)
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
    fn day22_test() {
        let input: Vec<Input> = test_input(include_str!("day22.testinput"));
        assert_eq!(part2(&input), 7);
    }
}
