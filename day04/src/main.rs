use std::fs::File;
use std::collections::HashSet;
use std::io::{self, prelude::*, BufReader};

fn day04() {
    let file = File::open("input.txt").expect("File 'input.txt' not readable.");
    let reader = BufReader::new(file)
        .lines() // Get a line iterator
        .filter_map(|line| line.ok()); // Get Strings instead of Result

    let mut fully_overlapping: u32 = 0;
    let mut partially_overlapping: u32 = 0;

    for line in reader {
        let areas: Vec<HashSet<u32>> = line
            .split(",")                             // Split line into two assignments.
            .map(|x|                                // On each line,
                x.split("-")                        //  * split each assignment into min/max
                .map(|i| i.parse::<u32>().unwrap()) //  * convert min and max into u32's
                .collect::<Vec<u32>>()              // collect min/max vector
            ).map(|a| (a[0]..=a[1]).collect()       // Create 2x HashSet<u32> with [min..=max]
        ).collect();                                // Collect assignments into a vector

        if areas[0].is_subset(&areas[1]) || areas[1].is_subset(&areas[0]) {
            fully_overlapping += 1;
        }

        if areas[0].intersection(&areas[1]).count() > 0 {
            partially_overlapping += 1;
        }
    }

    println!("Part 1: {} are fully overlapping.", fully_overlapping);
    println!("Part 2: {} are partially overlapping.", partially_overlapping);
}

fn main() -> io::Result<()> {
    day04();
    Ok(())
}
