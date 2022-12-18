use std::fs::File;
use std::str::FromStr;
use std::collections::HashSet;
use std::io::{prelude::*, BufReader};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct XYZ { x: i32, y: i32, z: i32}

impl XYZ {
    fn new(x: i32, y: i32, z: i32) -> Self { Self {x, y, z} }

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
        let parts : Vec<i32> = s.split(",").map(|x| x.parse().unwrap()).collect();
        Ok(Self {x: parts[0], y: parts[1], z: parts[2] })
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

    fn get_neighbors_in_bounds(&self, xyz: XYZ, min: XYZ, max: XYZ) -> HashSet<XYZ> {
        self.get_neighbors(xyz)
            .iter()
            .filter(|xyz| 
                min.x <= xyz.x && xyz.x <= max.x &&
                min.y <= xyz.y && xyz.y <= max.y &&
                min.z <= xyz.z && xyz.z <= max.z
            )
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
        // Sum the number of faces without neighbors
        self.cubes.iter()
            .map(|cube| 6 - self.get_present_neighbor_count(*cube))
            .sum()
    }

    fn exterior_surface_area(&self) -> usize {
        // Generate a flooded exterior
        let flooded = self.flood();
        // Sum surfaces touching a flooded cube
        self.cubes.iter()
            .map(|cube| flooded.get_present_neighbor_count(*cube))
            .sum()
    }

    fn flood(&self) -> Self {
        let mut h2o_cubes = CubeMap::new();
        let mut new_cubes = CubeMap::new();

        // Start with two diagonal cubes, just at the bounds of the current cubemap
        let h2o_min = self.min_bound().left().down().back();
        new_cubes.add( h2o_min.clone() );

        let h2o_max = self.max_bound().right().up().fwd();
        new_cubes.add( h2o_max.clone() );

        // Strategy: for each new "water" cube, expand to empty neighbor slots within
        // the min/max bounds we've set.  Empty here means "not water" and also "not cube".
        // Repeat until we stop adding new water spots, at which point we're flooded.
        while !new_cubes.cubes.is_empty() {
            let mut next_new_cubes = CubeMap::new();
            for new_cube in new_cubes.cubes.iter() {
                next_new_cubes.cubes.extend(
                    h2o_cubes.get_neighbors_in_bounds(*new_cube, h2o_min, h2o_max)
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

    // These are kinda ugly.
    fn min_bound(&self) -> XYZ {
        self.cubes.iter()
            .fold(XYZ::new(i32::MAX, i32::MAX, i32::MAX), |mut min, xyz| {
                if xyz.x < min.x { min.x = xyz.x }
                if xyz.y < min.y { min.y = xyz.y }
                if xyz.z < min.z { min.z = xyz.z }
                min
            })
    }

        // These are kinda ugly.
    fn max_bound(&self) -> XYZ {
        self.cubes.iter()
            .fold(XYZ::new(i32::MIN, i32::MIN, i32::MIN), |mut max, xyz| {
                if xyz.x > max.x { max.x = xyz.x }
                if xyz.y > max.y { max.y = xyz.y }
                if xyz.z > max.z { max.z = xyz.z }
                max
            })
    }
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
