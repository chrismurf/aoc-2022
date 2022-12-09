use std::fs::File;
use std::collections::HashSet;
use std::io::{self, prelude::*, BufReader};

pub mod geom;

#[derive(Default, Debug)]
pub struct RopeSimulation {
    knots: Vec<geom::Point>,
    tail_visited: HashSet<geom::Point>
}

impl RopeSimulation {
    pub fn from_num_followers(followers: usize) -> Self {
        return Self {
            knots: vec![geom::Point::new(0, 0); followers + 1],
            tail_visited: HashSet::from([geom::Point::new(0, 0)])
        }
    }

    pub fn step(&mut self, motion: &geom::Delta) {
        for _ in 0..motion.abs() {
            self.knots[0] += motion.unit_direction();

            for i in 1..self.knots.len() {
                if !self.knots[i-1].is_adjacent(&self.knots[i]) {
                    let diff = self.knots[i-1] - self.knots[i];
                    self.knots[i] += diff.unit_direction();
                }    
            }

            self.tail_visited.insert(self.knots[self.knots.len()-1]);
        }
    }
}

pub fn day09() {
    let file = File::open("input.txt").expect("File 'input.txt' not readable.");

    let mut part1 = RopeSimulation::from_num_followers(1);
    let mut part2 = RopeSimulation::from_num_followers(9);
    let reader = BufReader::new(file)
        .lines() // Get a line iterator
        .filter_map(|line| line.ok()); // Get Strings instead of Result

    for line in reader {
        part1.step(&line.parse().expect("Failed to parse input line."));
        part2.step(&line.parse().expect("Failed to parse input line."));
    }

    println!("Number of spots visited in Part 1 is {}", part1.tail_visited.len());
    println!("Number of spots visited in Part 2 is {}", part2.tail_visited.len());

}

pub fn main() -> io::Result<()> {
    day09();
    Ok(())
}
//6030
//2545
