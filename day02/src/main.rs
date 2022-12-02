use std::fs::File;
use std::io::{self, prelude::*, BufReader};

// This implementation is entitled "Chris really likes Rust's match syntax"

#[derive(PartialEq)]
enum Play { Rock, Paper, Scissors }

#[derive(PartialEq)]
enum Ending { Win, Loss, Draw }

fn round_score(my_play: &Play, ending: &Ending) -> i32 {
    return match my_play {
        Play::Rock => 1,
        Play::Paper => 2,
        Play::Scissors => 3,
    } + match ending {
        Ending::Win => 6,
        Ending::Draw => 3,
        Ending::Loss => 0,
    };
}

fn get_my_ending(my_play: &Play, their_play: &Play) -> Ending {
    return match my_play {
        Play::Rock => match their_play {
            Play::Rock => Ending::Draw,
            Play::Paper => Ending::Loss,
            Play::Scissors => Ending::Win
        },
        Play::Paper => match their_play {
            Play::Rock => Ending::Win,
            Play::Paper => Ending::Draw,
            Play::Scissors => Ending::Loss
        },
        Play::Scissors => match their_play {
            Play::Scissors => Ending::Draw,
            Play::Rock => Ending::Loss,
            Play::Paper => Ending::Win
        },
    }
}

fn get_my_play(their_play: &Play, desired_ending: &Ending) -> Play {
    return match their_play {
        Play::Rock => match desired_ending {
            Ending::Draw => Play::Rock,
            Ending::Win => Play::Paper,
            Ending::Loss => Play::Scissors
        },
        Play::Paper => match desired_ending {
            Ending::Loss => Play::Rock,
            Ending::Draw => Play::Paper,
            Ending::Win => Play::Scissors
        },
        Play::Scissors => match desired_ending {
            Ending::Win => Play::Rock,
            Ending::Loss => Play::Paper,
            Ending::Draw => Play::Scissors,
        },
    }
}

fn rock_paper_scissors() {
    let file = File::open("input.txt")
        .unwrap_or_else(|_| panic!("File 'input.txt' not readable.") );


    let mut total_score_part1 : i32 = 0;
    let mut total_score_part2 : i32 = 0;

    for line in BufReader::new(file).lines().filter_map(|line| line.ok()) {
        let columns : Vec<&str> = line.split(" ").collect();
        let their_play : Play = match columns[0] {
            "A" => Play::Rock,
            "B" => Play::Paper,
            "C" => Play::Scissors,
            &_ => todo!("Unrecognized play found!")
        };

        let my_play_part1 : Play = match columns[1] {
            "X" => Play::Rock,
            "Y" => Play::Paper,
            "Z" => Play::Scissors,
            &_ => todo!("Unrecognized play found!")
        };

        let desired_ending_part2 : Ending = match columns[1] {
            "X" => Ending::Loss,
            "Y" => Ending::Draw,
            "Z" => Ending::Win,
            &_ => todo!("Unrecognized ending found!")
        };


        let ending_part1 = get_my_ending(&my_play_part1, &their_play);
        total_score_part1 += round_score(&my_play_part1, &ending_part1);

        let my_play_part2 = get_my_play(&their_play, &desired_ending_part2);
        total_score_part2 += round_score(&my_play_part2, &desired_ending_part2);

    }

    println!("Final score for Part 1: {}", total_score_part1);
    println!("Final score for Part 2: {}", total_score_part2);
}

fn main() -> io::Result<()> {
    rock_paper_scissors();
    Ok(())
}
// Final score: 15572
// Final score: 16098

