mod inputs;

const WIDTH: usize = 7;
const START_OFFSET_X: u32 = 2;
const START_OFFSET_Y: u32 = 3;

struct Rock {
    width: u32,
    height: u32,
    pixels: Vec<bool>
}

fn main() {
    let rocks = [
        // -
        Rock {
            width: 4,
            height: 1,
            pixels: vec![
                true, true, true, true,
            ],
        },
        // +
        Rock {
            width: 3,
            height: 3,
            pixels: vec![
                false, true, false,
                true, true, true,
                false, true, false,
            ],
        },
        // _|
        Rock {
            width: 3,
            height: 3,
            pixels: vec![
                true, true, true,
                false, false, true,
                false, false, true,
            ],
        },
        // |
        Rock {
            width: 1,
            height: 4,
            pixels: vec![
                true, true, true, true,
            ],
        },
        // []
        Rock {
            width: 2,
            height: 2,
            pixels: vec![
                true, true, true, true,
            ],
        },
    ];

    let mut rows: Vec<[bool; WIDTH]> = vec![];
    let mut fallen_rocks = 0;
    let mut rock_type = 0;
    let mut jet_shot = 0;
    let jet_shot_count = inputs::INPUT.len();
    let mut current_rock = &rocks[rock_type];
    let mut rock_position = (START_OFFSET_X, START_OFFSET_Y);

    add_empty_rows(START_OFFSET_Y + current_rock.height, &mut rows);

    loop {
        // render(&rock_position, &current_rock, &rows);

        // blowing
        if inputs::INPUT[jet_shot] {
            // right
            if !collision(&(rock_position.0 + 1, rock_position.1), &current_rock, &rows) {
                rock_position.0 += 1;
            }
        } else {
            // left
            if rock_position.0 > 0 && !collision(&(rock_position.0 - 1, rock_position.1), &current_rock, &rows) {
                rock_position.0 -= 1;
            }
        }

        // falling
        if rock_position.1 > 0 && !collision(&(rock_position.0, rock_position.1 - 1), &current_rock, &rows) {
            rock_position.1 -= 1;
        } else {
            // stopped!
            fallen_rocks += 1;

            if fallen_rocks == 2023 {
                break;
            }

            add_rock_to_rows(&rock_position, &current_rock, &mut rows);

            rock_type += 1;
            if rock_type > 4 {
                rock_type = 0;
            }

            current_rock = &rocks[rock_type];
            rock_position = (START_OFFSET_X, get_highest_pixel(&rows) + START_OFFSET_Y + 1);

            add_empty_rows(current_rock.height, &mut rows);
        }

        jet_shot += 1;
        if jet_shot == jet_shot_count {
            jet_shot = 0;
        }
    }

    println!("Part 1: {}", get_highest_pixel(&rows) + 1);

}

fn render(rock_coords: &(u32, u32), rock: &Rock, rows: &Vec<[bool; WIDTH]>) {
    let mut rock_pixel_coords = vec![];
    for (i, p) in rock.pixels.iter().enumerate() {
        if !p {
            continue;
        }

        let x = i % rock.width as usize;
        let y = (i as f32 / rock.width as f32).floor() as usize;

        rock_pixel_coords.push((rock_coords.0 as usize + x, rock_coords.1 as usize + y));
    }

    let mut r_y = rows.len();
    for r in rows.iter().rev() {
        r_y -= 1;

        print!("|");

        for (x, p) in r.iter().enumerate() {
            if rock_pixel_coords.contains(&(x, r_y)) {
                print!("@");
            } else if *p {
                print!("#");
            } else {
                print!(".");
            }
        }

        print!("|");
        println!();
    }

    println!();
}

fn add_empty_rows(count: u32, rows: &mut Vec<[bool; WIDTH]>) {
    for _i in 0..count {
        rows.push([false; WIDTH]);
    }
}

fn add_rock_to_rows(coords: &(u32, u32), rock: &Rock, rows: &mut Vec<[bool; WIDTH]>) {
    for (i, p) in rock.pixels.iter().enumerate() {
        if !p {
            continue;
        }

        let x = i % rock.width as usize;
        let y = (i as f32 / rock.width as f32).floor() as usize;

        rows[coords.1 as usize + y][coords.0 as usize + x] = true;
    }
}

fn collision(coords: &(u32, u32), rock: &Rock, rows: &Vec<[bool; WIDTH]>) -> bool {
    for (i, p) in rock.pixels.iter().enumerate() {
        if !p {
            continue;
        }

        let x = i % rock.width as usize;
        let y = (i as f32 / rock.width as f32).floor() as usize;

        match rows.get(coords.1 as usize + y) {
            None => return false,
            Some(r) => {
                if coords.0 as usize + x >= WIDTH || r[coords.0 as usize + x] {
                    return true;
                }
            }
        }
    }
    false
}

fn get_highest_pixel(rows: &Vec<[bool; WIDTH]>) -> u32 {
    let mut height = rows.len();
    for r in rows.iter().rev() {
        height -= 1;
        if r.contains(&true) {
            return height as u32;
        }
    }

    0
}

