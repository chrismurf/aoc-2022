use std::ops::{Add, Sub, AddAssign};
use std::cmp::max;
use std::str::FromStr;
use std::num::ParseIntError;    

/// A Cartesian Point
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point { x: i32, y: i32 }

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
       Self {x, y}
   }

   pub fn is_adjacent(&self, other: &Point) -> bool {
        (self.x - other.x).abs() <= 1 && (self.y - other.y).abs() <= 1
    }
}

impl Add<Delta> for Point {
    type Output = Self;

    fn add(self, delta: Delta) -> Self::Output {
        Self { x: self.x + delta.dx, y: self.y + delta.dy }
    }
}

impl AddAssign<Delta> for Point {
    fn add_assign(&mut self, delta: Delta) {
        self.x += delta.dx;
        self.y += delta.dy;
    }
}

impl Sub for Point {
    type Output = Delta;

    fn sub(self, other: Point) -> Self::Output {
        Delta { dx: self.x - other.x, dy: self.y - other.y }
    }
}

/// Difference between two cartesian points
#[derive(Debug, Clone, Copy)]
pub struct Delta { dx: i32, dy: i32 }

impl FromStr for Delta {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (direction, distance_str) = s.split_once(" ").unwrap();
        let distance : i32 = distance_str.parse().unwrap();
        match direction {
            "U" => Ok(Delta {dx: 0, dy: distance} ),
            "D" => Ok(Delta {dx: 0, dy: -distance} ),
            "L" => Ok(Delta {dx: -distance, dy: 0} ),
            "R" => Ok(Delta {dx: distance, dy: 0} ),
            _ => unreachable!()
        }
    }
}

impl Delta {
    pub fn abs(&self) -> i32 {
        // No need for Pythagorus today.
        max(self.dx.abs(), self.dy.abs())
    }

    pub fn unit_direction(&self) -> Self {
        Self {dx: self.dx.signum(), dy: self.dy.signum() }
    }
}
