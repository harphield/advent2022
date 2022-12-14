use std::fs::File;
use std::io;
use std::io::BufRead;

fn main() -> Result<(), io::Error> {
    let file = File::open("input.txt")?;

    let starting_point = (500, 0);
    let mut lowest_point = 0;
    let mut walls = vec![];
    let mut grains = vec![];

    for line_r in io::BufReader::new(file).lines() {
        match line_r {
            Ok(line) => {
                let mut wall = vec![];
                for xy_str in line.split(" -> ") {
                    let xy: Vec<&str> = xy_str.split(',').collect();

                    let x = xy[0].parse::<i32>().unwrap();
                    let y = xy[1].parse::<i32>().unwrap();

                    if y > lowest_point {
                        lowest_point = y;
                    }

                    wall.push((x, y));
                }

                walls.push(wall);
            }
            Err(_) => break,
        }
    }

    let floor = 2 + lowest_point;

    let mut count = 1;
    let mut current = starting_point;
    loop {
        // if current.1 > lowest_point {
        //     break;
        // }

        if check_collision(&current, &vec![], &grains) {
            break;
        }

        if current.1 + 1 == floor {
            count += 1;
            grains.push(current);
            current = starting_point;
        } else if !check_collision(&(current.0, current.1 + 1), &walls, &grains) {
            // down
            current.1 += 1;
        } else if !check_collision(&(current.0 - 1, current.1 + 1), &walls, &grains) {
            // down left
            current.0 -= 1;
            current.1 += 1;
        } else if !check_collision(&(current.0 + 1, current.1 + 1), &walls, &grains) {
            // down right
            current.0 += 1;
            current.1 += 1;
        } else {
            // stopped
            count += 1;
            grains.push(current);
            current = starting_point;
        }
    }

    println!("Part 2: {}", count - 1); // the last one fell down

    Ok(())
}

fn check_collision(
    point: &(i32, i32),
    walls: &Vec<Vec<(i32, i32)>>,
    grains: &[(i32, i32)],
) -> bool {
    if grains.contains(point) {
        return true;
    }

    for wall in walls {
        for part in wall.windows(2) {
            let start = part[0];
            let end = part[1];

            if start.0 == end.0 && point.0 == start.0 {
                // vertical line
                if start.1 < end.1 {
                    if point.1 >= start.1 && point.1 <= end.1 {
                        return true;
                    }
                } else if point.1 <= start.1 && point.1 >= end.1 {
                    return true;
                }
            } else if start.1 == end.1 && point.1 == start.1 {
                // horizontal line
                if start.0 < end.0 {
                    if point.0 >= start.0 && point.0 <= end.0 {
                        return true;
                    }
                } else if point.0 <= start.0 && point.0 >= end.0 {
                    return true;
                }
            }
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use crate::check_collision;

    #[test]
    fn collisions() {
        let walls = vec![
            vec![(498, 4), (498, 6), (496, 6)],
            vec![(503, 4), (502, 4), (502, 9), (494, 9)],
        ];

        let grains = vec![];

        assert!(!check_collision(&(500, 0), &walls, &grains));
        assert!(check_collision(&(498, 4), &walls, &grains));
        assert!(check_collision(&(498, 5), &walls, &grains));
        assert!(check_collision(&(500, 9), &walls, &grains));
    }
}
