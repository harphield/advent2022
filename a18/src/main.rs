use std::collections::HashMap;

mod inputs;

const WALL_X_MINUS_1: usize = 0;
const WALL_X_PLUS_1: usize = 1;
const WALL_Y_MINUS_1: usize = 2;
const WALL_Y_PLUS_1: usize = 3;
const WALL_Z_MINUS_1: usize = 4;
const WALL_Z_PLUS_1: usize = 5;

#[derive(Debug)]
struct Cube {
    x: u8,
    y: u8,
    z: u8,
    walls: [bool; 6],
}

fn main() {
    let mut free_walls = 0;
    let mut cubes: HashMap<[u8; 3], Cube> = HashMap::new();
    for c in inputs::INPUT_FULL {
        let mut walls = [true; 6];
        free_walls += 6;

        // find neighbors
        // x - 1
        if c[0] > 0 {
            match cubes.get_mut(&[c[0] - 1, c[1], c[2]]) {
                None => {}
                Some(n) => {
                    walls[WALL_X_MINUS_1] = false;
                    n.walls[WALL_X_PLUS_1] = false;

                    free_walls -= 2;
                }
            }
        }
        // x + 1
        match cubes.get_mut(&[c[0] + 1, c[1], c[2]]) {
            None => {}
            Some(n) => {
                walls[WALL_X_PLUS_1] = false;
                n.walls[WALL_X_MINUS_1] = false;

                free_walls -= 2;
            }
        }
        // y - 1
        if c[1] > 0 {
            match cubes.get_mut(&[c[0], c[1] - 1, c[2]]) {
                None => {}
                Some(n) => {
                    walls[WALL_Y_MINUS_1] = false;
                    n.walls[WALL_Y_PLUS_1] = false;

                    free_walls -= 2;
                }
            }
        }
        // y + 1
        match cubes.get_mut(&[c[0], c[1] + 1, c[2]]) {
            None => {}
            Some(n) => {
                walls[WALL_Y_PLUS_1] = false;
                n.walls[WALL_Y_MINUS_1] = false;

                free_walls -= 2;
            }
        }
        // z - 1
        if c[2] > 0 {
            match cubes.get_mut(&[c[0], c[1], c[2] - 1]) {
                None => {}
                Some(n) => {
                    walls[WALL_Z_MINUS_1] = false;
                    n.walls[WALL_Z_PLUS_1] = false;

                    free_walls -= 2;
                }
            }
        }
        // z + 1
        match cubes.get_mut(&[c[0], c[1], c[2] + 1]) {
            None => {}
            Some(n) => {
                walls[WALL_Z_PLUS_1] = false;
                n.walls[WALL_Z_MINUS_1] = false;

                free_walls -= 2;
            }
        }

        cubes.insert(
            c,
            Cube {
                x: c[0],
                y: c[1],
                z: c[2],
                walls, // all walls are free
            },
        );
    }

    // println!("{:?}", cubes);

    println!("Part 1: {}", free_walls);
}
