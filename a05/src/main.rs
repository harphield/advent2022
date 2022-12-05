use regex::Regex;
use std::fs::File;
use std::io;
use std::io::BufRead;

fn main() -> Result<(), io::Error> {
    let file = File::open("input.txt")?;

    let mut diagram = true;
    let mut diagram_columns: Vec<Vec<char>> = vec![];
    let mut diagram_columns_2: Vec<Vec<char>> = vec![];

    let box_regex = Regex::new(r"\[(.+)]").unwrap();
    let instructions_regex = Regex::new(r"move (.+) from (.+) to (.+)").unwrap();

    for line_r in io::BufReader::new(file).lines() {
        match line_r {
            Ok(line) => {
                if line.eq("") {
                    // end of the crate diagram, start of move instructions
                    diagram = false;

                    diagram_columns_2 = diagram_columns.clone();
                } else if diagram {
                    // crate diagram
                    let len = line.len();
                    let mut start = 0;
                    let mut end = 3;
                    let mut column = 0usize;
                    loop {
                        if start >= len || end > len {
                            break;
                        }

                        match diagram_columns.get(column) {
                            None => {
                                diagram_columns.push(vec![]);
                            }
                            Some(_c) => {}
                        }

                        let part = &line[start..end];

                        if !part.eq("   ") {
                            match &box_regex.captures(part) {
                                Some(m) => {
                                    diagram_columns[column].insert(0, m[1].chars().last().unwrap());
                                }
                                None => {}
                            };
                        }

                        start += 4;
                        end += 4;
                        column += 1;
                    }
                } else {
                    // move instructions
                    let c = &instructions_regex.captures(&line).unwrap();

                    let how_many = c.get(1).unwrap().as_str().parse::<usize>().unwrap();
                    let from = c.get(2).unwrap().as_str().parse::<usize>().unwrap() - 1;
                    let to = c.get(3).unwrap().as_str().parse::<usize>().unwrap() - 1;

                    // part 2 solution
                    let mut slc = diagram_columns_2[from]
                        [(diagram_columns_2[from].len() - how_many)..]
                        .to_vec();
                    diagram_columns_2[to].append(&mut slc);

                    // part 1 solution
                    for _i in 0..how_many {
                        let c = diagram_columns[from].pop().unwrap();
                        diagram_columns[to].push(c);

                        diagram_columns_2[from].pop();
                    }
                }
            }
            Err(_) => break,
        }
    }

    let mut msg = String::new();
    diagram_columns.iter().for_each(|col| {
        msg.push(*col.last().unwrap());
    });
    println!("Part 1: {}", msg);

    let mut msg = String::new();
    diagram_columns_2.iter().for_each(|col| {
        msg.push(*col.last().unwrap());
    });
    println!("Part 2: {}", msg);

    Ok(())
}
