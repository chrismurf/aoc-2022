use std::str::FromStr;
use std::{fs, io};

#[derive(Debug, Clone)]
pub struct Error;

#[derive(Debug, Clone)]
pub struct PathSolver {
    start : (usize, usize),
    end : (usize, usize),
    height : Vec<Vec<i32>>,

    // For calculations
    cost_to_end : Vec<Vec<Option<i32>>>,
}

impl PathSolver {
    fn neighbors(&self, x: usize, y : usize) -> Vec<(usize, usize)> {
        let mut neighbors = Vec::new();
        for [dx, dy] in [[-1i32, 0], [1, 0], [0, -1], [0, 1]] {
            if x as i32 + dx >= self.height[0].len() as i32 ||
               x as i32 + dx < 0 ||
               y as i32 + dy >= self.height.len() as i32 ||
               y as i32 + dy < 0 {
                   continue;
            }
            let level = self.height[y][x];
            let neighbor_level = self.height[(y as i32 + dy) as usize][(x as i32 + dx) as usize];
            if level - neighbor_level <= 1 { // could be negative, that's OK.
                neighbors.push(((x as i32 + dx) as usize, (y as i32 + dy) as usize));
            }

        }
        neighbors
    }

    fn compute_cost_to_end(&mut self) {
        let mut candidates = Vec::new();
        candidates.push(self.end);

        loop {
            if candidates.is_empty() { break; }
            let prev_candidates = candidates.clone();
            candidates.clear();

            for (cx, cy) in prev_candidates {
                for (nx, ny) in self.neighbors(cx, cy) {
                    if self.cost_to_end[ny][nx].is_some() { continue; }
                    self.cost_to_end[ny][nx] = Some(self.cost_to_end[cy][cx].unwrap() + 1);
                    candidates.push((nx, ny));
                }
            }
        }
    }
}

impl FromStr for PathSolver {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut start = (0, 0);
        let mut end = (0, 0);

        let mut height : Vec<Vec<i32>> = Vec::new();
        let mut steps : Vec<Vec<Option<i32>>> = Vec::new();
        for (y, line) in s.lines().enumerate() {
            let mut height_row : Vec<i32> = Vec::new();
            let mut steps_row : Vec<Option<i32>> = Vec::new();
            for (x, byte) in line.bytes().enumerate() {
                if byte == 'S' as u8 {
                    start = (x, y);
                    height_row.push(0);
                    steps_row.push( None );
                } else if byte == 'E' as u8 {
                    end = (x, y);
                    height_row.push(25);
                    steps_row.push( Some(0) );
                } else {
                    height_row.push(byte as i32 - 'a' as i32);
                    steps_row.push( None );
                }
            }
            height.push( height_row );
            steps.push( steps_row );
        }

        Ok(Self { start, end, height, cost_to_end:steps })
    }
}

pub fn day12() {
    let file = fs::read_to_string("input.txt").expect("File 'input.txt' not readable.");
    let mut solver : PathSolver = file.parse().unwrap();
    solver.compute_cost_to_end();
    let (sx, sy) = solver.start;
    if let Some(value) = solver.cost_to_end[sy][sx] {
        println!("Part 1 steps: {:?}", value);
    };

    let mut min_steps : i32 = 1000000;

    for (height_row, step_row) in solver.height.iter().zip(solver.cost_to_end) {
        for (height, steps) in height_row.iter().zip(step_row) {
            if *height == 0 && steps.is_some() && steps.unwrap() < min_steps {
                min_steps = steps.unwrap();
            }
        }
    }

    println!("Min Steps: {}", min_steps);
}

pub fn main() -> io::Result<()> {
    day12();
    Ok(())
}

#[cfg(test)]
 mod test {
    use indoc::indoc;
    use super::*;

    #[test]
    fn test_one() {
        let input = indoc!{"
            Sabqponm
            abcryxxl
            accszExk
            acctuvwj
            abdefghi
        "};
        let mut solver : PathSolver = input.parse().unwrap();
        solver.compute_cost_to_end();
        let (sx, sy) = solver.start;
        assert_eq!(solver.cost_to_end[sy][sx], Some(31));
    }
}
