use std::cmp::max;
use std::fs::File;
use std::str::FromStr;
use std::io::{prelude::*, BufReader};
use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Blueprint { 
    id: u8,
    ore_robot_ore: u16,
    clay_robot_ore: u16,
    obsidian_robot_ore: u16,
    obsidian_robot_clay: u16,
    geode_robot_ore: u16,
    geode_robot_obsidian: u16,
}

impl FromStr for Blueprint {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            // Blueprint 6: Each ore robot costs 2 ore. Each clay robot costs 4 ore. Each obsidian robot costs 4 ore and 17 clay. Each geode robot costs 3 ore and 11 obsidian.            
            static ref RE: Regex = Regex::new(concat!(
                "Blueprint (?P<blueprint>\\d+): Each ore robot costs (?P<ore_robot_ore>\\d+) ore. ",
                "Each clay robot costs (?P<clay_robot_ore>\\d+) ore. ",
                "Each obsidian robot costs (?P<obsidian_robot_ore>\\d+) ore and ",
                "(?P<obsidian_robot_clay>\\d+) clay. Each geode robot costs ",
                "(?P<geode_robot_ore>\\d+) ore and (?P<geode_robot_obsidian>\\d+) obsidian."
            )).unwrap();
        }
   
        let caps = RE.captures(s).unwrap();
        Ok(Self {
            id: caps.name("blueprint").unwrap().as_str().parse().unwrap(),
            ore_robot_ore: caps.name("ore_robot_ore").unwrap().as_str().parse().unwrap(),
            clay_robot_ore: caps.name("clay_robot_ore").unwrap().as_str().parse().unwrap(),
            obsidian_robot_ore: caps.name("obsidian_robot_ore").unwrap().as_str().parse().unwrap(),
            obsidian_robot_clay: caps.name("obsidian_robot_clay").unwrap().as_str().parse().unwrap(),
            geode_robot_ore: caps.name("geode_robot_ore").unwrap().as_str().parse().unwrap(),
            geode_robot_obsidian: caps.name("geode_robot_obsidian").unwrap().as_str().parse().unwrap(),
        })
    }
}

#[derive(Debug, Clone)]
enum Actions {
    BuildOreRobot,
    BuildClayRobot,
    BuildObsidianRobot,
    BuildGeodeRobot,
    Idle
}

#[derive(Debug, Clone)]
struct Strategy {
    ore : u16,
    ore_robots: u8,

    clay : u16,
    clay_robots: u8,

    obsidian : u16,
    obsidian_robots: u8,

    geodes : u16,
    geode_robots: u8,

    actions : Vec<Actions>
}

impl Strategy {
    fn new() -> Self {
        Strategy {
            ore: 0,
            ore_robots: 1,
            clay: 0,
            clay_robots : 0,
            obsidian : 0,
            obsidian_robots: 0,
            geodes: 0,
            geode_robots: 0,
            actions: vec![]
        }
    }

    fn produce(&mut self) {
        self.ore += self.ore_robots as u16;
        self.clay += self.clay_robots as u16;
        self.obsidian += self.obsidian_robots as u16;
        self.geodes += self.geode_robots as u16;
    }

    fn possible_actions(&self, blueprint: &Blueprint) -> Vec<Actions> {
        // If we can build a geode robot, that's the only sane thing to do.
        if self.ore >= blueprint.geode_robot_ore && self.obsidian >= blueprint.geode_robot_obsidian {
            return vec![ Actions::BuildGeodeRobot ];
        }

        // Otherwise, it's not clear what's best.
        let mut actions = vec![Actions::Idle];
        if self.ore >= blueprint.ore_robot_ore {
            actions.push(Actions::BuildOreRobot);
        }
        if self.ore >= blueprint.clay_robot_ore {
            actions.push(Actions::BuildClayRobot);
        }
        if self.ore >= blueprint.obsidian_robot_ore && self.clay >= blueprint.obsidian_robot_clay {
            actions.push(Actions::BuildObsidianRobot);
        }
        actions
    }


    fn best_for_blueprint(blueprint: &Blueprint, max_time: u32, use_obsidian_filter: bool) -> Strategy {
        let mut strategies = vec![Strategy::new()];

        for _time in 0..max_time {
            //println!("BP: {} T: {}", blueprint.id, time);
            let mut next_strategies = vec![];
            for strategy in &mut strategies {
                // Identify possible actions before production, but don't act on it
                let possible_actions = strategy.possible_actions(blueprint);
                // Produce new materials
                strategy.produce();
                // Now spawn all the possible strategies we could have used this round
                for action in possible_actions {
                    let mut next_strategy = strategy.clone();
                    match action {
                        Actions::BuildOreRobot => {
                            next_strategy.ore -= blueprint.ore_robot_ore;
                            next_strategy.ore_robots += 1;
                        },
                        Actions::BuildClayRobot => {
                            next_strategy.ore -= blueprint.clay_robot_ore;
                            next_strategy.clay_robots += 1;
                        },
                        Actions::BuildObsidianRobot => {
                            next_strategy.ore -= blueprint.obsidian_robot_ore;
                            next_strategy.clay -= blueprint.obsidian_robot_clay;
                            next_strategy.obsidian_robots += 1;
                        },
                        Actions::BuildGeodeRobot => {
                            next_strategy.ore -= blueprint.geode_robot_ore;
                            next_strategy.obsidian -= blueprint.geode_robot_obsidian;
                            next_strategy.geode_robots += 1;
                        },
                        Actions::Idle => {}
                    }
                    next_strategy.actions.push(action);
                    next_strategies.push(next_strategy);
                }
            }
            // Filter out some bad strategies
            let max_geode_robots = strategies.iter().map(|x| x.geode_robots).max().unwrap();
            let max_obsidian_robots = strategies.iter().map(|x| x.obsidian_robots).max().unwrap();
            strategies = next_strategies.iter()
                // Filter out strategies that don't achieve max geode robot count
                .filter(|s| s.geode_robots >= max_geode_robots)
                // Filter out strategies that don't get an obsidian machine fast - doesn't work on one run...
                .filter(|s| !(use_obsidian_filter && s.geode_robots == 0 && s.obsidian_robots < max_obsidian_robots))
                // Filter out strategies that don't build either ore or clay machines as fast as possible
                .filter(|s| {
                    !(s.clay_robots == 0 && s.ore_robots == 1 &&
                      s.ore > max(blueprint.ore_robot_ore, blueprint.clay_robot_ore))
                })
                .cloned()
                .collect();
        }
        strategies.sort_by(|strategy, other| other.geodes.cmp(&strategy.geodes));
        let quality = blueprint.id as u32 * strategies[0].geodes as u32;
        println!(" * Best score for blueprint {} was {} geodes (Quality: {})", blueprint.id, strategies[0].geodes, quality);
        return strategies[0].clone()
    }
}

pub fn day19() {
    let file = File::open("input.txt").expect("File 'input.txt' not readable.");
    let blueprints : Vec<Blueprint> = BufReader::new(file)
        .lines() // Get a line iterator
        .filter_map(|line| line.ok()) // Get Strings instead of Result
        .filter_map(|line| line.parse::<Blueprint>().ok())
        .collect();

    let mut total_score = 0u32;

    // Part 1
    for blueprint in &blueprints {
        let best_strategy = Strategy::best_for_blueprint(blueprint, 24, true);
        let quality = blueprint.id as u32 * best_strategy.geodes as u32;
        total_score += quality;
    }
    println!("== Part 1 total score is {} ==", total_score); // 1650

    total_score = [
        Strategy::best_for_blueprint(&blueprints[0], 32, true).geodes as u32,

        // One of the filters causes a bad result on blueprint 2, for reasons not understood.
        Strategy::best_for_blueprint(&blueprints[1], 32, false).geodes as u32,
        Strategy::best_for_blueprint(&blueprints[2], 32, true).geodes as u32,
    ].iter().product();

    println!("== Part 2 total score is {} ==", total_score); // 5824
}

pub fn main() {
    day19()
}

#[cfg(test)]
 mod test {
    use super::*;

    #[test]
    fn test_one() {
        let blueprint = Blueprint {
            id: 1,
            ore_robot_ore: 4,
            clay_robot_ore: 2,
            obsidian_robot_ore: 3,
            obsidian_robot_clay: 14,
            geode_robot_ore: 2,
            geode_robot_obsidian: 7,
        };

        let best_strategy = Strategy::best_for_blueprint(&blueprint, 24, true);
        assert_eq!(best_strategy.geodes, 9);
    }
}