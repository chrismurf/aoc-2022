use std::str::FromStr;
use std::cmp::{min, max};
use std::{fs, io};
use std::io::BufRead;
use ndarray::Array2;
use itertools::Itertools;
use std::{thread, time};

#[derive(Debug, Clone)]
pub struct Error;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Cell { Empty, Wall, Sand }

impl Cell {
    fn as_char(&self) -> char {
        match &self {
            Self::Empty => ' ',
            Self::Wall  => '█',
            Self::Sand  => '░',
        }
    }
}

#[derive(Debug, Clone)]
struct SandSimulation {
    x0 : i32,
    y0 : i32,
    grid : Array2<Cell>
}

impl SandSimulation {
    fn from_bounds(x0: i32, y0: i32, x1: i32, y1: i32) -> Self {
        let shape: (usize, usize) = ((y1-y0+1) as usize, (x1-x0+1) as usize).into();
        let grid: Array2<Cell> = Array2::from_elem(shape, Cell::Empty );
        Self { x0, y0, grid }
    }

    fn add_wall(&mut self, wall: &Polyline) {
        for point in wall.rasterize().iter() {
            let x = (point.x - self.x0) as usize;
            let y = (point.y - self.y0) as usize;
            self.grid[[y, x]] = Cell::Wall;
        }
    }

    fn print(&self) {
        for row in self.grid.rows() {
            for cell in row {
                print!("{}", cell.as_char());
            }
            println!();
        }    
    }

    // Runs until either we overflow, or clog the inlet.
    fn run(&mut self, inlet : Point, print : bool) -> usize {
        let mut count = 0;

        loop {
            let mut grain = Point {x: inlet.x - self.x0, y: inlet.y - self.y0};

            'grain: loop {
                if print {
                    self.print();
                    thread::sleep(time::Duration::from_millis(50));
                }

                for x in [grain.x, grain.x-1, grain.x+1] {
                    let y = (grain.y + 1) as i32;
                    match self.grid.get((y as usize, x as usize)) {
                        Some(Cell::Empty) => {
                            *self.grid.get_mut((grain.y as usize, grain.x as usize)).unwrap() = Cell::Empty;
                            grain.y = y;
                            grain.x = x;
                            *self.grid.get_mut((grain.y as usize, grain.x as usize)).unwrap() = Cell::Sand;
                            continue 'grain;
                        },
                        None => {
                            // Fell out of grid - report current count
                            return count;
                        },
                        _ => { },
                    }
                }               
                break 'grain;
            }

            // Stopped moving
            count += 1;

            if grain.x == inlet.x - self.x0 && grain.y == inlet.y - self.y0 {
                return count;
            }            
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Point { x: i32, y: i32 }

#[derive(Debug, Clone)]
struct Polyline { points : Vec<Point> }

impl FromStr for Polyline {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut points = Vec::new();
        for point in s.split(" -> ") {
            let (x, y) = point.split_once(",").expect("Failed to split a point");
            points.push(Point {x: x.parse::<i32>().unwrap(), y: y.parse::<i32>().unwrap()});
        }

        assert!(points.len() > 1);
        Ok(Self { points })
    }
}

impl Polyline {
    fn min_x(&self) -> i32 { self.points.iter().map(|p| p.x).min().unwrap() }
    fn min_y(&self) -> i32 { self.points.iter().map(|p| p.y).min().unwrap() }
    fn max_x(&self) -> i32 { self.points.iter().map(|p| p.x).max().unwrap() }
    fn max_y(&self) -> i32 { self.points.iter().map(|p| p.y).max().unwrap() }

    fn rasterize(&self) -> Vec<Point> {
        let mut vec = vec![];
        for (prev, next) in self.points.iter().tuple_windows() {
            if next.x != prev.x {
                for x in min(next.x, prev.x)..=max(next.x, prev.x) {
                    vec.push(Point {x, y: next.y});
                }
            } else {
                for y in min(next.y, prev.y)..=max(next.y, prev.y) {
                    vec.push(Point {x: next.x, y});
                }
            }
        }

        vec
    }
}


pub fn day14() -> Result<(), Error> {
    // First parse all the wall boundaries
    let file = fs::File::open("input.txt").expect("File 'input.txt' not readable.");
    let reader = io::BufReader::new(file)
        .lines() // Get a line iterator
        .filter_map(|line| line.ok()); // Get Strings instead of Result
    let walls = reader.map(|x| x.parse().unwrap()).collect::<Vec<Polyline>>();

    // Get outer bounds of work area
    let x0 = walls.iter().map(|p| p.min_x()).min().unwrap();
    let y0 = walls.iter().map(|p| p.min_y()).min().unwrap();
    let x1 = walls.iter().map(|p| p.max_x()).max().unwrap();
    let y1 = walls.iter().map(|p| p.max_y()).max().unwrap();    

    // Construct a simulation for Part 1
    const BUFFER: i32 = 2;
    let mut sim = SandSimulation::from_bounds(x0-BUFFER, min(y0, -1), x1+BUFFER, y1+BUFFER);

    for wall in &walls {
        sim.add_wall(&wall);
    }

    // Now run the simulation
    let sand_to_overflow = sim.run(Point {x: 500, y: 0}, false);
    sim.print();
    println!("In Part 1, {} units of sand fell *before* we went into the abyss.", sand_to_overflow);

    println!("\n===========================================================================================================\n");

    //////
    // Construct a simulation for Part 2
    const FLOOR_WIDTH: i32 = 160;
    let mut sim2 = SandSimulation::from_bounds(x0-FLOOR_WIDTH-2, min(y0, -1), x1+FLOOR_WIDTH+2, y1+2);

    for wall in &walls {
        sim2.add_wall(&wall);
    }

    sim2.add_wall(&Polyline {
        points: vec![
            Point {x: x0-FLOOR_WIDTH, y: y1+2 },
            Point {x: x1+FLOOR_WIDTH, y: y1+2 }
        ],
    });

    // Now run the simulation
    let sand_to_clog = sim2.run(Point {x: 500, y: 0}, false);
    sim2.print();
    println!("In Part 2, after {} units of sand fell we clogged the inlet.", sand_to_clog);

    Ok(())
}

pub fn main() -> Result<(), Error> {
    day14()
}
