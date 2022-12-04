use std::fs::File;
use std::collections::HashSet;
use std::io::{self, prelude::*, BufReader};

fn fully_overlapping(area_a : &HashSet<u32>, area_b : &HashSet<u32>) -> bool {
    return area_a.is_subset(area_b) || area_b.is_subset(area_a);
}

fn overlapped_at_all(area_a : &HashSet<u32>, area_b : &HashSet<u32>) -> bool {
    return area_a.intersection(area_b).count() > 0;
}

fn day04() {
    let file = File::open("input.txt")
        .unwrap_or_else(|_| panic!("File 'input.txt' not readable.") );
    let reader = BufReader::new(file)
        .lines() // Get a line iterator
        .filter_map(|line| line.ok()); // Get Strings instead of Result

    let mut count: u32 = 0;
    let mut count2: u32 = 0;

    for line in reader {
        let areas: Vec<HashSet<u32>> = line
            .split(",")                             // Split line into two assignments.
            .map(|x|                                // On each line,
                x.split("-")                        //  * split each assignment into min/max
                .map(|i| i.parse::<u32>().unwrap()) //  * convert min and max into u32's
                .collect::<Vec<u32>>()              // collect min/max vector
            ).map(|a| (a[0]..=a[1]).collect()       // Create 2x HashSet<u32> with [min..=max]
        ).collect();                                // Collect assignments into a vector

        if fully_overlapping(&areas[0], &areas[1]) { 
            count += 1;
        }
        if overlapped_at_all(&areas[0], &areas[1]) { 
            count2 += 1;
        }
    }

    println!("Final score for Part 1: {}", count);
    println!("Final score for Part 2: {}", count2);
}

fn main() -> io::Result<()> {
    day04();
    Ok(())
}
