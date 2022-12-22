use std::fs;
use std::str::FromStr;
use ndarray::prelude::*;
use regex::Regex;
use indoc::indoc;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Action { Move(u32), TurnLeft, TurnRight }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Dir {
    Right = 0,
    Down = 1,
    Left = 2,
    Up = 3,
}

impl Dir {
    fn turn_left(&mut self) {
        *self = match *self {
            Dir::Up => Dir::Left,
            Dir::Left => Dir::Down,
            Dir::Down => Dir::Right,
            Dir::Right => Dir::Up
        }
    }

    fn turn_right(&mut self) {
        *self = match *self {
            Dir::Up => Dir::Right,
            Dir::Left => Dir::Up,
            Dir::Down => Dir::Left,
            Dir::Right => Dir::Down
        }
    }

    fn dy_dx(&self) -> (isize, isize) {
        match *self {
            Dir::Up => (-1, 0),
            Dir::Left => (0, -1),
            Dir::Down => (1, 0),
            Dir::Right => (0, 1),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
enum Cell { Wall, Empty, OOB }

#[derive(Debug, Clone, PartialEq)]
struct Map {
    grid: Array2::<Cell>,
    loc: (usize, usize),
    dir: Dir,
}

impl Map {
    fn navigate(&mut self, actions : &Vec<Action>) {
        for action in actions {
            match action {
                Action::TurnLeft => self.dir.turn_left(),
                Action::TurnRight => self.dir.turn_right(),
                Action::Move(d) => self.move_forward(*d),
            }
        }
    }

    fn move_forward(&mut self, dist: u32) {
        for _ in 0..dist {
            let mut new_y = self.loc.0 as isize;
            let mut new_x = self.loc.1 as isize;
            let (dy, dx) = self.dir.dy_dx();

            loop {
                new_y += dy;
                new_x += dx;
                match self.grid.get((new_y as usize, new_x as usize)) {
                    None => { // Off of grid, wrap around to other side
                        if new_y < 0 { new_y = self.grid.nrows() as isize; }
                        else if new_y >= self.grid.nrows() as isize { new_y = -1; }
                        else if new_x < 0 { new_x = self.grid.ncols() as isize; }
                        else if new_x >= self.grid.ncols() as isize { new_x = -1; }
                    },
                    Some(Cell::OOB) => continue, // Invalid cell, keep incrementing
                    Some(Cell::Wall) => return, // Can't move - wall
                    Some(Cell::Empty) => { // Valid, empty cell - move here.
                        self.loc = (new_y as usize, new_x as usize);
                        break;
                    },
                }
            };
        }
    }
}

impl FromStr for Map {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let map_width = s.lines().map(|x| x.len()).max().unwrap();
        let map_height = s.lines().count();
    
        let mut grid = Array2::<Cell>::from_elem((map_height, map_width), Cell::OOB);
    
        for (row, line) in s.lines().enumerate() {
            for (col, ch) in line.chars().enumerate() {
                *grid.get_mut((row, col)).unwrap() = match ch {
                    '.' => Cell::Empty,
                    '#' => Cell::Wall,
                    _ => Cell::OOB
                }
            }
        }

        let col = grid.row(0).iter()
            .position(|x| match x { Cell::Empty => true, _ => false, }).unwrap();
        Ok( Self { grid, loc: (0, col), dir: Dir::Right } )
    }    
}

fn parse_input(s: &str) -> (Map, Vec<Action>) {
    let (map_str, actions_str) = s
        .split_once("\n\n")
        .unwrap();
    
    let map = map_str.parse::<Map>().unwrap();
    
    let re = Regex::new(r"([0-9]+|L|R)").unwrap();
    let actions = re.find_iter(actions_str)
        .map(|x| match x.as_str() {
            "L" => Action::TurnLeft,
            "R" => Action::TurnRight,
            v => Action::Move(v.parse().unwrap()),
        }).collect::<Vec<Action>>();
    (map, actions)
}

pub fn day22() {
    let file = fs::read_to_string("input.txt").expect("Couldn't read input.txt");
    let (mut map, actions) = parse_input(&file);
    map.navigate(&actions);

    let password = (map.loc.0 + 1) * 1000 + (map.loc.1 + 1) * 4 + map.dir as usize;
    println!("Part 1 is {}", password);
}

pub fn main() {
    day22()
}

#[cfg(test)]
 mod test {
    use super::*;

    #[test]
    fn test_one() {
        let input = indoc! {"
        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5"};
        let (mut map, actions) = parse_input(input);
        map.navigate(&actions);

        let password = (map.loc.0 + 1) * 1000 + (map.loc.1 + 1) * 4 + map.dir as usize;
        assert_eq!(password, 6032);
    }
}