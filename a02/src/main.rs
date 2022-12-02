use std::fs::File;
use std::io;
use std::io::BufRead;

fn main() -> Result<(), io::Error> {
    let file = File::open("input.txt")?;

    let mut total_score_part1 = 0u32;
    let mut total_score_part2 = 0u32;

    for line_r in io::BufReader::new(file).lines() {
        match line_r {
            Ok(line) => {
                let opponent = &line[0..1];
                let me = &line[2..];

                total_score_part1 += eval_round_part1(opponent, me);
                total_score_part2 += eval_round_part2(opponent, me);
            }
            Err(_) => break,
        }
    }

    println!("{:?}", total_score_part1);
    println!("{:?}", total_score_part2);

    Ok(())
}

/// X = rock, Y = paper, Z = scissors
fn eval_round_part1(opponent: &str, me: &str) -> u32 {
    if opponent.eq("A") {
        if me.eq("X") {
            1 + 3 // rock + draw
        } else if me.eq("Y") {
            2 + 6 // paper + win
        } else {
            3 // scissors + loss
        }
    } else if opponent.eq("B") {
        if me.eq("X") {
            1 // rock + loss
        } else if me.eq("Y") {
            2 + 3 // paper + draw
        } else {
            3 + 6 // scissors + win
        }
    } else if me.eq("X") {
        1 + 6 // rock + win
    } else if me.eq("Y") {
        2 // paper + loss
    } else {
        3 + 3 // scissors + draw
    }
}

/// X = lose, Y = draw, Z = win
fn eval_round_part2(opponent: &str, me: &str) -> u32 {
    if opponent.eq("A") {
        if me.eq("X") {
            3 // I lose with scissors
        } else if me.eq("Y") {
            1 + 3 // I draw with rock
        } else {
            2 + 6 // I win with paper
        }
    } else if opponent.eq("B") {
        if me.eq("X") {
            1 // I lose with rock
        } else if me.eq("Y") {
            2 + 3 // I draw with paper
        } else {
            3 + 6 // I win with scissors
        }
    } else if me.eq("X") {
        2 // I lose with paper
    } else if me.eq("Y") {
        3 + 3 // I draw with scissors
    } else {
        1 + 6 // I win with rock
    }
}
