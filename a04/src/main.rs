use std::fs::File;
use std::io;
use std::io::BufRead;
use std::ops::Range;

fn main() -> Result<(), io::Error> {
    let file = File::open("input.txt")?;

    let mut count_contained = 0u32;
    let mut count_overlap = 0u32;

    for line_r in io::BufReader::new(file).lines() {
        match line_r {
            Ok(line) => {
                let mut ranges: Vec<Range<u32>> = vec![];

                for elf in line.split(',') {
                    let numbers: Vec<u32> = elf
                        .split('-')
                        .map(|value| value.parse::<u32>().unwrap())
                        .collect();

                    ranges.push(numbers[0]..numbers[1]);
                }

                if (ranges[0].start >= ranges[1].start && ranges[0].end <= ranges[1].end)
                    || (ranges[1].start >= ranges[0].start && ranges[1].end <= ranges[0].end)
                {
                    count_contained += 1;
                    count_overlap += 1;
                } else if ranges[0].start <= ranges[1].end && ranges[0].end >= ranges[1].start {
                    count_overlap += 1;
                }
            }
            Err(_) => break,
        }
    }

    println!("part 1: {}", count_contained);
    println!("part 2: {}", count_overlap);

    Ok(())
}
