use std::str::FromStr;
use std::collections::{HashSet, HashMap};
use std::num::ParseIntError;
use std::io::{self, prelude::*};
use colored::*;

#[derive(Debug)]
pub enum Instruction {
    NoOp,
    AddX(i32)
}

impl FromStr for Instruction {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(" ").collect();
        match parts[0] {
            "noop" => Ok(Self::NoOp),
            "addx" => {
                let x : i32 = parts[1].parse()?;
                Ok(Self::AddX(x))
            },
            _ => unreachable!()
        }
    }
}

#[derive(Default, Debug)]
pub struct CPU {
    pub cycle: u32,
    pub register_x: i32,
    pub watchpoints: HashSet<u32>,
    pub watchvalues: HashMap<u32, i32>
}

impl CPU {
    const SCREEN_WIDTH : u32 = 40;
    const CHAR_WIDTH : u32 = 5;

    pub fn new() -> Self {
        Self {
            cycle: 1,
            register_x: 1,
            watchpoints: HashSet::new(),
            watchvalues: HashMap::new(),
        }
    }

    pub fn add_watchpoint(&mut self, watch: u32) {
        self.watchpoints.insert(watch);
    }

    fn do_cycle(&mut self) {
        // Check for any watchpoints
        if self.watchpoints.contains(&self.cycle) {
            self.watchvalues.insert(self.cycle, self.register_x);
        }

        // Use one color per character because we cool that way.
        let scan_pos = ((self.cycle-1) % CPU::SCREEN_WIDTH) as i32;
        const RAINBOW_PLUS_WHITE : [colored::Color; 8] = [
            Color::TrueColor {r: 255, g: 0, b: 0},
            Color::TrueColor {r: 255, g: 127, b: 0},
            Color::TrueColor {r: 255, g: 255, b: 0},
            Color::TrueColor {r: 0, g: 255, b: 0},
            Color::TrueColor {r: 0, g: 0, b: 255},
            Color::TrueColor {r: 75, g: 0, b: 130},
            Color::TrueColor {r: 148, g: 0, b: 211},
            Color::TrueColor {r: 255, g: 255, b: 255},
        ];
        let color = RAINBOW_PLUS_WHITE[(scan_pos / CPU::CHAR_WIDTH as i32) as usize];

        // Print values as appropriate
        if scan_pos == 0 { println!(); }
        if (self.register_x - scan_pos).abs() <= 1 {
            print!("{}", "█".color(color));
            io::stdout().flush().unwrap();
        } else {
            print!(" ");
            io::stdout().flush().unwrap();
        }

        // Increment program counter
        self.cycle += 1;
    }

    pub fn execute(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::NoOp => {
                self.do_cycle();
            },

            Instruction::AddX(arg) => {
                self.do_cycle();
                self.do_cycle();
                self.register_x += arg;
            }
        }
    }
}
