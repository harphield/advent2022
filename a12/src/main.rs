use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::BufRead;

fn main() -> Result<(), io::Error> {
    let file = File::open("input.txt")?;

    let mut grid_width = 0;
    let mut grid_height = 0;

    let mut grid = vec![];
    let mut start_points = vec![];
    let mut end = 0;

    for line_r in io::BufReader::new(file).lines() {
        match line_r {
            Ok(line) => {
                grid_height += 1;

                if grid_width == 0 {
                    grid_width = line.len();
                }

                for c in line.chars() {
                    if c == 'S' {
                        start_points.push(grid.len());
                        grid.push('a' as u8 - 97);
                    } else if c == 'E' {
                        end = grid.len();
                        grid.push('z' as u8 - 97);
                    } else if c == 'a' {
                        start_points.push(grid.len());
                        grid.push(c as u8 - 97);
                    } else {
                        grid.push(c as u8 - 97);
                    }
                }
            }
            Err(_) => break,
        }
    }

    // println!("{:?}, {}, {}", grid, start, end);

    let mut paths = vec![];
    for start in start_points {
        let path = astar(start, end, &grid, grid_width, grid_height);
        // println!("{}", path.len() - 1);
        if !path.is_empty() {
            paths.push(path.len() - 1);
        }
    }

    paths.sort();

    println!("Part 2: {}", paths[0]);

    Ok(())
}

fn heur(index: &usize, end_index: usize, grid_width: usize, grid_height: usize) -> u64 {
    let x = index % grid_width;
    let y = (index - x) / grid_height;

    let end_x = end_index % grid_width;
    let end_y = (end_index - end_x) / grid_height;

    (end_x as i64 - x as i64 + end_y as i64 - y as i64).unsigned_abs()
}

fn reconstruct_path(came_from: &HashMap<usize, usize>, mut current: usize) -> Vec<usize> {
    let mut total_path = vec![current];
    loop {
        if came_from.contains_key(&current) {
            current = *came_from.get(&current).unwrap();
            total_path.push(current);
        } else {
            break;
        }
    }
    // while current in came_from.:
    //     current := cameFrom[current]
    //     total_path.prepend(current)

    total_path.into_iter().rev().collect()
}

fn get_possible_next_steps(index: &usize, grid_width: usize, grid_height: usize) -> Vec<usize> {
    let mut try_these = vec![];

    if *index == 0 {
        // top left
        try_these.push(index + 1);
        try_these.push(index + grid_width);
    } else if *index == grid_width - 1 {
        // top right
        try_these.push(index - 1);
        try_these.push(index + grid_width);
    } else if *index == (grid_width * grid_height) - grid_width {
        // bottom left
        try_these.push(index + 1);
        try_these.push(index - grid_width);
    } else if *index == (grid_width * grid_height) - 1 {
        // bottom right
        try_these.push(index - 1);
        try_these.push(index - grid_width);
    } else if *index < grid_width {
        // top edge
        try_these.push(index + 1);
        try_these.push(index - 1);
        try_these.push(index + grid_width);
    } else if (*index + 1) % grid_width == 0 {
        // right edge
        try_these.push(index - 1);
        try_these.push(index - grid_width);
        try_these.push(index + grid_width);
    } else if *index > (grid_width * grid_height) - grid_width {
        // bottom edge
        try_these.push(index + 1);
        try_these.push(index - 1);
        try_these.push(index - grid_width);
    } else if *index % grid_width == 0 {
        // left edge
        try_these.push(index + 1);
        try_these.push(index - grid_width);
        try_these.push(index + grid_width);
    } else {
        try_these.push(index + 1);
        try_these.push(index + grid_width);
        try_these.push(index - 1);
        try_these.push(index - grid_width);
    }

    // let next_steps: Vec<usize> = try_these.into_iter().filter(|v| !path.contains(v)).collect();

    // next_steps
    try_these
}

fn astar(
    start: usize,
    goal: usize,
    grid: &[u8],
    grid_width: usize,
    grid_height: usize,
) -> Vec<usize> {
    // The set of discovered nodes that may need to be (re-)expanded.
    // Initially, only the start node is known.
    // This is usually implemented as a min-heap or priority queue rather than a hash-set.
    let mut open_set = vec![start];

    // For node n, cameFrom[n] is the node immediately preceding it on the cheapest path from start
    // to n currently known.
    let mut came_from: HashMap<usize, usize> = HashMap::new();

    // For node n, gScore[n] is the cost of the cheapest path from start to n currently known.
    let mut g_score: HashMap<usize, u64> = HashMap::new();
    g_score.insert(start, 0);

    // For node n, fScore[n] := gScore[n] + h(n). fScore[n] represents our current best guess as to
    // how short a path from start to finish can be if it goes through n.
    // fScore := map with default value of Infinity
    let mut f_score: HashMap<usize, u64> = HashMap::new();
    f_score.insert(start, heur(&start, goal, grid_width, grid_height));

    // while open_set is not empty
    loop {
        if open_set.is_empty() {
            break;
        }

        // This operation can occur in O(1) time if openSet is a min-heap or a priority queue
        // let current = the node in openSet having the lowest fScore[] value
        let mut current = *open_set.first().unwrap();
        for opi in open_set.iter() {
            if match f_score.get(opi) {
                None => u64::MAX,
                Some(v) => *v,
            } < match f_score.get(&current) {
                None => u64::MAX,
                Some(v) => *v,
            } {
                current = *opi;
            }
        }

        if current == goal {
            return reconstruct_path(&came_from, current);
        }

        // println!("{:?} {}", open_set, current);
        open_set.remove(open_set.iter().position(|v| *v == current).unwrap());

        for neighbor in get_possible_next_steps(&current, grid_width, grid_height).iter() {
            // skip neighbors that are > 1 level higher
            if grid[current] < grid[*neighbor] && grid[*neighbor] - grid[current] > 1 {
                continue;
            }

            // for each neighbor of current
            // d(current,neighbor) is the weight of the edge from current to neighbor
            // tentative_gScore is the distance from start to the neighbor through current
            let tentative_g_score = match g_score.get(&current) {
                None => u64::MAX,
                Some(v) => *v, // } + grid[*neighbor] as u64;
            } + 1;

            if tentative_g_score
                < match g_score.get(neighbor) {
                    None => u64::MAX,
                    Some(v) => *v,
                }
            {
                // This path to neighbor is better than any previous one. Record it!
                // cameFrom[neighbor] := current
                came_from.insert(*neighbor, current);

                g_score.insert(*neighbor, tentative_g_score);

                f_score.insert(
                    *neighbor,
                    tentative_g_score + heur(neighbor, goal, grid_width, grid_height),
                );

                if !open_set.contains(neighbor) {
                    open_set.push(*neighbor);
                }
            }
        }
    }

    // Open set is empty but goal was never reached
    vec![]
}
