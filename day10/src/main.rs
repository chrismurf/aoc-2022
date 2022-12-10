use std::fs::File;
use std::io::{self, prelude::*, BufReader};

mod elf;

pub fn day10() {
    let file = File::open("input.txt").expect("File 'input.txt' not readable.");

    let mut cpu = elf::CPU::new();
    // Add Watchpoints for Part 1
    [20, 60, 100, 140, 180, 220].iter().for_each(|w| cpu.add_watchpoint(*w));

    let reader = BufReader::new(file)
        .lines() // Get a line iterator
        .filter_map(|line| line.ok()); // Get Strings instead of Result

    // Print top of screen
    print!("\n{}", "-".repeat(40));
    // Execute Program
    for line in reader {
        cpu.execute(&line.parse().expect("Failed to parse input line."));
    }
    // Print bottom of screen
    println!("\n{}\n", "-".repeat(40));

    // Compute and print total for part 1
    let part1_total : i32 = cpu.watchvalues.iter().map(|(cyc, x)| *cyc as i32 * x).sum();
    println!("Part 1 Total: {}", part1_total);
}

pub fn main() -> io::Result<()> {
    day10();
    Ok(())
}

// 14320
// PCPBKAPJ