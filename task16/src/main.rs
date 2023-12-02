use std::cmp::Reverse;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

use regex::Regex;

#[derive(Debug, Clone)]
struct ParsedValve {
    id: String,
    flow: u32,
    tunnels_to: Vec<String>,
}

#[derive(Debug)]
struct Valve {
    bitmask: u64,
    flow: u32,
    tunnels_to: Vec<u8>,
}

#[derive(Debug, Clone)]
struct State {
    open_valves: u64,
    player1_at: u8,
    player2_at: u8,
    time_left: u8,
}

impl Hash for State {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.open_valves.hash(state);
        self.player1_at.max(self.player2_at).hash(state);
        self.player2_at.min(self.player1_at).hash(state);
        self.time_left.hash(state);
    }
}

impl PartialEq<Self> for State {
    fn eq(&self, other: &Self) -> bool {
        self.open_valves == other.open_valves
            && self.player1_at.max(self.player2_at) == other.player1_at.max(other.player2_at)
            && self.player2_at.min(self.player1_at) == other.player2_at.min(other.player1_at)
            && self.time_left == other.time_left
    }
}

impl Eq for State {}

fn index_of_valve(valves: &Vec<ParsedValve>, id: &str) -> u8 {
    valves.iter().position(|v| v.id == id).unwrap() as u8
}

struct Progress {
    best_score: u32,
    states_inserts: u32,
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    let re = Regex::new(
        "Valve (?P<valve>[A-Z]{2}) has flow rate=(?P<flow>\\d+); \
            tunnel(s)? lead(s)? to valve(s)? (?P<valves>([A-Z]{2}, )*[A-Z]{2})",
    ).unwrap();
    let parsed_valves = input.lines().map(|line| {
        let caps = re.captures(line).unwrap();
        let id = caps.name("valve").unwrap().as_str().to_string();
        let flow = caps.name("flow").unwrap().as_str().parse::<u32>().unwrap();
        let tunnels_to = caps.name("valves").unwrap().as_str().split(", ")
            .map(|s| s.to_string()).collect::<Vec<String>>();
        ParsedValve { id, flow, tunnels_to }
    })
        .collect::<Vec<ParsedValve>>();

    let mut sorted_valves = parsed_valves.clone();
    sorted_valves.sort_by_key(|v| Reverse(v.flow));

    let valves = sorted_valves.iter().enumerate().map(|(idx, v)| {
        let bitmask = 1 << idx;
        Valve {
            bitmask,
            flow: v.flow,
            tunnels_to: v.tunnels_to.iter()
                .map(|id| index_of_valve(&sorted_valves, id))
                .collect::<Vec<u8>>(),
        }
    }).collect::<Vec<Valve>>();

    part1(&sorted_valves, &valves);
    part2(&sorted_valves, &valves);

}

fn part1(parsed_valves: &Vec<ParsedValve>, valves: &Vec<Valve>) {
    let openable_valves_count = valves.iter().filter(|v| v.flow != 0).count() as u8;
    let mut states: HashMap<State, u32> = HashMap::new();
    let mut progress = Progress {
        best_score: 0,
        states_inserts: 0,
    };
    walk(&valves, &mut states, State {
        open_valves: 0,
        player1_at: index_of_valve(&parsed_valves, "AA"),
        player2_at: 100,
        time_left: 30,
    }, 0, 1, openable_valves_count, &mut progress);

    println!("Part 1 Max score: {}", progress.best_score);
}

fn part2(parsed_valves: &Vec<ParsedValve>, valves: &Vec<Valve>) {
    let openable_valves_count = valves.iter().filter(|v| v.flow != 0).count() as u8;
    let mut states: HashMap<State, u32> = HashMap::new();
    let mut progress = Progress {
        best_score: 0,
        states_inserts: 0,
    };
    walk(&valves, &mut states, State {
        open_valves: 0,
        player1_at: index_of_valve(&parsed_valves, "AA"),
        player2_at: index_of_valve(&parsed_valves, "AA"),
        time_left: 26,
    }, 0, 2, openable_valves_count, &mut progress);

    println!("Part 2 Max score: {}", progress.best_score);
}

fn walk(valves: &Vec<Valve>, states: &mut HashMap<State, u32>, new_state: State, new_score: u32,
        current_player: u8, openable_valves_count: u8,
        progress: &mut Progress) {
    if current_player == 1 {
        let old_score = states.get(&new_state);
        if old_score.is_some() && new_score <= *old_score.unwrap() {
            return;
        }

        states.insert(new_state.clone(), new_score);

        if new_score > progress.best_score {
            progress.best_score = new_score;
            println!("New best score: {}", progress.best_score);
        }

        progress.states_inserts += 1;
        if progress.states_inserts % 10_000_000 == 0 {
            println!("State inserts: {}, states: {}", progress.states_inserts, states.len());
        }

        if new_state.time_left == 0 {
            return;
        }

        // no need to walk around if all valves are open
        let open_valves_count = new_state.open_valves.count_ones() as u8;
        if open_valves_count == openable_valves_count {
            return;
        }

        // no need to continue on the path if the current best score can't be beaten
        if new_score + potential_score_left(valves, &new_state) <= progress.best_score {
            return;
        }
    }

    let next_score = new_score /*+ match current_player {
        2 => 0,
        1 => valves.iter()
                .filter(|v| new_state.open_valves & v.bitmask != 0)
                .map(|v| v.flow)
                .sum::<u32>(),
        _ => panic!("Invalid player"),
    }*/;
    let next_player = if current_player == 1 && new_state.player2_at != 100 { 2 } else { 1 };
    let next_time_left = new_state.time_left - if current_player == 1 { 1 } else { 0 };
    let player_at = if current_player == 1 { new_state.player1_at } else { new_state.player2_at };
    let valve = &valves[player_at as usize];

    // open valve action
    if valve.flow != 0
        && new_state.time_left > 1
        && new_state.open_valves & valve.bitmask == 0
    {
        let next_state = State {
            open_valves: new_state.open_valves | valve.bitmask,
            player1_at: new_state.player1_at,
            player2_at: new_state.player2_at,
            time_left: next_time_left,
        };
        let next_score = new_score + (valve.flow * (new_state.time_left - 1) as u32);
        walk(valves, states, next_state, next_score, next_player, openable_valves_count, progress);
    }

    // go tunnel action
    for tunnel_to in &valve.tunnels_to {
        let next_state = State {
            open_valves: new_state.open_valves,
            player1_at: if current_player == 1 { *tunnel_to } else { new_state.player1_at },
            player2_at: if current_player == 2 { *tunnel_to } else { new_state.player2_at },
            time_left: next_time_left,
        };
        walk(valves, states, next_state, next_score, next_player, openable_valves_count, progress);
    }
}

fn potential_score_left(valves: &Vec<Valve>, state: &State) -> u32 {
    let mut score = 0u32;
    let mut time_left = state.time_left;
    let mut move_num = 0;
    for valve in valves {
        if valve.flow == 0 {
            break;
        }
        if state.open_valves & valve.bitmask == 0 {
            score += valve.flow * (state.time_left - 1) as u32;
            move_num += 1;
            if move_num == 2 {
                if time_left < 3 {
                    break;
                }
                time_left -= 2;
                move_num = 0;
            }
        }
    }
    score
}
