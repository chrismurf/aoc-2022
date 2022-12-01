use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn find_maximums() {
    let mut current_total: u64 = 0;
    let mut totals = Vec::new();

    let file = File::open("input.txt")
        .unwrap_or_else(|_| panic!("File 'input.txt' not readable.") );

    for line in BufReader::new(file).lines().filter_map(|line| line.ok()) {
        if line.trim().is_empty() {
            totals.push(current_total);
            current_total = 0;
        } else {
            current_total += line.parse::<u64>().unwrap();
        }
    }

    totals.sort_by(|a, b| b.cmp(a));

    assert!(totals.len() >= 3);
    println!("Top value: {}", totals[0]);
    println!("Sum of top three values: {}", totals[0..3].iter().sum::<u64>());
}

fn main() -> io::Result<()> {
    find_maximums();

    Ok(())
}

// Top value: 74198
// Sum of top three values: 209914
