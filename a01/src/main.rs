use std::fs::File;
use std::io;
use std::io::BufRead;

fn main() -> Result<(), io::Error> {
    let file = File::open("input.txt")?;

    let mut elves = vec![];
    let mut current = 0u32;

    for line_r in io::BufReader::new(file).lines() {
        match line_r {
            Ok(line) => {
                // new elf
                if line.eq("") {
                    elves.push(current);
                    current = 0;
                } else {
                    current += line.trim().parse::<u32>().unwrap();
                }
            }
            Err(_) => break,
        }
    }

    elves.sort();
    elves.reverse();

    // part 1: the largest amount
    println!("{:?}", elves.first().unwrap());

    // part 2: sum of top 3 largest amounts
    println!("{:?}", elves[0..3].iter().sum::<u32>());

    Ok(())
}
