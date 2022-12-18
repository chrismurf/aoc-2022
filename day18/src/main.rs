use std::fs::File;
use std::str::FromStr;
use std::collections::HashSet;
use std::io::{prelude::*, BufReader};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct XYZ { x: i32, y: i32, z: i32}

impl XYZ {
    fn up(&self) -> Self { Self { x:self.x, y: self.y+1, z:self.z } }
    fn down(&self) -> Self { Self { x:self.x, y: self.y-1, z:self.z } }
    fn left(&self) -> Self { Self { x:self.x-1, y: self.y, z:self.z } }
    fn right(&self) -> Self { Self { x:self.x+1, y: self.y, z:self.z } }
    fn fwd(&self) -> Self { Self { x:self.x, y: self.y, z:self.z+1 } }
    fn back(&self) -> Self { Self { x:self.x, y: self.y, z:self.z-1 } }
}

impl FromStr for XYZ {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts : Vec<&str> = s.split(",").collect();
        Ok(XYZ {x: parts[0].parse().unwrap(), y: parts[1].parse().unwrap(), z: parts[2].parse().unwrap() })
    }
}

#[derive(Debug, Clone)]
struct CubeMap {
    cubes : HashSet<XYZ>,
}

impl CubeMap {
    fn new() -> Self { Self { cubes: HashSet::new() } }

    fn add(&mut self, cube: XYZ) {
        self.cubes.insert(cube);
    }

    fn get_neighbors(&self, xyz: XYZ) -> HashSet<XYZ> {
        [xyz.up(), xyz.down(), xyz.left(), xyz.right(), xyz.fwd(), xyz.back()]
            .iter()
            .cloned()
            .collect()
    }

    fn get_neighbors_in_bounds(&self, xyz: XYZ) -> HashSet<XYZ> {
        self.get_neighbors(xyz)
            .iter()
            .filter(|xyz| self.in_bounds(xyz))
            .cloned()
            .collect()
    }

    fn get_present_neighbor_count(&self, xyz: XYZ) -> usize {
        self.get_neighbors(xyz)
            .iter()
            .filter(|xyz| self.cubes.contains(&xyz))
            .count()
    }

    fn surface_area(&self) -> usize {
        self.cubes.iter()
            .map(|cube| 6 - self.get_present_neighbor_count(*cube))
            .sum()
    }

    fn exterior_surface_area(&self) -> usize {
        let flooded = self.flood();
        self.cubes.iter()
            .map(|cube| flooded.get_present_neighbor_count(*cube))
            .sum()
    }

    fn flood(&self) -> Self {
        let mut new_cubes = CubeMap::new();

        // Start with two diagonal cubes, outside the bounds of the current cubemap
        new_cubes.add( XYZ {x: self.x_min()-1, y: self.y_min()-1, z: self.z_min()-1 } );
        new_cubes.add( XYZ {x: self.x_max()+1, y: self.y_max()+1, z: self.z_max()+1 } );
        let mut h2o_cubes = new_cubes.clone();

        // Strategy: start outside, fill inward as much as we can.
        while !new_cubes.cubes.is_empty() {
            let mut next_new_cubes = CubeMap::new();
            for new_cube in new_cubes.cubes.iter() {
                next_new_cubes.cubes.extend(
                    h2o_cubes.get_neighbors_in_bounds(*new_cube)
                    .iter()
                    .filter(|nbr| !h2o_cubes.cubes.contains(nbr) &&
                                  !new_cubes.cubes.contains(nbr) &&
                                  !next_new_cubes.cubes.contains(nbr) &&
                                  !self.cubes.contains(nbr))
                    .cloned()
                    .collect::<HashSet<XYZ>>()
                )
            }
            h2o_cubes.cubes.extend(new_cubes.cubes);
            new_cubes = next_new_cubes;
        }

        h2o_cubes
    }

    fn in_bounds(&self, xyz: &XYZ) -> bool {
        xyz.x >= self.x_min() && xyz.x <= self.x_max() &&
        xyz.y >= self.y_min() && xyz.y <= self.y_max() &&
        xyz.z >= self.z_min() && xyz.z <= self.z_max()
    }

    fn x_min(&self) -> i32 { self.cubes.iter().map(|x| x.x).min().unwrap() }
    fn y_min(&self) -> i32 { self.cubes.iter().map(|x| x.y).min().unwrap() }
    fn z_min(&self) -> i32 { self.cubes.iter().map(|x| x.z).min().unwrap() }
    
    fn x_max(&self) -> i32 { self.cubes.iter().map(|x| x.x).max().unwrap() }
    fn y_max(&self) -> i32 { self.cubes.iter().map(|x| x.y).max().unwrap() }
    fn z_max(&self) -> i32 { self.cubes.iter().map(|x| x.z).max().unwrap() }
}

pub fn day18() {
    let file = File::open("input.txt").expect("File 'input.txt' not readable.");
    let map = CubeMap {cubes : BufReader::new(file)
        .lines() // Get a line iterator
        .filter_map(|line| line.ok()) // Get Strings instead of Result
        .filter_map(|line| line.parse::<XYZ>().ok())
        .collect() };

    println!("Part 1: {}", map.surface_area()); // 4460
    println!("Part 2: {}", map.exterior_surface_area()); // 2498 too high
}

pub fn main() {
    day18()
}
