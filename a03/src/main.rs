use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io;
use std::io::BufRead;

fn main() -> Result<(), io::Error> {
    let file = File::open("input.txt")?;

    let mut sum = 0;
    let mut i = 0;
    let mut group = vec![];
    let mut sum_2 = 0;

    for line_r in io::BufReader::new(file).lines() {
        match line_r {
            Ok(line) => {
                if i == 3 {
                    i = 0;

                    // check the 3 rucksacks
                    sum_2 += get_priority(intersect(group)[0]);

                    group = vec![];
                }

                // part 1
                sum += parse_line_1(&line);
                group.push(parse_line_2(&line));

                i += 1;
            }
            Err(_) => break,
        }
    }

    // check the 3 rucksacks
    sum_2 += get_priority(intersect(group)[0]);

    ////////
    println!("PART 1 = {}", sum);

    println!("PART 2 = {}", sum_2);

    Ok(())
}

fn parse_line_1(line: &str) -> u32 {
    let len = line.len();
    let len_half = len / 2;

    let mut compartment_1 = HashMap::new();
    let mut compartment_2 = HashMap::new();

    let mut sum_vec = vec![];

    for (i, c) in line.chars().enumerate() {
        if i < len_half {
            compartment_1.insert(
                c,
                match compartment_1.get(&c) {
                    Some(value) => value + 1u32,
                    None => 1,
                },
            );

            match compartment_2.get(&c) {
                None => {}
                Some(_) => {
                    let prio = get_priority(c);
                    if !sum_vec.contains(&prio) {
                        sum_vec.push(prio);
                    }
                }
            }
        } else {
            compartment_2.insert(
                c,
                match compartment_2.get(&c) {
                    Some(value) => value + 1u32,
                    None => 1,
                },
            );

            match compartment_1.get(&c) {
                None => {}
                Some(_) => {
                    let prio = get_priority(c);
                    if !sum_vec.contains(&prio) {
                        sum_vec.push(prio);
                    }
                }
            }
        }
    }

    sum_vec.iter().sum()
}

fn parse_line_2(line: &str) -> Vec<char> {
    // deduplication
    line.chars()
        .into_iter()
        .collect::<HashSet<_>>()
        .into_iter()
        .collect()
}

fn intersect(group: Vec<Vec<char>>) -> Vec<char> {
    let mut intersect_result: Vec<char> = group[0].clone();

    for temp_vec in group {
        let unique_a: HashSet<char> = temp_vec.into_iter().collect();
        intersect_result = unique_a
            .intersection(&intersect_result.into_iter().collect())
            .copied()
            .collect::<Vec<_>>();
    }
    intersect_result
}

fn get_priority(c: char) -> u32 {
    let mut num = c as u32;

    if c.is_lowercase() {
        num -= 96;
    } else {
        num -= 38;
    }

    num
}
