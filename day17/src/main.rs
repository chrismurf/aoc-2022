use std::collections::HashSet;
use std::fs;
use std::time::SystemTime;
use ndarray::prelude::*;

#[derive(Debug, Clone)]
struct Piece {
    filled : HashSet<(usize, usize)>,
}

#[derive(Debug, Clone)]
struct Chamber {
    width: usize,
    current_height: usize,
    grid : Array2<bool>
}

impl Chamber {
    fn new(height: usize, width: usize) -> Self {
        Self { width, current_height: 0, grid : Array2::<bool>::from_elem((height, width), false) }
    }

    fn add_piece(&mut self, piece: &Piece, pos: (i32, i32)) {
        let mut max_row : usize = 0;
        for (px, py) in &piece.filled {
            if (pos.1 as usize + *py) > max_row {
                max_row = pos.1 as usize + *py
            }
            *self.grid.get_mut([(pos.1 as usize + *py), (pos.0 as usize + *px)]).unwrap() = true;
        }
        if max_row as usize >= self.current_height {
            self.current_height = max_row + 1;
        }
    }

    fn print(&self) {
        for row in (0..=self.current_height).rev() {
            println!("{}", self.grid.row(row).iter().map(|x| if *x {'#'} else {' '} ).collect::<String>());
        }
    }

    fn collides(&self, piece: &Piece, pos: (i32, i32)) -> bool {
        for (px, py) in &piece.filled {
            // If we run off right side, or off bottom, that's a collision
            if ((pos.0 + *px as i32) < 0) ||
               ((pos.0 + *px as i32) >= self.width as i32) ||
               ((pos.1 + *py as i32) < 0) { return true }
            
            if *self.grid.get([(pos.1 + *py as i32) as usize, (pos.0 + *px as i32) as usize])
                .unwrap() {return true }
        }
        return false;
    }

    fn full_top_row(&self) -> bool {
        return self.grid.row(self.current_height-1).iter().all(|x| *x);
    }
}

fn get_pieces() -> Vec<Piece> {
    vec![
        Piece { filled: HashSet::from([(0, 0), (1, 0), (2, 0), (3, 0)]) },
        Piece { filled: HashSet::from([(0, 1), (1, 0), (1, 1), (1, 2), (2, 1)]) },
        Piece { filled: HashSet::from([(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)]) },
        Piece { filled: HashSet::from([(0, 0), (0, 1), (0, 2), (0, 3)]) },
        Piece { filled: HashSet::from([(0, 0), (0, 1), (1, 0), (1, 1)]) },
    ]
}

pub fn day17() {
    let dx_input : Vec<i32> = fs::read_to_string("input.txt")
        .unwrap()
        .chars()
        .filter(|x| *x == '>' || *x == '<')
        .map(|dir| if dir == '>' { 1 } else { -1 })
        .collect();
    let mut dx_iter = dx_input.iter().cycle();
    let pieces = get_pieces();
    let piece_iter = pieces.iter().cycle();
    let mut chamber = Chamber::new(50000000, 7);

    let now = SystemTime::now();

    for (piece_num, piece) in piece_iter.enumerate() {
        let mut x = 2;
        let mut y = chamber.current_height as i32 + 3;

        for dx in &mut dx_iter {
            if !chamber.collides(piece, (x+dx, y)) {
                x += dx;
            }
            if !chamber.collides(piece, (x, y - 1)) {
                y -= 1;
            } else {
                chamber.add_piece(piece, (x, y));
                break;
            }
        }

        if piece_num == 2021 {
            println!("Part 1: {}", chamber.current_height);
            break;
        }

//        println!("{},{}", piece_num, chamber.current_height);
        // I output the data part 2, and computed the autocorrelation with numpy to find that
        // the *change* in the height "signal" repeats every 1730 time steps, increasing by
        // 2644 each time -- after the first round due to startup transients.
        // 1000000000000 % 1730 == 140
        // 1000000000000 // 1730 == 578034682
        // The height at step 140+1730 (to get past startup) was 2878, plus 578034681*2644,
        // which is 1528323699442.
    }

}

pub fn main() {
    day17()
}
