use regex::Regex;
use std::collections::HashSet;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::ops::Sub;

const MIN_COORD: i64 = 0;
const MAX_COORD: i64 = 4000000;

#[derive(Debug)]
struct Point {
    x: i64,
    y: i64,
}

fn main() -> Result<(), io::Error> {
    let file = File::open("input.txt")?;

    let rx =
        Regex::new(r"^Sensor at x=(.+), y=(.+): closest beacon is at x=(.+), y=(.+)$").unwrap();

    let mut potential: Vec<HashSet<(i64, i64)>> = vec![HashSet::new(); (MAX_COORD + 1) as usize];

    for line_r in io::BufReader::new(file).lines() {
        match line_r {
            Ok(line) => {
                let caps = rx.captures(&line).unwrap();

                let s_x = caps.get(1).unwrap().as_str().parse::<i64>().unwrap();
                let s_y = caps.get(2).unwrap().as_str().parse::<i64>().unwrap();
                let b_x = caps.get(3).unwrap().as_str().parse::<i64>().unwrap();
                let b_y = caps.get(4).unwrap().as_str().parse::<i64>().unwrap();

                let start = Point { x: s_x, y: s_y };
                let distance = manhattan(&start, &Point { x: b_x, y: b_y });

                let min_x = *[s_x - distance, MIN_COORD].iter().max().unwrap(); // 0 or more
                let max_x = *[s_x + distance, MAX_COORD].iter().min().unwrap(); // 20 or less
                let min_y = *[s_y - distance, MIN_COORD].iter().max().unwrap();
                let max_y = *[s_y + distance, MAX_COORD].iter().min().unwrap();

                for y in min_y..=max_y {
                    match get_collision_points_with_row(&start, distance, y) {
                        None => {}
                        Some(points) => {
                            if points.len() == 2 {
                                if points[0].x <= max_x && points[1].x >= min_x {
                                    let limits = (
                                        if points[0].x >= MIN_COORD {
                                            points[0].x
                                        } else {
                                            MIN_COORD
                                        },
                                        if points[1].x <= MAX_COORD {
                                            points[1].x
                                        } else {
                                            MAX_COORD
                                        },
                                    );

                                    potential[y as usize].insert(limits);
                                }
                            } else if points.len() == 1
                                && points[0].x >= MIN_COORD
                                && points[0].x <= MAX_COORD
                            {
                                potential[y as usize].insert((points[0].x, points[0].x));
                            }
                        }
                    };
                }
            }
            Err(_) => break,
        }
    }

    for (y, row) in potential.iter().enumerate() {
        if row.is_empty() {
            continue;
        }

        if row.len() == 1 {
            // hmmm
            continue;
        }

        let mut row_vec = row.iter().collect::<Vec<&(i64, i64)>>();
        row_vec.sort();

        let mut bubbles = vec![];

        let mut current_bubble = (row_vec[0].0, row_vec[0].1);
        for (x1, x2) in row_vec {
            // gap?
            if *x1 > current_bubble.1 + 1 || *x2 < current_bubble.0 - 1 {
                bubbles.push(current_bubble);
                current_bubble = (*x1, *x2);
                continue;
            }

            if *x1 < current_bubble.0 {
                current_bubble.0 = *x1;
            }

            if *x2 > current_bubble.1 {
                current_bubble.1 = *x2;
            }
        }

        if bubbles.is_empty() {
            continue;
        }

        bubbles.push(current_bubble);

        for window in bubbles.windows(2) {
            for x in (window[0].1 + 1)..window[1].0 {
                println!("{}", x * 4000000 + y as i64);
            }
        }
    }

    Ok(())
}

fn abs_difference<T>(x: T, y: T) -> T
where
    T: Sub<Output = T> + Ord,
{
    if x < y {
        y - x
    } else {
        x - y
    }
}

fn manhattan(a: &Point, b: &Point) -> i64 {
    abs_difference(b.x, a.x) + abs_difference(b.y, a.y)
}

fn get_collision_points_with_row(start: &Point, distance: i64, y: i64) -> Option<Vec<Point>> {
    let direct_up = Point { x: start.x, y };

    let distance_to_row = manhattan(start, &direct_up);

    if distance < distance_to_row {
        return None;
    }

    if distance == distance_to_row {
        return Some(vec![direct_up]);
    }

    if distance > distance_to_row {
        // 2 points, pythagoras to the rescue
        let x = (distance - distance_to_row).abs();

        return Some(vec![
            Point { x: start.x - x, y },
            Point { x: start.x + x, y },
        ]);
    }

    None
}

#[cfg(test)]
mod tests {
    use crate::{get_collision_points_with_row, manhattan, Point};

    #[test]
    fn reverse_manhattan() {
        let a = Point { x: 8, y: 7 };
        let b = Point { x: 2, y: 10 };

        let distance = manhattan(&a, &b);

        let points = get_collision_points_with_row(&a, distance, 10).unwrap();

        assert!(points[0].x == 2 && points[1].x == 14);
    }

    #[test]
    fn reverse_manhattan_2() {
        let a = Point { x: 0, y: 11 };
        let b = Point { x: 2, y: 10 };

        let distance = manhattan(&a, &b);

        let points = get_collision_points_with_row(&a, distance, 10).unwrap();

        assert!(points[0].x == -2 && points[1].x == 2);
    }
}
