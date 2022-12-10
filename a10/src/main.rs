extern crate core;

use regex::Regex;
use std::fs::File;
use std::io;
use std::io::BufRead;

enum InstructionType {
    Noop,
    Addx,
}

trait Instruction {
    fn get_type(&self) -> InstructionType;
    fn cycles_left(&self) -> i32;
    fn update(&mut self);
    fn finish(&self, register: &mut i32);
}

struct Noop {
    cycles: i32,
}

impl Noop {
    fn new() -> Noop {
        Noop { cycles: 1 }
    }
}

impl Instruction for Noop {
    fn get_type(&self) -> InstructionType {
        InstructionType::Noop
    }

    fn cycles_left(&self) -> i32 {
        self.cycles
    }

    fn update(&mut self) {
        self.cycles -= 1;
    }

    fn finish(&self, mut _register: &mut i32) {
        // nothing
    }
}

struct Addx {
    cycles: i32,
    value: i32,
}

impl Addx {
    fn new(value: i32) -> Addx {
        Addx { cycles: 2, value }
    }
}

impl Instruction for Addx {
    fn get_type(&self) -> InstructionType {
        InstructionType::Addx
    }

    fn cycles_left(&self) -> i32 {
        self.cycles
    }

    fn update(&mut self) {
        self.cycles -= 1;
    }

    fn finish(&self, register: &mut i32) {
        *register += self.value;
    }
}

fn main() -> Result<(), io::Error> {
    let file = File::open("input.txt")?;

    let regex_addx = Regex::new(r"^addx (.+)$").unwrap();

    let mut cycle = 1u32;
    let mut register_x = 1i32;
    let mut instructions = vec![];

    for line_r in io::BufReader::new(file).lines() {
        match line_r {
            Ok(line) => {
                if line.eq("noop") {
                    instructions.push((InstructionType::Noop, None));
                } else {
                    let caps = regex_addx.captures(&line).unwrap();
                    let value = caps.get(1).unwrap().as_str().parse::<i32>().unwrap();

                    instructions.push((InstructionType::Addx, Some(value)));
                }
            }
            Err(_) => break,
        }
    }

    let mut last_instruction = 0usize;
    let mut running_instruction: Box<dyn Instruction>;

    match instructions.get(last_instruction) {
        None => panic!("oh no"),
        Some(ins) => match ins.0 {
            InstructionType::Noop => {
                running_instruction = Box::new(Noop::new());
            }
            InstructionType::Addx => {
                running_instruction = Box::new(Addx::new(ins.1.unwrap()));
            }
        },
    }

    let mut sum_of_strengths = 0;

    loop {
        // start cycle
        if running_instruction.cycles_left() == 0 {
            last_instruction += 1;
            match instructions.get(last_instruction) {
                None => break,
                Some(ins) => match ins.0 {
                    InstructionType::Noop => {
                        running_instruction = Box::new(Noop::new());
                    }
                    InstructionType::Addx => {
                        running_instruction = Box::new(Addx::new(ins.1.unwrap()));
                    }
                },
            }
        }

        // during cycle
        render_next(&cycle, &register_x);

        if [20, 60, 100, 140, 180, 220].contains(&cycle) {
            sum_of_strengths += (cycle as i32) * register_x;
        }

        // after cycle
        running_instruction.update();

        if running_instruction.cycles_left() == 0 {
            running_instruction.finish(&mut register_x);
        }

        cycle += 1;
    }

    println!();
    println!("Part 1: {}", sum_of_strengths);

    Ok(())
}

fn render_next(cycle: &u32, register: &i32) {
    if [41, 81, 121, 161, 201].contains(cycle) {
        println!();
    }

    let mut cmp = (*cycle as i32) - 1;
    if cmp >= 40 {
        cmp %= 40;
    }

    if [register - 1, *register, register + 1].contains(&cmp) {
        print!("#");
    } else {
        print!(".");
    }
}
