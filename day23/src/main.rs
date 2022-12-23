use std::fs;
use std::str::FromStr;
use std::collections::{HashSet, HashMap};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use lazy_static::lazy_static;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumIter)]
enum Dir { N, NE, E, SE, S, SW, W, NW }

impl Dir {
    // East / South is positive
    fn dy(&self) -> isize {
        match *self {
            Dir::NW | Dir::N | Dir::NE => -1,
            Dir::SW | Dir::S | Dir::SE => 1,
            _ => 0,
        }
    }

    fn dx(&self) -> isize {
        match *self {
            Dir::NW | Dir::W | Dir::SW => -1,
            Dir::NE | Dir::E | Dir::SE => 1,
            _ => 0,
        }
    }
}

#[derive(Debug, Clone)]
struct ElfPlan {
    elves : HashSet<(isize, isize)>,
    step : usize
}

impl FromStr for ElfPlan {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut elves = HashSet::new();
        for (row, line) in s.lines().enumerate() {
            for (col, ch) in line.chars().enumerate() {
                if ch == '#' { elves.insert((col as isize, row as isize)); }
            }
        }
        Ok ( Self { elves, step: 0 } )
    }    
}

impl ElfPlan {
    fn neighbors_present(&self, pos : &(isize, isize)) -> HashSet<Dir> {
        Dir::iter()
            .filter(|dir| self.elves.contains(&(pos.0 + dir.dx(), pos.1 + dir.dy())))
            .collect()
    }

    fn move_for(&self, neighbors: &HashSet<Dir>) -> Option<Dir> {
        lazy_static! {
            static ref FUNCS : [(Dir, HashSet<Dir>); 4] = [
                (Dir::N, HashSet::from([Dir::NW, Dir::N, Dir::NE])),
                (Dir::S, HashSet::from([Dir::SW, Dir::S, Dir::SE])),
                (Dir::W, HashSet::from([Dir::NW, Dir::W, Dir::SW])),
                (Dir::E, HashSet::from([Dir::NE, Dir::E, Dir::SE])),
            ];
        }

        // If no other Elves are in one of those eight positions, the Elf does not do anything
        let count = neighbors.len();
        if count == 0 || count == 8 { return None }

        for func_attempt in 0..4 as usize {
            // Get which function to try next, they change each step.
            let func = (self.step + func_attempt) % 4;
            let (ref dir, ref incompatible_neighbors) = &FUNCS[func];
            if neighbors.intersection(&incompatible_neighbors).count() == 0 { return Some(*dir); }
        }
        None
    }

    fn step(&mut self) -> bool {
        let mut moves = HashMap::new();

        for elf in &self.elves {
            // Propose a move based on neighbor count
            let neighbors = self.neighbors_present(elf);
            let my_move = self.move_for(&neighbors);
            if my_move.is_some() {
                let new_pos = (elf.0 + my_move.unwrap().dx(), elf.1 + my_move.unwrap().dy());
                *moves.entry(new_pos).or_insert(0) += 1;
            } else {
                *moves.entry(*elf).or_insert(0) += 1;
            }
        }

        // Now everybody move... or not.
        let mut new_elves = HashSet::new();
        let mut anybody_moved : bool = false;
        for elf in &self.elves {
            // Propose a move based on neighbor count
            let neighbors = self.neighbors_present(elf);
            let my_move = self.move_for(&neighbors);
            if my_move.is_some() {
                let new_elf = (elf.0 + my_move.unwrap().dx(), elf.1 + my_move.unwrap().dy());
                if moves.get(&new_elf) == Some(&1) {
                    new_elves.insert(new_elf);
                    anybody_moved = true;
                } else {
                    new_elves.insert(*elf);
                }    
            } else {
                new_elves.insert(*elf);
            }
        }
        self.elves = new_elves;
        self.step += 1;
        anybody_moved
    }

    fn bounds(&self) -> (isize, isize, isize, isize) {(
        self.elves.iter().map(|elf| elf.0).min().unwrap(),
        self.elves.iter().map(|elf| elf.0).max().unwrap(),
        self.elves.iter().map(|elf| elf.1).min().unwrap(),
        self.elves.iter().map(|elf| elf.1).max().unwrap(),
    )}

    fn bounded_empty(&self) -> usize {
        let (min_x, max_x, min_y, max_y) = self.bounds();
        let area = ((max_x-min_x+1) * (max_y-min_y+1)) as usize;
        area - self.elves.len()
    }

    fn print(&self) {
        let (min_x, max_x, min_y, max_y) = self.bounds();

        for y in min_y..=max_y {
            for x in min_x..=max_x {
                if self.elves.contains(&(x,y)) { print!("#"); }
                else { print!("."); }
            }
            println!();
        }
    }
}

pub fn day23() {
    let file = fs::read_to_string("input.txt").expect("Couldn't read input.txt");
    let mut plan : ElfPlan = file.parse().unwrap();
    for _ in 0..10 {
        plan.step();
    }

    println!("Part 1 is {}", plan.bounded_empty());

    while plan.step() { }
    println!("Part 2 is {}", plan.step);
}

pub fn main() {
    day23()
}

#[cfg(test)]
 mod test {
    use indoc::indoc;
    use super::*;

    #[test]
    fn test_one() {
        let input = indoc! {"
        .....
        ..##.
        ..#..
        .....
        ..##.
        .....
        "};
        let mut plan : ElfPlan = input.parse().unwrap();

        plan.print();

        plan.step();
        plan.print();

        plan.step();
        plan.print();
    }
}