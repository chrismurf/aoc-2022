use std::fs;
use std::str::FromStr;
use ndarray::prelude::*;
use regex::Regex;

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

    fn dy(&self) -> isize {
        match *self {
            Dir::Up => -1,
            Dir::Down => 1,
            _ => 0,
        }
    }

    fn dx(&self) -> isize {
        match *self {
            Dir::Left => -1,
            Dir::Right => 1,
            _ => 0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Cell { Wall, Empty, OOB }

#[derive(Debug, Clone, PartialEq)]
struct Map {
    grid: Array2::<Cell>,
    loc: (usize, usize),
    dir: Dir
}

impl Map {
    fn navigate(&mut self, actions : &Vec<Action>, as_cube: bool) {
        for action in actions {
            match action {
                Action::TurnLeft => self.dir.turn_left(),
                Action::TurnRight => self.dir.turn_right(),
                Action::Move(d) => self.move_forward(*d, as_cube),
            }
        }
    }

    fn move_forward(&mut self, dist: u32, as_cube: bool) {
        for _ in 0..dist {
            let mut new_dir = self.dir;
            let mut new_y = self.loc.0 as isize;
            let mut new_x = self.loc.1 as isize;

            loop {
                // This loop tries to move one logical square,
                // though that may mean multiple squares on the grid
                new_y += self.dir.dy();
                new_x += self.dir.dx();

                // First, check if we went off the map, and deal with that.
                match self.grid.get((new_y as usize, new_x as usize)) {
                    None | Some(Cell::OOB) => {
                        // Off of grid, wrap around to other side
                        if as_cube {
                            // All the connected faces are handled correctly by normal rules above,
                            // but off the other 14 edges things get more complicated.
                            // My faces are laid out like   X 5 6
                            // at right, and my solution    X 4
                            // will only work if laid out   2 3
                            // similarly.                   1
                            if new_y < 50 && new_x < 50 && new_dir == Dir::Left {
                                // 5L -> 2L
                                (new_x, new_y) = (0, 149 - new_y);
                                new_dir = Dir::Right;
                            } else if new_y < 0 && new_x < 100 && new_dir == Dir::Up {
                                // 5T -> 1L
                                (new_x, new_y) = (0, new_x + 100);
                                new_dir = Dir::Right;
                            } else if new_y < 0 && new_x >= 100 && new_dir == Dir::Up {
                                // 6T -> 1B
                                (new_x, new_y) = (new_x - 100, 199);
                            } else if new_y < 50 && new_x >= 150 && new_dir == Dir::Right {
                                // 6R -> 3R
                                (new_x, new_y) = (99, 149 - new_y);
                                new_dir = Dir::Left;
                            } else if new_y >= 50 && new_x >= 100 && new_dir == Dir::Down {
                                //6B -> 4R
                                (new_x, new_y) = (99, new_x - 50);
                                new_dir = Dir::Left;
                            } else if new_y < 100 && new_x < 50 && new_dir == Dir::Left {
                                // 4L -> 2T
                                (new_x, new_y) = (new_y - 50, 100);
                                new_dir = Dir::Down;
                            } else if new_y < 100 && new_x >= 100 && new_dir == Dir::Right {
                                // 4R -> 6B
                                (new_x, new_y) = (new_y + 50, 49);
                                new_dir = Dir::Up;
                            } else if new_y >= 100 && new_x >= 100 && new_dir == Dir::Right {
                                // 3R -> 6R
                                (new_x, new_y) = (149, 149 - new_y);
                                new_dir = Dir::Left;
                            } else if new_y >= 150 && new_x >= 50 && new_dir == Dir::Down {
                                // 3B -> 1R
                                (new_x, new_y) = (49, new_x + 100);
                                new_dir = Dir::Left;
                            } else if new_y < 100 && new_x < 50 && new_dir == Dir::Up {
                                // 2T -> 4L
                                (new_x, new_y) = (50, new_x + 50);
                                new_dir = Dir::Right;
                            } else if new_y < 150 && new_x < 0 && new_dir == Dir::Left {
                                // 2L -> 5L
                                (new_x, new_y) = (50, 149 - new_y);
                                new_dir = Dir::Right;
                            } else if new_y >= 150 && new_x < 0 && new_dir == Dir::Left {
                                // 1L -> 5T
                                (new_x, new_y) = (new_y-100, 0);
                                new_dir = Dir::Down;
                            } else if new_y >= 150 && new_x >= 50 && new_dir == Dir::Right {
                                // 1R -> 3B
                                (new_x, new_y) = (new_y-100, 149);
                                new_dir = Dir::Up;
                            } else if new_y >= 200 && new_x < 50 && new_dir == Dir::Down {
                                // 1B -> 6T
                                (new_x, new_y) = (new_x + 100, 0);
                            } else {
                                unreachable!();
                            }
                        } else {
                            // Simple, planar case
                            if new_y < 0 { new_y = self.grid.nrows() as isize; }
                            else if new_y >= self.grid.nrows() as isize { new_y = -1; }
                            else if new_x < 0 { new_x = self.grid.ncols() as isize; }
                            else if new_x >= self.grid.ncols() as isize { new_x = -1; }
                        }
                    },

                    _ => {} // not off map, nothing to do.
                }

                // Now, assuming we are back on the map, handle it.
                match self.grid.get((new_y as usize, new_x as usize)) {
                    Some(Cell::Empty) => {
                        // Valid empty cell - move here.
                        self.loc = (new_y as usize, new_x as usize);
                        self.dir = new_dir;
                        break;
                    },
                    Some(Cell::Wall) => {
                        // Hit a wall, so return without moving further.
                        return
                    },
                    None | Some(Cell::OOB) => {
                        if as_cube { unreachable!() }
                    }
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

fn compute_password(loc: (usize, usize), dir: Dir) -> usize {
    (loc.0 + 1) * 1000 + (loc.1 + 1) * 4 + dir as usize
}

pub fn day22() {
    let file = fs::read_to_string("input.txt").expect("Couldn't read input.txt");
    let (mut map, actions) = parse_input(&file);
    let mut cube = map.clone();

    map.navigate(&actions, false);
    println!("Part 1 is {}", compute_password(map.loc, map.dir)); // 162186

    cube.navigate(&actions, true);
    println!("Part 2 is {}", compute_password(cube.loc, cube.dir)); // 55267
}

pub fn main() {
    day22()
}

#[cfg(test)]
 mod test {
    use indoc::indoc;
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
        map.navigate(&actions, false);
        assert_eq!(compute_password(map.loc, map.dir), 6032);
    }
}