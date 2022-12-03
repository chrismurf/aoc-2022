use std::fs::File;
use std::collections::HashSet;
use std::io::{self, prelude::*, BufReader};

const LETTERS: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

fn find_character_intersection(parts: Vec<String>) -> char {
    let overlap : HashSet<char> = parts.iter()
        .map(|x| x.chars().collect::<HashSet<char>>())
        .fold(
            LETTERS.chars().collect(), 
            |acc, el| acc.intersection(&el).map(|x| x.to_owned()).collect()
        );

    assert!(overlap.len() == 1);
    return overlap.iter().next().unwrap().to_owned();
}

fn part_1() {
    let file = File::open("input.txt")
        .unwrap_or_else(|_| panic!("File 'input.txt' not readable.") );
    let reader = BufReader::new(file).lines().filter_map(|line| line.ok());

    let mut total: usize = 0;

    for line in reader {
        let parts : (&str, &str) = line.split_at(line.len()/2);
        let intersection = find_character_intersection(vec!(parts.0.to_owned(), parts.1.to_owned()));
        total += LETTERS.find(intersection).unwrap() + 1;
    }

    println!("Final score for Part 1: {}", total);
}

fn part_2() {
    let  file = File::open("input.txt")
        .unwrap_or_else(|_| panic!("File 'input.txt' not readable.") );
    let mut reader = BufReader::new(file).lines().filter_map(|line| line.ok());

    let mut total: usize = 0;

    loop {
        let opts = vec!(reader.next(), reader.next(), reader.next());
        if opts.iter().any(|x| x.is_none()) { break; }
        let parts = opts.iter().map(|x| x.clone().unwrap()).collect();
        let intersection = find_character_intersection(parts);
        total += LETTERS.find(intersection).unwrap() + 1;
    }

    println!("Final score for Part 2: {}", total);
}

fn main() -> io::Result<()> {
    part_1();
    part_2();
    Ok(())
}

// Final score for Part 1: 8202
// Final score for Part 2: 2864