use indoc::indoc;
use regex::Regex;
use std::collections::VecDeque;
use std::str::FromStr;
use std::{fs, io};

#[derive(Debug, Clone)]
pub struct Error;

#[derive(Debug, Clone)]
pub enum Operation {
    Mul(u64),
    Add(u64),
    Square,
}

impl Operation {
    fn evaluate(&self, old: &u64) -> u64 {
        match self {
            Self::Mul(i) => old * i,
            Self::Add(i) => old + i,
            Self::Square => old * old,
        }
    }
}

impl FromStr for Operation {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(" ").collect();
        if parts[0] == "*" && parts[1] == "old" {
            Ok(Operation::Square)
        } else if parts[0] == "*" {
            Ok(Operation::Mul(parts[1].parse().unwrap()))
        } else if parts[0] == "+" {
            Ok(Operation::Add(parts[1].parse().unwrap()))
        } else {
            Err(Error {})
        }
    }
}

#[derive(Debug, Clone)]
pub struct Monkey {
    pub id: u32,
    pub items: VecDeque<u64>,
    pub operation: Operation,
    pub test_divisor: u64,
    pub test_true_dest: usize,
    pub test_false_dest: usize,
    pub num_inspections: u32,
}

impl Monkey {
    fn inspect_next<F>(&mut self, normalize: &F) -> Option<(u64, usize)>
    where
        F: Fn(u64) -> u64,
    {
        match self.items.pop_front() {
            None => {
                return None;
            }
            Some(item) => {
                self.num_inspections += 1;
                let mut new = self.operation.evaluate(&item);
                new = normalize(new);
                if &new % &self.test_divisor == 0 {
                    return Some((new, self.test_true_dest));
                } else {
                    return Some((new, self.test_false_dest));
                }
            }
        }
    }
}

impl FromStr for Monkey {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = indoc! {r#"
        Monkey (?P<id>\d+):
        \s*Starting items: (?P<items>[0-9, ]+)
        \s*Operation: new = old (?P<operation>.+)
        \s*Test: divisible by (?P<test_divisor>\d+)
        \s*If true: throw to monkey (?P<test_true_dest>\d+)
        \s*If false: throw to monkey (?P<test_false_dest>\d+)"#};

        let monkey_regex = Regex::new(&re).unwrap();

        let cap = monkey_regex.captures(s).unwrap();
        let id: u32 = cap["id"].parse().unwrap();
        let items: VecDeque<u64> = cap["items"]
            .split(", ")
            .map(|x| x.parse::<u64>().unwrap())
            .collect();
        let operation: Operation = cap["operation"].parse().unwrap();
        let test_divisor: u64 = cap["test_divisor"].parse().unwrap();
        let test_true_dest: usize = cap["test_true_dest"].parse().unwrap();
        let test_false_dest: usize = cap["test_false_dest"].parse().unwrap();

        Ok(Self { id, items, operation, test_divisor, test_true_dest, test_false_dest, num_inspections: 0 })
    }
}

pub fn run_part<F>(part: u32, mut monkeys: Vec<Monkey>, cycles: u32, normalize: &F)
where
    F: Fn(u64) -> u64,
{
    for _ in 1..=cycles {
        for i in 0..monkeys.len() {
            loop {
                let result: Option<(u64, usize)>;
                {
                    result = monkeys[i].inspect_next(normalize);
                }
                if let Some((item, dest)) = result {
                    monkeys[dest].items.push_back(item);
                } else {
                    break;
                }
            }
        }
    }

    let mut inspections: Vec<u32> = monkeys.iter().map(|m| m.num_inspections).collect();
    inspections.sort_by(|a, b| b.cmp(a));
    println!(
        "Answer for part {} is {}",
        part,
        inspections[0] as u128 * inspections[1] as u128
    );
}

pub fn day11() {
    let file = fs::read_to_string("input.txt").expect("File 'input.txt' not readable.");
    let monkey_defs = file.split("\n\n").collect::<Vec<&str>>();
    let mut monkeys: Vec<Monkey> = Vec::new();
    for monkey_def in monkey_defs {
        monkeys.push(monkey_def.parse().unwrap())
    }

    // For part two, we need the LCM of the divisors (which are all prime)
    let lcm: u64 = monkeys.iter().map(|x| x.test_divisor).product();

    run_part(1, monkeys.clone(), 20, &|x| x / 3);
    run_part(2, monkeys, 10000, &|x| x % lcm);
}

pub fn main() -> io::Result<()> {
    day11();
    Ok(())
}

// Answer for part 1 is 55216
// Answer for part 2 is 12848882750
