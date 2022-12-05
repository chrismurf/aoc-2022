use std::fs::File;
use std::collections::VecDeque;
use std::io::{self, prelude::*, BufReader};

#[derive(Default, Debug)]
struct MoveInstruction {
    quantity : usize,
    from : usize,
    to : usize
}

impl MoveInstruction {
    fn parse_line(line: &str) -> Self {
        let parts : Vec<&str> = line.split(" ").collect();
        assert!(parts.len() == 6);
        let quantity = parts[1].parse::<usize>().unwrap();
        // Convert to zero indexing because we're not animals.
        let from = parts[3].parse::<usize>().unwrap() - 1;
        let to = parts[5].parse::<usize>().unwrap() - 1;
        MoveInstruction {quantity, from, to}
    }
}

#[derive(Default, Debug)]
struct CranePier {
    stacks: Vec<VecDeque<char>>
}

impl CranePier {
    fn new() -> Self {
        Self { stacks: Vec::new() }
    }

    fn prepend_box_layer(&mut self, chars: &VecDeque<char>) {
        // Get the letters with box contents
        let values : Vec<&char> = chars.range(1..).step_by(4).collect();

        // Ensure we have enough VecDeque's, since they didn't give us headings first... :-(
        while self.stacks.len() < values.len() {
            self.stacks.push(VecDeque::new());
        }

        // Now add letters onto "bottom" of VecDeques, since we're working top down
        for idx in 0..values.len() {
            if *values[idx] == ' ' {
                continue;
            } else {
                self.stacks[idx].push_front(*values[idx]);
            }
        }
    }

    fn move_boxes_sequentially(&mut self, instruction: &MoveInstruction) {
        for _ in 0..instruction.quantity {
            let moving = self.stacks[instruction.from].pop_back().unwrap();
            self.stacks[instruction.to].push_back(moving);
        }
    }

    fn move_boxes_enmasse(&mut self, instruction: &MoveInstruction) {
        let idx = self.stacks[instruction.from].len() - instruction.quantity;
        let mut moving : VecDeque<char> = self.stacks[instruction.from].drain(idx..).collect();
        self.stacks[instruction.to].append(&mut moving);
    }

    fn top_boxes(&self) -> String {
        return self.stacks.iter().map(|x| x.back().unwrap()).collect();
    }
}

fn day05() {
    let file = File::open("input.txt").expect("File 'input.txt' not readable.");
    let reader = BufReader::new(file)
        .lines() // Get a line iterator
        .filter_map(|line| line.ok()); // Get Strings instead of Result

    let mut crane_pier_1 = CranePier::new();
    let mut crane_pier_2 = CranePier::new();

    for line in reader {
        if line.is_empty() { continue; } // skip empty line
        let chars : VecDeque<char> = line.chars().collect();
        if chars[1] == '1' { continue; } // skip index line
        if chars[0] == 'm' {
            let instruction = MoveInstruction::parse_line(&line);
            crane_pier_1.move_boxes_sequentially(&instruction);
            crane_pier_2.move_boxes_enmasse(&instruction);
        } else {
            crane_pier_1.prepend_box_layer(&chars);
            crane_pier_2.prepend_box_layer(&chars);
        }
    }

    println!("Part 1 top boxes: '{}'", crane_pier_1.top_boxes());
    println!("Part 2 top boxes: '{}'", crane_pier_2.top_boxes());
}

fn main() -> io::Result<()> {
    day05();
    Ok(())
}

// Part 1 top boxes: 'SHQWSRBDL'
// Part 2 top boxes: 'CDTQZHBRS'