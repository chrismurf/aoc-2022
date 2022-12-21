use std::fs::File;
use std::str::FromStr;
use std::collections::HashMap;
use std::io::{prelude::*, BufReader};

#[derive(Debug, Clone, PartialEq)]
enum Operation {
    Value(i64),
    Mul(String, String),
    Div(String, String),
    Add(String, String),
    Sub(String, String),
}

#[derive(Debug, Clone, PartialEq)]
struct MonkeyAssignment {
    name: String,
    job: Operation
}

impl FromStr for MonkeyAssignment {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts : Vec<&str> = s.split(" ").collect();
        let name = parts[0].replace(":","").to_string();
        if parts.len() == 2 {
            Ok( MonkeyAssignment { name, job: Operation::Value(parts[1].parse().unwrap()) } )
        } else {
            let var1 = parts[1].to_string();
            let var2 = parts[3].to_string();
            let job = match parts[2] {
                "+" => Operation::Add(var1, var2),
                "-" => Operation::Sub(var1, var2),
                "*" => Operation::Mul(var1, var2),
                "/" => Operation::Div(var1, var2),
                _ => unreachable!()
            };
            Ok(MonkeyAssignment { name, job })
        }
    }
}

impl MonkeyAssignment {
    fn evaluate(&self, others: &HashMap<String, MonkeyAssignment>) -> i64 {
        match &self.job {
            Operation::Value(v) => *v,
            Operation::Add(a, b) => others[a].evaluate(others) + others[b].evaluate(others),
            Operation::Sub(a, b) => others[a].evaluate(others) - others[b].evaluate(others),
            Operation::Mul(a, b) => others[a].evaluate(others) * others[b].evaluate(others),
            Operation::Div(a, b) => others[a].evaluate(others) / others[b].evaluate(others),
        }
    }
}

pub fn day21() {
    let file = File::open("input.txt").expect("File 'input.txt' not readable.");
    let mut assignments : HashMap<String, MonkeyAssignment> = BufReader::new(file)
        .lines() // Get a line iterator
        .filter_map(|line| line.ok()) // Get Strings instead of Result
        .filter_map(|line| line.parse::<MonkeyAssignment>().ok())
        .map(|ass| (ass.name.clone(), ass))
        .collect();

    println!("Part 1 evaluates to {}", assignments["root"].evaluate(&assignments));

    // Mess with assignments a bit...
    let mut root = assignments.remove("root").unwrap();
    let (a, b) = match root.job {
        Operation::Add(a, b) | Operation::Sub(a, b) | Operation::Mul(a, b) | Operation::Div(a, b) => (a, b),
        _ => unreachable!()
    };

    // Found by just narrowing window by large steps, since it was linear it can only be so weird.
    // This is ... dirty... but I didn't feel like inverting all the operations.  Plus side - going to sleep at 1am!
    //for i in (3699945358563..=3699945358567) {
    let v = 3699945358564;
    assignments.get_mut("humn").unwrap().job = Operation::Value(v);
    println!("Part 2: {} results in a difference of {}", v, assignments[&a].evaluate(&assignments)-assignments[&b].evaluate(&assignments));    
    //}
}

pub fn main() {
    day21()
}

#[cfg(test)]
 mod test {
    use super::*;

    #[test]
    fn test_one() {
    }
}