use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

use ya_advent_lib::grid::Grid;
use ya_advent_lib::read::read_input;
use ya_advent_lib::coords::Coord2D;

#[derive(Copy, Clone)]
enum Cell {
    Open,
    Wall,
    Waypoint(char),
}

impl From<char> for Cell {
    fn from(c: char) -> Self {
        match c {
            '.' => Cell::Open,
            d if d >= '0' && d <= '9' => Cell::Waypoint(d),
            _ => Cell::Wall,
        }
    }
}

fn mkgrid(input: &Vec<String>) -> Grid<Cell> {
    Grid::from_input(input, Cell::Wall, 0)
}

fn distbetween(grid: &Grid<Cell>, x1: i64, y1: i64, x2: i64, y2: i64) -> usize {
    let mut queue: BinaryHeap<(Reverse<usize>, Coord2D)> = BinaryHeap::new();
    let mut visited: HashMap<Coord2D, usize> = HashMap::new();
    let initial = Coord2D::new(x1, y1);
    let target = Coord2D::new(x2, y2);
    visited.insert(initial, 0);
    queue.push((Reverse(initial.mdist_to(&target) as usize), initial));
    while let Some(node) = queue.pop() {
        let current = node.1;
        let dist = visited[&current];
        if current == target {
            return dist;
        }
        for n in current.neighbors4() {
            match grid.get_or_default(n.x, n.y, Cell::Wall) {
                Cell::Wall => continue,
                _ => {},
            };
            if !visited.contains_key(&n) || dist + 1 < visited[&n] {
                visited.insert(n, dist + 1);
                queue.push((Reverse(dist + n.mdist_to(&current) as usize), n));
            }
        }
    }
    panic!();
}

fn bothparts(input: &Vec<String>, partnum: usize) -> usize {
    assert!(partnum == 1 || partnum == 2);
    let grid = mkgrid(input);
    let mut waypoints = grid
        .iter_with_coord()
        .filter(|(c, _, _)| matches!(c, Cell::Waypoint(_)))
        .map(|(c, x, y)| (match c { Cell::Waypoint(d) => d, _ => panic!() }, x, y))
        .collect::<Vec<_>>();
    waypoints.sort_by_key(|&(c,_,_)| c);
    let mut dx_cache: HashMap<(char, char), usize> = HashMap::new();
    let start = waypoints[0];
    let mut queue: BinaryHeap<(Reverse<usize>, Vec<char>)> = BinaryHeap::new();
    queue.push((Reverse(0), vec![start.0]));
    while let Some(node) = queue.pop() {
        let current = node.1;
        if current.len() == waypoints.len() + (partnum - 1) {
            return node.0.0;
        }
        let cwp = waypoints.iter().find(|wp| wp.0 == current[current.len() - 1]).unwrap();
        for n in waypoints.iter().filter(|wp| !current.contains(&wp.0)) {
            let dx = dx_cache.entry((cwp.0, n.0))
                .or_insert_with(|| distbetween(&grid, cwp.1, cwp.2, n.1, n.2));
            let mut next = current.clone();
            next.push(n.0);
            queue.push((Reverse(*dx + node.0.0), next));
        }
        if partnum == 2 && current.len() == waypoints.len() {
            let dx = dx_cache.get(&('0', cwp.0)).unwrap();
            let mut next = current.clone();
            next.push('0');
            queue.push((Reverse(*dx + node.0.0), next));
        }
    }
    panic!();
}

fn main() {
    let input: Vec<String> = read_input();
    println!("Part 1: {}", bothparts(&input, 1));
    println!("Part 2: {}", bothparts(&input, 2));
}

#[cfg(test)]
mod tests {
    use super::*;
    use ya_advent_lib::read::test_input;

    #[test]
    fn day24_test() {
        let input: Vec<String> = test_input(
            "###########\n\
            #0.1.....2#\n\
            #.#######.#\n\
            #4.......3#\n\
            ###########\n"
        );
        assert_eq!(bothparts(&input, 1), 14);
    }
}
