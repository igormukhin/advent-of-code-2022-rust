use std::collections::VecDeque;
use regex::Regex;

#[derive(Debug)]
struct Config {
    blueprint_num: u32,
    ore_robot_cost_ore: u32,
    clay_robot_cost_ore: u32,
    obsidian_robot_cost_ore: u32,
    obsidian_robot_cost_clay: u32,
    geode_robot_cost_ore: u32,
    geode_robot_cost_obsidian: u32,
}

impl Config {
    fn max_ore_robots(&self) -> u32 {
        self.ore_robot_cost_ore.max(self.clay_robot_cost_ore)
            .max(self.obsidian_robot_cost_ore)
            .max(self.geode_robot_cost_ore)
    }

    fn max_clay_robots(&self) -> u32 {
        self.obsidian_robot_cost_clay
    }

    fn max_obsidian_robots(&self) -> u32 {
        self.geode_robot_cost_obsidian
    }
}

fn main() {
    let input = include_str!("../input.txt");

    let num_regex = Regex::new(r"\d+").unwrap();
    let configs = input.lines().map(|line| {
        let nums = num_regex.find_iter(line)
            .map(|m| m.as_str().parse::<u32>().unwrap())
            .collect::<Vec<u32>>();
        Config {
            blueprint_num: nums[0],
            ore_robot_cost_ore: nums[1],
            clay_robot_cost_ore: nums[2],
            obsidian_robot_cost_ore: nums[3],
            obsidian_robot_cost_clay: nums[4],
            geode_robot_cost_ore: nums[5],
            geode_robot_cost_obsidian: nums[6],
        }
    }).collect::<Vec<Config>>();

    let part_a = configs.iter()
        .map(|config| config.blueprint_num * max_geodes(config, 24))
        .sum::<u32>();
    println!("Part A: {}", part_a);

    let part_b = configs.iter()
        .take(3)
        .map(|config| max_geodes(config, 32))
        .product::<u32>();
    println!("Part B: {}", part_b);
}

struct State {
    time_elapsed: u32,
    ore: u32,
    clay: u32,
    obsidian: u32,
    geodes: u32,
    ore_robots: u32,
    clay_robots: u32,
    obsidian_robots: u32,
    geode_robots: u32,
}

impl State {
    fn initial() -> State {
        State {
            time_elapsed: 0,
            ore: 0,
            clay: 0,
            obsidian: 0,
            geodes: 0,
            ore_robots: 1,
            clay_robots: 0,
            obsidian_robots: 0,
            geode_robots: 0,
        }
    }
}

fn max_geodes(config: &Config, max_time: u32) -> u32 {
    let mut max: u32 = 0;

    println!("Starting graph walk of blueprint {}", config.blueprint_num);

    let mut states_queue = VecDeque::new();
    states_queue.push_back(State::initial());

    while let Some(state) = states_queue.pop_front() {
        let this_max_geodes = state.geodes + (state.geode_robots * (max_time - state.time_elapsed));
        if this_max_geodes > max {
            max = this_max_geodes;
        }

        if config.max_ore_robots() > state.ore_robots {
            let skip_moves = 1 + wait_moves(config.ore_robot_cost_ore, state.ore, state.ore_robots);
            if state.time_elapsed + skip_moves <= max_time {
                states_queue.push_back(State {
                    time_elapsed: state.time_elapsed + skip_moves,
                    ore: state.ore + (state.ore_robots * skip_moves) - config.ore_robot_cost_ore,
                    clay: state.clay + (state.clay_robots * skip_moves),
                    obsidian: state.obsidian + (state.obsidian_robots * skip_moves),
                    geodes: state.geodes + (state.geode_robots * skip_moves),
                    ore_robots: state.ore_robots + 1,
                    clay_robots: state.clay_robots,
                    obsidian_robots: state.obsidian_robots,
                    geode_robots: state.geode_robots,
                });
            }
        }

        if config.max_clay_robots() > state.clay_robots {
            let skip_moves = 1 + wait_moves(config.clay_robot_cost_ore, state.ore, state.ore_robots);
            if state.time_elapsed + skip_moves <= max_time {
                states_queue.push_back(State {
                    time_elapsed: state.time_elapsed + skip_moves,
                    ore: state.ore + (state.ore_robots * skip_moves) - config.clay_robot_cost_ore,
                    clay: state.clay + (state.clay_robots * skip_moves),
                    obsidian: state.obsidian + (state.obsidian_robots * skip_moves),
                    geodes: state.geodes + (state.geode_robots * skip_moves),
                    ore_robots: state.ore_robots,
                    clay_robots: state.clay_robots + 1,
                    obsidian_robots: state.obsidian_robots,
                    geode_robots: state.geode_robots,
                });
            }
        }

        if config.max_obsidian_robots() > state.obsidian_robots
            && state.clay_robots > 0 {
            let skip_moves = 1 +
                wait_moves(config.obsidian_robot_cost_ore, state.ore, state.ore_robots)
                    .max(wait_moves(config.obsidian_robot_cost_clay, state.clay, state.clay_robots));
            if state.time_elapsed + skip_moves <= max_time {
                states_queue.push_back(State {
                    time_elapsed: state.time_elapsed + skip_moves,
                    ore: state.ore + (state.ore_robots * skip_moves) - config.obsidian_robot_cost_ore,
                    clay: state.clay + (state.clay_robots * skip_moves) - config.obsidian_robot_cost_clay,
                    obsidian: state.obsidian + (state.obsidian_robots * skip_moves),
                    geodes: state.geodes + (state.geode_robots * skip_moves),
                    ore_robots: state.ore_robots,
                    clay_robots: state.clay_robots,
                    obsidian_robots: state.obsidian_robots + 1,
                    geode_robots: state.geode_robots,
                });
            }
        }

        if state.obsidian_robots > 0 {
            let skip_moves = 1 +
                wait_moves(config.geode_robot_cost_ore, state.ore, state.ore_robots)
                    .max(wait_moves(config.geode_robot_cost_obsidian, state.obsidian, state.obsidian_robots));
            if state.time_elapsed + skip_moves <= max_time {
                states_queue.push_back(State {
                    time_elapsed: state.time_elapsed + skip_moves,
                    ore: state.ore + (state.ore_robots * skip_moves) - config.geode_robot_cost_ore,
                    clay: state.clay + (state.clay_robots * skip_moves),
                    obsidian: state.obsidian + (state.obsidian_robots * skip_moves) - config.geode_robot_cost_obsidian,
                    geodes: state.geodes + (state.geode_robots * skip_moves),
                    ore_robots: state.ore_robots,
                    clay_robots: state.clay_robots,
                    obsidian_robots: state.obsidian_robots,
                    geode_robots: state.geode_robots + 1,
                });
            }
        }
    }

    println!("Max geodes: {}", max);
    max
}

fn wait_moves(cost: u32, stock: u32, robots: u32) -> u32 {
    return if stock >= cost {
        0
    } else {
        let missing_resource = cost - stock;
        missing_resource / robots + if missing_resource % robots > 0 { 1 } else { 0 }
    };
}

