use regex::Regex;
use std::collections::HashSet;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::ops::Sub;

const SEARCH_ROW: i64 = 2000000;
const MIN_COORD: i64 = 0;
const MAX_COORD: i64 = 20;

#[derive(Debug)]
struct Point {
    x: i64,
    y: i64,
}

fn main() -> Result<(), io::Error> {
    let file = File::open("input.txt")?;

    let rx =
        Regex::new(r"^Sensor at x=(.+), y=(.+): closest beacon is at x=(.+), y=(.+)$").unwrap();

    let mut beacons_on_row = HashSet::new();

    let mut cannot = HashSet::new();

    for line_r in io::BufReader::new(file).lines() {
        match line_r {
            Ok(line) => {
                let caps = rx.captures(&line).unwrap();

                let s_x = caps.get(1).unwrap().as_str().parse::<i64>().unwrap();
                let s_y = caps.get(2).unwrap().as_str().parse::<i64>().unwrap();
                let b_x = caps.get(3).unwrap().as_str().parse::<i64>().unwrap();
                let b_y = caps.get(4).unwrap().as_str().parse::<i64>().unwrap();

                if b_y == SEARCH_ROW {
                    beacons_on_row.insert(b_x);
                }

                let start = Point { x: s_x, y: s_y };

                let distance = manhattan(&start, &Point { x: b_x, y: b_y });

                match get_collision_points_with_row(&start, distance, SEARCH_ROW) {
                    None => {}
                    Some(points) => {
                        if points.len() == 2 {
                            for i in points[0].x..=points[1].x {
                                cannot.insert(i);
                            }
                        } else if points.len() == 1 {
                            cannot.insert(points[0].x);
                        }
                    }
                };
            }
            Err(_) => break,
        }
    }

    // let mut v:Vec<&i64> = cannot.iter().collect();
    // v.sort();
    //
    // println!("{:?}", v);

    println!("Part 1: {}", cannot.len() - beacons_on_row.len());

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

    let distance_to_row = manhattan(&start, &direct_up);

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
