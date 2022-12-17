use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::BufRead;

#[derive(Debug, Clone)]
struct Valve {
    rate: u32,
    neighbors: Vec<(String, u32)>,
}

type ValveHolder = HashMap<String, Valve>;

const MINUTES: u32 = 30;

fn main() -> Result<(), io::Error> {
    let file = File::open("input.txt")?;

    let rx = Regex::new(r"^Valve (.+) has flow rate=(.+); tunnel[s]? lead[s]? to valve[s]? (.+)$")
        .unwrap();

    let mut valves: ValveHolder = HashMap::new();

    for line_r in io::BufReader::new(file).lines() {
        match line_r {
            Ok(line) => {
                let caps = rx.captures(&line).unwrap();

                let name = caps.get(1).unwrap().as_str();
                let rate = caps.get(2).unwrap().as_str().parse::<u32>().unwrap();
                let neighbors: Vec<(String, u32)> = caps
                    .get(3)
                    .unwrap()
                    .as_str()
                    .split(", ")
                    .map(|v| (v.to_string(), 0))
                    .collect();

                valves.insert(name.to_string(), Valve { rate, neighbors });
            }
            Err(_) => break,
        }
    }

    // reduce them to a weighted graph!
    let mut weighted_valves = ValveHolder::new();
    // find nodes that have rate > 0 + the starting node
    // find shortest paths between all of them
    let important: Vec<String> = valves
        .iter()
        .filter(|(s, v)| &s[..] == "AA" || v.rate > 0)
        .map(|(s, _v)| s.to_string())
        .collect();

    for s in &important {
        let mut s_valve = valves.get(s).unwrap().clone();

        for g in &important {
            if s == g {
                continue;
            }

            let path = astar(s, g, &valves);

            s_valve.neighbors.push((g.to_string(), path.len() as u32));

            weighted_valves.insert(s.to_string(), s_valve.clone());
        }
    }

    for (_s, v) in weighted_valves.iter_mut() {
        let updated: Vec<(String, u32)> = v
            .neighbors
            .iter()
            .filter(|(_s, w)| *w > 0u32)
            .map(|(s, w)| (s.to_string(), *w))
            .collect();

        v.neighbors = updated;
    }

    let max_p = permutations(
        weighted_valves.get(&"AA".to_string()).unwrap(),
        0,
        &vec![],
        &weighted_valves,
        important
            .iter()
            .filter(|v| v != &"AA")
            .count(),
    );

    println!("Part 1: {}", max_p);

    Ok(())
}

fn permutations(
    current: &Valve,
    minute: u32,
    opened: &Vec<&String>,
    valves: &ValveHolder,
    finish_on: usize,
) -> u32 {
    if opened.len() == finish_on {
        return order_to_pressure(opened, valves);
    }

    let mut values = vec![];

    for neighbor in &current.neighbors {
        if neighbor.0.eq(&"AA") || opened.contains(&&neighbor.0) {
            continue;
        }

        if minute + neighbor.1 > MINUTES {
            values.push(order_to_pressure(opened, valves));
            continue;
        }

        let n = valves.get(&neighbor.0).unwrap();

        let mut new_opened = opened.clone();
        new_opened.push(&neighbor.0);
        values.push(permutations(
            n,
            minute + neighbor.1,
            &new_opened,
            valves,
            finish_on,
        ));
    }

    if values.is_empty() {
        return 0;
    }

    *values.iter().max().unwrap()
}

fn order_to_pressure(order: &[&String], valves: &ValveHolder) -> u32 {
    let current = &"AA".to_string();
    let mut rate = 0;
    let mut pressure = 0;
    let mut minutes = 0;

    let mut c_v = valves.get(current).unwrap();

    for next in order {
        let n_v = valves.get(&next.to_string()).unwrap();

        let travel_time = *c_v
            .neighbors
            .iter()
            .filter(|(s, _w)| s == *next)
            .map(|(_s, w)| *w)
            .collect::<Vec<u32>>()
            .first()
            .unwrap();

        if minutes + travel_time > MINUTES {
            // stop here
            pressure += (minutes + travel_time - MINUTES) * rate;
            break;
        }

        minutes += travel_time;

        pressure += travel_time * rate;

        rate += n_v.rate;

        c_v = n_v;
    }

    // minutes left waiting
    pressure += rate * (MINUTES - minutes);

    pressure
}

fn heur() -> u32 {
    1
}

fn get_possible_next_steps(
    current: &String,
    previous: Option<&String>,
    valves: &ValveHolder,
) -> Vec<String> {
    let cv = valves.get(current).unwrap();

    cv.neighbors
        .iter()
        .filter(|(s, _v)| {
            match previous {
                None => {}
                Some(prev) => {
                    if prev == s {
                        return false;
                    }
                }
            }

            true
        })
        .map(|(s, _v)| s.clone())
        .collect()
}

fn reconstruct_path(came_from: &HashMap<String, String>, current: &str) -> Vec<String> {
    let mut total_path = vec![current.to_owned()];
    let mut c = current.to_owned();
    loop {
        if came_from.contains_key(&c) {
            c = came_from.get(&c).unwrap().clone();
            total_path.push(c.clone());
        } else {
            break;
        }
    }
    // while current in came_from.:
    //     current := cameFrom[current]
    //     total_path.prepend(current)

    total_path.into_iter().rev().collect()
}

fn astar(start: &str, goal: &String, valves: &ValveHolder) -> Vec<String> {
    // The set of discovered nodes that may need to be (re-)expanded.
    // Initially, only the start node is known.
    // This is usually implemented as a min-heap or priority queue rather than a hash-set.
    let mut open_set = vec![start.to_owned()];

    // For node n, cameFrom[n] is the node immediately preceding it on the cheapest path from start
    // to n currently known.
    let mut came_from: HashMap<String, String> = HashMap::new();

    // For node n, gScore[n] is the cost of the cheapest path from start to n currently known.
    let mut g_score: HashMap<String, u32> = HashMap::new();
    g_score.insert(start.to_owned(), 0);

    // For node n, fScore[n] := gScore[n] + h(n). fScore[n] represents our current best guess as to
    // how short a path from start to finish can be if it goes through n.
    // fScore := map with default value of Infinity
    let mut f_score: HashMap<String, u32> = HashMap::new();
    f_score.insert(start.to_owned(), heur());

    // while open_set is not empty
    loop {
        if open_set.is_empty() {
            break;
        }

        // This operation can occur in O(1) time if openSet is a min-heap or a priority queue
        // let current = the node in openSet having the lowest fScore[] value
        let mut current = open_set.first().unwrap().clone();
        for opi in &open_set {
            if match f_score.get(opi) {
                None => u32::MAX,
                Some(v) => *v,
            } < match f_score.get(&current) {
                None => u32::MAX,
                Some(v) => *v,
            } {
                current = opi.clone();
            }
        }

        if current == *goal {
            return reconstruct_path(&came_from, &current);
        }

        // println!("{:?} {}", open_set, current);
        open_set.remove(open_set.iter().position(|v| *v == current).unwrap());

        let next_steps = get_possible_next_steps(&current, came_from.get(&current), valves);
        for neighbor in &next_steps {
            // for each neighbor of current
            // d(current,neighbor) is the weight of the edge from current to neighbor
            // tentative_gScore is the distance from start to the neighbor through current
            let tentative_g_score = match g_score.get(&current) {
                None => u32::MAX,
                Some(v) => *v, // } + grid[*neighbor] as u64;
            } + 1;

            if tentative_g_score
                < match g_score.get(neighbor) {
                    None => u32::MAX,
                    Some(v) => *v,
                }
            {
                // This path to neighbor is better than any previous one. Record it!
                // cameFrom[neighbor] := current
                came_from.insert(neighbor.clone(), current.clone());

                g_score.insert(neighbor.clone(), tentative_g_score);

                f_score.insert(neighbor.clone(), tentative_g_score + heur());

                if !open_set.contains(neighbor) {
                    open_set.push(neighbor.clone());
                }
            }
        }
    }

    // Open set is empty but goal was never reached
    vec![]
}

#[cfg(test)]
mod tests {
    use crate::{Valve, ValveHolder};
}
