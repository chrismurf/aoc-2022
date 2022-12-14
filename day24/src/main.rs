use std::fs;
use std::str::FromStr;
use ndarray::Array3;
use num::Integer;
use pathfinding::prelude::astar;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Pos {
    Start(usize),
    SpaceTime((usize, usize, usize)),
    End(usize),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
struct Blizzards {
    north: bool,
    east: bool,
    south: bool,
    west: bool
}

impl Blizzards {
    fn new() -> Self { Default::default() }

    fn from_char(ch : &char) -> Self {
        match ch {
            '.' => Blizzards::new(),
            '^' => Self { north: true, ..Default::default() },
            '>' => Self { east: true, ..Default::default() },
            'v' => Self { south: true, ..Default::default() },
            '<' => Self { west: true, ..Default::default() },
            _ => unimplemented!()
        }
    }

    fn as_char(&self) -> char {
        let count = self.count();
        if count == 0 { return '.' }
        else if count > 1 { return char::from_digit(count, 10).unwrap() }
        else if self.north { return '^' }
        else if self.east { return '>' }
        else if self.south { return 'v' }
        else if self.west { return '<' }
        unreachable!()
    }

    fn empty(&self) -> bool {
        !self.north && !self.east && !self.south && !self.west
    }

    fn count(&self) -> u32 {
        self.north as u32 + self.east as u32 + self.south as u32 + self.west as u32
    }
}

#[derive(Debug, Clone)]
struct ValleyMap {
    spacetime : Array3<Blizzards>
}

impl FromStr for ValleyMap {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Get needed matrix size - subtract two in each dim for the walls (#)
        let height = s.lines().count() - 2;
        let width = s.find("\n").expect("No newline found!") - 2;
        // Subtract two for the walls around the outside, which don't impact repeat.
        let repeat_time = (height).lcm(&(width));
        // Final element is contiguous, then prev elt, and finally the first elt (reverse order)
        let mut spacetime = Array3::from_elem((repeat_time, height, width), Blizzards::new());
        // Load in initial time (t0)
        for (row, line) in s.lines().skip(1).enumerate() {
            if line.starts_with("###") { continue; } // Skip last row too.
            for (col, ch) in line.chars().skip(1).enumerate() {
                if ch == '#' { continue; } // Skip last character too
                spacetime[[0, row, col]] = Blizzards::from_char(&ch);
            }
        }
        // Now propagate forward all the various blizzards, generating all time steps before repeat
        let shape = spacetime.dim();
        for t in 1..shape.0 {
            for r in 0..shape.1 {
                for c in 0..shape.2 {
                    spacetime[[t, r, c]] = Blizzards {
                        // Modulo on North / West to wrap around at end of line/column.
                        north: spacetime.get([t-1, (r+1) % shape.1, c]).unwrap().north,
                        west: spacetime.get([t-1, r, (c+1) % shape.2]).unwrap().west,
                        // inline if() to handle negative wrapping... usize here is annoying.
                        south: spacetime.get([t-1, if r==0 { shape.1 - 1 } else { r-1 }, c]).unwrap().south,                        
                        east: spacetime.get([t-1, r, if c==0 { shape.2 - 1 } else { c-1 }]).unwrap().east,
                    }
                }
            }
        }
        Ok ( ValleyMap { spacetime } )
    }    
}

impl ValleyMap {
    #![allow(dead_code)] // This is just for debugging and test
    fn time_to_string(&self, time: usize) -> String {
        let shape = self.spacetime.dim();
        let mut s = String::with_capacity(shape.1 + shape.1*shape.2);
        for r in 0..shape.1 {
            for c in 0..shape.2 {
                s.push(self.spacetime[[time % shape.0, r, c]].as_char());
            }
            s.push('\n');
        }
        s
    }

    fn min_distance_to_end(&self, pos: &Pos) -> usize {
        let shape = self.spacetime.dim();
        match pos {
            Pos::Start(_) => shape.1 + shape.2,
            Pos::SpaceTime((_t, r, c)) => (shape.1 - r) + (shape.2 - c) - 1,
            Pos::End(_) => 0
        }
    }

    fn min_distance_to_start(&self, pos: &Pos) -> usize {
        let shape = self.spacetime.dim();
        match pos {
            Pos::End(_) => shape.1 + shape.2,
            Pos::SpaceTime((_t, r, c)) => r + c + 1,
            Pos::Start(_) => 0
        }
    }

    fn available_moves(&self, pos: &Pos) -> Vec<(Pos, usize)> {
        let mut vec = Vec::with_capacity(5);
        match pos {
            Pos::Start(t) => {
                vec.push((Pos::Start(t+1), 1));

                let shape = self.spacetime.dim();
                let st_time = (t+1) % shape.0;
                if self.spacetime[[st_time, 0, 0]].empty() {
                    vec.push((Pos::SpaceTime((t+1, 0, 0)), 1))
                }
            },
            Pos::End(t) => {
                vec.push((Pos::End(t+1), 1));

                let shape = self.spacetime.dim();
                let st_time = (t+1) % shape.0;
                if self.spacetime[[st_time, shape.1-1, shape.2-1]].empty() {
                    vec.push((Pos::SpaceTime((t+1, shape.1-1, shape.2-1)), 1))
                }
             },
            Pos::SpaceTime((t, r, c)) => {
                let shape = self.spacetime.dim();
                let st_time = (t+1) % shape.0;
                // Wait
                if self.spacetime[[st_time, *r, *c]].empty() {
                    vec.push((Pos::SpaceTime((t+1, *r, *c)), 1))
                }
                // North
                if *r > 0 && self.spacetime[[st_time, *r - 1, *c]].empty() {
                    vec.push((Pos::SpaceTime((t+1, *r-1, *c)), 1))
                }
                // West
                if *c > 0 && self.spacetime[[st_time, *r, *c-1]].empty() {
                    vec.push((Pos::SpaceTime((t+1, *r, *c-1)), 1))
                }
                // South
                if *r < (shape.1 - 1) && self.spacetime[[st_time, *r+1, *c]].empty() {
                    vec.push((Pos::SpaceTime((t+1, *r+1, *c)), 1))
                }
                // East
                if *c < (shape.2 - 1) && self.spacetime[[st_time, *r, *c+1]].empty() {
                    vec.push((Pos::SpaceTime((t+1, *r, *c+1)), 1))
                }
                // Can we go back to start?!
                if *r == 0 && *c == 0 {
                    vec.push((Pos::Start(t+1), 1))
                }
                // Can we exit?!
                if *r == (shape.1-1) && *c == (shape.2-1) {
                    vec.push((Pos::End(t+1), 1))
                }
            },
        }
        vec
    }
}

pub fn day23() {
    let file = fs::read_to_string("input.txt").expect("Couldn't read input.txt");
    let plan : ValleyMap = file.parse().unwrap();

    // Part 1 - go to end.
    let (moves, minutes) = astar(
        &Pos::Start(0),
        |pos| plan.available_moves(pos),
        |pos| plan.min_distance_to_end(pos),
        |pos| match pos { Pos::End(_) => true, _ => false }
    ).expect("No solution found!");
    println!("Part 1 -- {} minutes.", minutes); // 253

    // Part 2 -  go back to start, and back to end.
    let (moves, minutes) = astar(
        moves.last().expect("Failed first part, second won't work either!"),
        |pos| plan.available_moves(pos),
        |pos| plan.min_distance_to_start(pos),
        |pos| match pos { Pos::Start(_) => true, _ => false }
    ).expect("Couldn't find way back to beginning.");

    println!("Part 2 -- another {} minutes to reach start.", minutes);

    let (moves, minutes) = astar(
        moves.last().expect("Failed first part, second won't work either!"),
        |pos| plan.available_moves(pos),
        |pos| plan.min_distance_to_end(pos),
        |pos| match pos { Pos::End(_) => true, _ => false }
    ).expect("Couldn't find way back to beginning.");

    println!("Part 2 -- another {} minutes to get back to end.", minutes);

    let Pos::End(total_minutes) = moves.last().unwrap() else { unreachable!() };
    println!("Part 2 -- {} minutes in total.", total_minutes);

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
        #.######
        #>>.<^<#
        #.<..<<#
        #>v.><>#
        #<^v^^>#
        ######.#
        "};

        let example : ValleyMap = input.parse().unwrap();

        let time11 = indoc! {"
        2^.^2>
        <v<.^<
        ..2.>2
        .<..>.
        "};
        assert_eq!(time11, example.time_to_string(11));
        assert_eq!(time11, example.time_to_string(23));
    }
}