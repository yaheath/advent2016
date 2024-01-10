use std::cmp::{Ordering, Reverse};
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::str::FromStr;
use std::vec::Vec;
use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use ya_advent_lib::read::read_input;

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
enum Item {
    Chip(String),
    Generator(String),
}
impl FromStr for Item {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.contains("-compatible") {
            let chip:String = s.split('-').next().unwrap().to_string();
            Ok(Item::Chip(chip))
        }
        else {
            let gen:String = s.split(' ').next().map(|x| x.to_string()).ok_or(())?;
            Ok(Item::Generator(gen))
        }
    }
}
impl Ord for Item {
    fn cmp(&self, other: &Self) -> Ordering {
        let (self_s, self_c) = match self {
            Item::Chip(s) => (s, 'c'),
            Item::Generator(s) => (s, 'g'),
        };
        let (other_s, other_c) = match other {
            Item::Chip(s) => (s, 'c'),
            Item::Generator(s) => (s, 'g'),
        };
        self_s.cmp(other_s).then_with(|| self_c.cmp(&other_c))
    }
}
impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug)]
struct Input {
    floor: usize,
    items: Vec<Item>,
}

impl FromStr for Input {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(
                r"(?: contains (?:a )?)|(?:,? (?:and )?a )|\."
            ).unwrap();
        }
        let mut iter = RE.split(s);
        let floor = (match iter.next() {
            Some("The first floor") => Some(0),
            Some("The second floor") => Some(1),
            Some("The third floor") => Some(2),
            Some("The fourth floor") => Some(3),
            _ => None,
        }).ok_or(())?;
        let mut items = Vec::new();
        for i in iter {
            if i == "" || i.contains("nothing") {
                continue;
            }
            let item = i.parse::<Item>()?;
            items.push(item);
        }
        Ok(Self { floor, items })
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
struct State {
    step: usize,
    elevator: usize,
    floors: Vec<Vec<Item>>,
}

impl State {
    fn from_input(input: &Vec<Input>, additional_items: &Vec<Item>) -> Self {
        let mut floors: Vec<Vec<Item>> = Vec::with_capacity(4);
        for _ in 0..4 { floors.push(Vec::new()); }
        for i in input {
            floors[i.floor].extend_from_slice(&i.items);
        }
        for i in additional_items {
            floors[0].push(i.clone());
        }
        for f in 0..4 {
            floors.get_mut(f).unwrap().sort();
        }
        Self {
            step: 0,
            elevator: 0,
            floors,
        }
    }

    fn valid_floor(items: &Vec<&Item>) -> bool {
        if items.len() < 2 { return true; }
        let chips:HashSet<&String> = items.iter()
            .flat_map(|i| match i {
                Item::Chip(n) => Some(n),
                _ => None,
            })
            .collect();
        if chips.len() == items.len() { return true; }
        let gens:HashSet<&String> = items.iter()
            .flat_map(|i| match i {
                Item::Generator(n) => Some(n),
                _ => None,
            })
            .collect();
        chips.iter()
            .all(|c| gens.contains(c))

    }

    fn valid_next_states(&self) -> Vec<Self> {
        let adj_floors = match self.elevator {
            0 => vec![1],
            3 => vec![2],
            _ => vec![self.elevator + 1, self.elevator - 1],
        };
        self.floors[self.elevator].iter()
            .map(|item| Some(item))
            .chain(std::iter::once(None))
            .combinations(2)
            .map(|v| {
                match (v[0], v[1]) {
                    (Some(item1), Some(item2)) => {
                        vec![item1, item2]
                    },
                    (Some(item), None) => {
                        vec![item]
                    },
                    _ => panic!(),
                }
            })
            .filter(|items| {
                let f = self.floors[self.elevator].iter().filter(|i| !items.contains(i)).collect();
                Self::valid_floor(&f)
            })
            .cartesian_product(adj_floors)
            .filter(|(items, floor)| {
                let mut f: Vec<&Item> = self.floors[*floor].iter().collect();
                f.extend_from_slice(items);
                Self::valid_floor(&f)
            })
            .map(|(items, floor)| self.move_items(items, floor))
            .collect()
    }

    fn move_items(&self, items: Vec<&Item>, to_floor: usize) -> Self {
        let floors: Vec<Vec<Item>> = self.floors.iter().enumerate()
            .map(|(idx, floor)| {
                if idx == self.elevator {
                    floor.iter().filter(|i| !items.contains(i)).cloned().collect()
                }
                else if idx == to_floor {
                    let mut r = floor.clone();
                    items.iter().for_each(|i| r.push((*i).clone()));
                    r.sort();
                    r
                }
                else {
                    floor.clone()
                }
            })
            .collect();
        Self {
            step: self.step + 1,
            elevator: to_floor,
            floors
        }
    }

    fn is_complete(&self) -> bool {
        self.floors[0].len() == 0 &&
        self.floors[1].len() == 0 &&
        self.floors[2].len() == 0
    }
}

#[derive(Eq, PartialEq, Hash)]
struct SubState {
    elevator: usize,
    floors: Vec<(usize,usize,usize)>, // (pairs, single_chips, single_gens)
}
impl SubState {
    fn from_state(state: &State) -> Self {
        let floors = state.floors.iter()
            .map(|f| {
                let chips:HashSet<&String> = f.iter()
                    .flat_map(|i| match i {
                        Item::Chip(n) => Some(n),
                        _ => None,
                    })
                    .collect();
                let gens:HashSet<&String> = f.iter()
                    .flat_map(|i| match i {
                        Item::Generator(n) => Some(n),
                        _ => None,
                    })
                    .collect();
                let pairs = chips.intersection(&gens).count();
                let single_chips = chips.difference(&gens).count();
                let single_gens = gens.difference(&chips).count();
                (pairs, single_chips, single_gens)
            })
            .collect();

        Self {
            elevator: state.elevator,
            floors,
        }
    }
}

fn solve(input: &Vec<Input>, additional_items: Vec<Item>) -> State {
    let initial = State::from_input(input, &additional_items);
    let mut visited: HashMap<SubState, usize> = HashMap::new();
    visited.insert(SubState::from_state(&initial), 0);
    let mut queue = BinaryHeap::new();
    queue.push(Reverse(initial));
    while let Some(state) = queue.pop() {
        let state = state.0;
        //println!("{state:?}");
        if state.is_complete() {
            return state;
        }
        for newstate in state.valid_next_states() {
            let ss = SubState::from_state(&newstate);
            if visited.contains_key(&ss) && visited[&ss] <= newstate.step {
                continue;
            }
            visited.insert(ss, newstate.step);
            queue.push(Reverse(newstate));
        }
    }
    panic!();
}

fn part1(input: &Vec<Input>) -> usize {
    let state = solve(input, vec![]);
    state.step
}

fn part2(input: &Vec<Input>) -> usize {
    let state = solve(input, vec![
        Item::Chip("elerium".to_string()),
        Item::Generator("elerium".to_string()),
        Item::Chip("dilithium".to_string()),
        Item::Generator("dilithium".to_string()),
    ]);
    state.step
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
    fn day11_test() {
        let input: Vec<Input> = test_input(
            "The first floor contains a hydrogen-compatible microchip and a lithium-compatible microchip.\n\
             The second floor contains a hydrogen generator.\n\
             The third floor contains a lithium generator.\n\
             The fourth floor contains nothing relevant.\n"
        );
        assert_eq!(part1(&input), 11);
    }
}
