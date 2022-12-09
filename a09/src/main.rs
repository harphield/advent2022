use regex::Regex;
use std::fs::File;
use std::io;
use std::io::BufRead;

#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
struct Position {
    x: i32,
    y: i32,
}

const KNOTS: usize = 9;

fn main() -> Result<(), io::Error> {
    let file = File::open("input.txt")?;

    // starting at 0,0
    let mut head_position = Position { x: 0, y: 0 };

    let mut tail_histories: Vec<Vec<Position>> = vec![];
    for _i in 0..KNOTS {
        tail_histories.push(vec![head_position.clone()]);
    }

    let regex_move = Regex::new(r"^(.+) (.+)$").unwrap();

    for line_r in io::BufReader::new(file).lines() {
        match line_r {
            Ok(line) => {
                let caps = regex_move.captures(&line).unwrap();
                let direction = caps.get(1).unwrap().as_str();
                let distance = caps.get(2).unwrap().as_str().parse::<u32>().unwrap();

                for _i in 0..distance {
                    head_position = update_head(&head_position, direction, 1);

                    let mut ref_position = head_position.clone();
                    for history in tail_histories.iter_mut().take(KNOTS) {
                        let knot_position = history.last().unwrap();
                        ref_position = match update_tail(&ref_position, knot_position) {
                            None => knot_position.clone(),
                            Some(new_position) => {
                                history.push(new_position.clone());
                                new_position
                            }
                        };
                    }
                }
            }
            Err(_) => break,
        }
    }

    let first_tail: &mut Vec<Position> = &mut tail_histories[0].to_owned();
    first_tail.sort();
    first_tail.dedup();

    println!("Part 1: {}", first_tail.len());

    let last_tail: &mut Vec<Position> = &mut tail_histories.last().unwrap().to_owned();

    last_tail.sort();
    last_tail.dedup();

    println!("Part 2: {}", last_tail.len());

    Ok(())
}

fn update_head(head_position: &Position, direction: &str, distance: u32) -> Position {
    let mut new_position = head_position.clone();

    match direction {
        "L" => {
            new_position.x -= distance as i32;
        }
        "R" => {
            new_position.x += distance as i32;
        }
        "U" => {
            new_position.y -= distance as i32;
        }
        "D" => {
            new_position.y += distance as i32;
        }
        &_ => panic!("oh no"),
    }

    new_position
}

fn update_tail(head_position: &Position, tail_position: &Position) -> Option<Position> {
    if head_position.eq(tail_position) {
        return None;
    }

    // distance is 1, so they are touching
    if (head_position.x - tail_position.x).abs() < 2
        && (head_position.y - tail_position.y).abs() < 2
    {
        return None;
    }

    let mut new_position = tail_position.clone();

    if head_position.y == tail_position.y {
        // same row
        if head_position.x > tail_position.x {
            new_position.x += 1;
        } else {
            new_position.x -= 1;
        }
    } else if head_position.x == tail_position.x {
        // same column
        if head_position.y > tail_position.y {
            new_position.y += 1;
        } else {
            new_position.y -= 1;
        }
    } else if head_position.x < tail_position.x {
        // head to the left
        new_position.x -= 1;
        if head_position.y > tail_position.y {
            new_position.y += 1;
        } else {
            new_position.y -= 1;
        }
    } else if head_position.x > tail_position.x {
        // head to the right
        new_position.x += 1;
        if head_position.y > tail_position.y {
            new_position.y += 1;
        } else {
            new_position.y -= 1;
        }
    }

    Some(new_position)
}

#[cfg(test)]
mod tests {
    use crate::{update_head, update_tail, Position};

    #[test]
    fn test_head_movement() {
        let mut head_position = Position { x: 0, y: 0 };

        head_position = update_head(&head_position, "L", 1);

        assert_eq!(head_position.x, -1);
        assert_eq!(head_position.y, 0);

        head_position = update_head(&head_position, "R", 1);

        assert_eq!(head_position.x, 0);
        assert_eq!(head_position.y, 0);

        head_position = update_head(&head_position, "U", 1);

        assert_eq!(head_position.x, 0);
        assert_eq!(head_position.y, -1);

        head_position = update_head(&head_position, "D", 1);

        assert_eq!(head_position.x, 0);
        assert_eq!(head_position.y, 0);
    }

    #[test]
    fn test_tail_movement_right() {
        let mut head_position = Position { x: 2, y: 0 };

        let mut tail_position = Position { x: 0, y: 0 };

        tail_position = match update_tail(&head_position, &tail_position) {
            None => tail_position,
            Some(new_position) => new_position,
        };

        assert_eq!(tail_position.x, 1);
        assert_eq!(tail_position.y, 0);
    }

    #[test]
    fn test_tail_movement_left() {
        let mut head_position = Position { x: -2, y: 0 };

        let mut tail_position = Position { x: 0, y: 0 };

        tail_position = match update_tail(&head_position, &tail_position) {
            None => tail_position,
            Some(new_position) => new_position,
        };

        assert_eq!(tail_position.x, -1);
        assert_eq!(tail_position.y, 0);
    }

    #[test]
    fn test_tail_movement_up() {
        let mut head_position = Position { x: 0, y: -2 };

        let mut tail_position = Position { x: 0, y: 0 };

        tail_position = match update_tail(&head_position, &tail_position) {
            None => tail_position,
            Some(new_position) => new_position,
        };

        assert_eq!(tail_position.x, 0);
        assert_eq!(tail_position.y, -1);
    }

    #[test]
    fn test_tail_movement_down() {
        let mut head_position = Position { x: 0, y: 2 };

        let mut tail_position = Position { x: 0, y: 0 };

        tail_position = match update_tail(&head_position, &tail_position) {
            None => tail_position,
            Some(new_position) => new_position,
        };

        assert_eq!(tail_position.x, 0);
        assert_eq!(tail_position.y, 1);
    }

    #[test]
    fn test_tail_movement_up_left() {
        let mut head_position = Position { x: -1, y: -2 };

        let mut tail_position = Position { x: 0, y: 0 };

        tail_position = match update_tail(&head_position, &tail_position) {
            None => tail_position,
            Some(new_position) => new_position,
        };

        assert_eq!(tail_position.x, -1);
        assert_eq!(tail_position.y, -1);
    }

    #[test]
    fn test_tail_movement_up_right() {
        let mut head_position = Position { x: 1, y: -2 };

        let mut tail_position = Position { x: 0, y: 0 };

        tail_position = match update_tail(&head_position, &tail_position) {
            None => tail_position,
            Some(new_position) => new_position,
        };

        assert_eq!(tail_position.x, 1);
        assert_eq!(tail_position.y, -1);
    }

    #[test]
    fn test_tail_movement_down_right() {
        let mut head_position = Position { x: 1, y: 2 };

        let mut tail_position = Position { x: 0, y: 0 };

        tail_position = match update_tail(&head_position, &tail_position) {
            None => tail_position,
            Some(new_position) => new_position,
        };

        assert_eq!(tail_position.x, 1);
        assert_eq!(tail_position.y, 1);
    }

    #[test]
    fn test_tail_movement_down_left() {
        let mut head_position = Position { x: -1, y: 2 };

        let mut tail_position = Position { x: 0, y: 0 };

        tail_position = match update_tail(&head_position, &tail_position) {
            None => tail_position,
            Some(new_position) => new_position,
        };

        assert_eq!(tail_position.x, -1);
        assert_eq!(tail_position.y, 1);
    }

    #[test]
    fn test_tail_movement_right_up() {
        let mut head_position = Position { x: 2, y: 1 };

        let mut tail_position = Position { x: 0, y: 0 };

        tail_position = match update_tail(&head_position, &tail_position) {
            None => tail_position,
            Some(new_position) => new_position,
        };

        assert_eq!(tail_position.x, 1);
        assert_eq!(tail_position.y, 1);
    }

    #[test]
    fn test_tail_movement_left_up() {
        let mut head_position = Position { x: -2, y: 1 };

        let mut tail_position = Position { x: 0, y: 0 };

        tail_position = match update_tail(&head_position, &tail_position) {
            None => tail_position,
            Some(new_position) => new_position,
        };

        assert_eq!(tail_position.x, -1);
        assert_eq!(tail_position.y, 1);
    }
}
