extern crate core;

use regex::Regex;
use std::fs::File;
use std::io;
use std::io::BufRead;

#[derive(Clone, Debug)]
enum Operation {
    Squared,
    Plus(u32),
    Mult(u32),
}

#[derive(Clone, Debug)]
struct Item {
    base: u32,
    testee: u32,
}

#[derive(Debug)]
struct Monkey {
    items: Vec<Item>,
    operation: Operation,
    test: u32,
    throw: (usize, usize),
    inspections: u32
}

impl Monkey {
    fn inspect(&mut self) -> Vec<(usize, Vec<Item>)> {
        let mut throws = vec![];
        let mut throw_t = vec![];
        let mut throw_f = vec![];

        for item in &self.items {
            let mut item_base = item.base;
            let mut item_testee = item.testee;
            let mut new_item = Item {
                base: item_base,
                testee: item_testee,
            };

            match self.operation {
                Operation::Squared => {
                    new_item.testee *= item.testee;
                }
                Operation::Plus(v) => {
                    new_item.testee += v;
                }
                Operation::Mult(v) => {
                    new_item.testee *= v;
                }
            }

            new_item.testee = (new_item.testee as f32 / 3f32).floor() as u32;
            // reduction
            // - get the test numbers, for now just hardcode 23,13,19,17
            // let mut reduced = 1;
            // if new_item % 13 == 0 {
            //     reduced *= 13;
            // }
            //
            // if new_item % 17 == 0 {
            //     reduced *= 17;
            // }
            //
            // if new_item % 19 == 0 {
            //     reduced *= 19;
            // }
            //
            // if new_item % 23 == 0 {
            //     reduced *= 23;
            // }
            //
            // if reduced == 1 {
            //     reduced = new_item;
            // }

            let reduced = new_item;

            if reduced.testee % self.test == 0 {
                throw_t.push(reduced.clone());
            } else {
                throw_f.push(reduced.clone());
            }

            self.inspections += 1;
        }

        throws.push((self.throw.0, throw_t));
        throws.push((self.throw.1, throw_f));

        self.items = vec![];

        throws
    }
}

fn main() -> Result<(), io::Error> {
    let file = File::open("input.txt")?;

    let regex_monkey = Regex::new(r"^Monkey (.+):$").unwrap();
    let regex_starting_items = Regex::new(r"^\s\sStarting items: (.+)$").unwrap();
    let regex_operation = Regex::new(r"^\s\sOperation: new = (.+)$").unwrap();
    let regex_operation_plus = Regex::new(r"^old \+ (.+)$").unwrap();
    let regex_operation_mult = Regex::new(r"^old \* (.+)$").unwrap();
    let regex_test = Regex::new(r"^\s\sTest: divisible by (.+)$").unwrap();
    let regex_if_true = Regex::new(r"^\s\s\s\sIf true: throw to monkey (.+)$").unwrap();
    let regex_if_false = Regex::new(r"^\s\s\s\sIf false: throw to monkey (.+)$").unwrap();

    let mut current_monkey = 0;
    let mut starting_items = vec![];
    let mut operation = Operation::Squared;
    let mut test = 0;
    let mut throw = (0, 0);

    let mut tests = vec![];

    let mut monkeys = vec![];

    for line_r in io::BufReader::new(file).lines() {
        match line_r {
            Ok(line) => {
                if regex_monkey.is_match(&line) {
                    let prev_monkey = current_monkey;
                    current_monkey = regex_monkey.captures(&line).unwrap().get(1).unwrap().as_str().parse::<u32>().unwrap();

                    if current_monkey != prev_monkey {
                        let op = operation.clone();

                        monkeys.push(Monkey {
                            items: starting_items.clone(),
                            operation: op,
                            test,
                            throw,
                            inspections: 0,
                        });
                    }
                } else if regex_starting_items.is_match(&line) {
                    let starting_items_str = regex_starting_items.captures(&line).unwrap().get(1).unwrap().as_str();
                    starting_items = starting_items_str.split(", ").map(|v| {
                        let base = v.parse::<u32>().unwrap();
                        Item {
                            base,
                            testee: base,
                        }
                    }).collect::<Vec<Item>>();
                } else if regex_operation.is_match(&line) {
                    let op = regex_operation.captures(&line).unwrap().get(1).unwrap().as_str();
                    if op == "old * old" {
                        operation = Operation::Squared;
                    } else if regex_operation_plus.is_match(&op) {
                        operation = Operation::Plus(regex_operation_plus.captures(&op).unwrap().get(1).unwrap().as_str().parse::<u32>().unwrap());
                    } else if regex_operation_mult.is_match(&op) {
                        operation = Operation::Mult(regex_operation_mult.captures(&op).unwrap().get(1).unwrap().as_str().parse::<u32>().unwrap());
                    }
                } else if regex_test.is_match(&line) {
                    test = regex_test.captures(&line).unwrap().get(1).unwrap().as_str().parse::<u32>().unwrap();

                    if !tests.contains(&test) {
                        tests.push(test);
                    }
                } else if regex_if_true.is_match(&line) {
                    throw.0 = regex_if_true.captures(&line).unwrap().get(1).unwrap().as_str().parse::<usize>().unwrap();
                } else if regex_if_false.is_match(&line) {
                    throw.1 = regex_if_false.captures(&line).unwrap().get(1).unwrap().as_str().parse::<usize>().unwrap();
                }
            }
            Err(_) => break,
        }
    }

    let op = operation.clone();

    monkeys.push(Monkey {
        items: starting_items.clone(),
        operation: op,
        test,
        throw,
        inspections: 0,
    });

    // println!("{:#?}", tests);

    for _round in 0..20u8 {
        for i in 0..monkeys.len() {
            let throws = monkeys[i].inspect();
            for throw in throws {
                let mut t_items = throw.1;
                monkeys[throw.0].items.append(&mut t_items);
            }
        }
    }

    println!("{:#?}", monkeys);

    monkeys.sort_by(|a, b| {
        a.inspections.cmp(&b.inspections)
    });

    monkeys.reverse();

    println!("Part 1: {}", monkeys[0].inspections * monkeys[1].inspections);

    Ok(())
}