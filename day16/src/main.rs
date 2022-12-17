use std::{fs, io};
use std::collections::{HashMap, HashSet};
use std::io::BufRead;
use std::io::Write;
use lazy_static::lazy_static;
use regex::Regex;
use std::str::FromStr;
use rand::seq::SliceRandom; 
use rand::{Rng, thread_rng};

#[derive(Debug, Clone)]
pub struct Error;

#[derive(Debug, Clone)]
struct Map {
    valves: HashMap<String, Valve>
}

impl Map {
    fn from_filename(filename: &str) -> Self {
        let file = fs::File::open(filename).expect("File not readable.");
        let valves : HashMap<String, Valve> = io::BufReader::new(file)
            .lines() // Get a line iterator
            .filter_map(|line| line.ok()) // Get Strings instead of Result
            .map(|line| {
                let valve : Valve = line.parse().unwrap();
                (valve.name.clone(), valve)
            }).collect();
        Self { valves }
    }

    fn to_graphviz_file(&self, filename: &str) {
        let mut file = fs::File::create(filename).expect("Couldn't create file.");
        writeln!(&mut file, "strict graph {{").unwrap();

        for valve in self.valves.values() {
            let color = if valve.flow_rate > 0 { "yellowgreen" } else { "yellow" };
            writeln!(&mut file, "{} [style=filled, label=\"{}={}\", fontsize=8, color={}]", valve.name, valve.name, valve.flow_rate, color).unwrap();
            for (neighbor, cost) in &valve.neighbors {
                writeln!(&mut file, "{} -- {} [label={}, fontsize=8]", valve.name, neighbor, cost).unwrap();
            }
        }

        writeln!(&mut file, "}}").unwrap();

    }

    fn simplify(&mut self) {
        let valve_names : HashSet<String> = self.valves.keys().map(|x| x.to_owned()).collect();
        for name in valve_names {
            {
                let valve = self.valves.get(&name).unwrap().clone();

                if valve.flow_rate == 0 && valve.neighbors.len() == 2 {
                    let mut neighbors = valve.neighbors.iter();
                    let (n1_name, n1_cost) = neighbors.next().unwrap().clone();
                    let (n2_name, n2_cost) = neighbors.next().unwrap().clone();
                    let cost = n1_cost + n2_cost;

                    {
                        let n1 = &mut self.valves.get_mut(n1_name).unwrap();
                        n1.neighbors.remove(&name);
                        n1.neighbors.insert(n2_name.to_string(), cost);
                    }

                    {
                        let n2 = &mut self.valves.get_mut(n2_name).unwrap();
                        n2.neighbors.remove(&name);
                        n2.neighbors.insert(n1_name.to_string(), cost);
                    }

                    self.valves.remove(&name);
                }
            }
        }
    }

    fn max_flow_within_time(&self, start: String, max_time: u32) {

    }
}


#[derive(Debug, Clone)]
struct Valve {
    name: String,
    flow_rate: i32,
    open: bool,
    neighbors: HashMap<String, u32> // name, cost
}

impl FromStr for Valve {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"Valve ([A-Z][A-Z]) has flow rate=(\d+); tunnels? leads? to valves? (.*)").unwrap();
        }
   
        let caps = RE.captures(s).unwrap();
        let name = caps.get(1).unwrap().as_str().to_owned();
        let flow_rate = caps.get(2).unwrap().as_str().parse::<i32>().unwrap();
        let neighbors : HashMap<String, u32> = caps.get(3).unwrap().as_str().split(", ").map(|x| (x.to_string(), 1)).collect();
        Ok(Valve {name, flow_rate, open: false, neighbors})
    }
}

pub fn day16() -> Result<(), Error> {
    let mut map = Map::from_filename("input.txt");
    map.to_graphviz_file("unsimplified.gv");
    map.simplify();
    map.to_graphviz_file("simplified.gv");
    map.max_flow_within_time("AA".to_string(), 30);

    Ok(())
}

pub fn main() -> Result<(), Error> {
    day16()
}
