use regex::Regex;
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::io;
use std::io::BufRead;

#[derive(Clone, Hash, Debug)]
enum FSType {
    File,
    Directory,
}

#[derive(Clone, Debug)]
struct FSNode {
    name: String,
    fs_type: FSType,
    size: u32,
    parents: Vec<u64>,
}

impl FSNode {
    fn calculate_hash(&self) -> u64 {
        let mut s = DefaultHasher::new();
        self.name.hash(&mut s);
        self.fs_type.hash(&mut s);
        self.parents.hash(&mut s);
        s.finish()
    }
}

fn main() -> Result<(), io::Error> {
    let file = File::open("input.txt")?;

    let regex_cd_in = Regex::new(r"^\$ cd ([a-z]+|/)$").unwrap();
    let regex_file = Regex::new(r"^(\d+) (\D+)$").unwrap();
    let regex_dir = Regex::new(r"^dir (\D+)$").unwrap();

    let mut nodes = HashMap::new();

    let root_node = Box::new(FSNode {
        name: "/".to_string(),
        fs_type: FSType::Directory,
        size: 0,
        parents: vec![],
    });

    let mut current = root_node.calculate_hash();

    nodes.insert(current, root_node);

    for line_r in io::BufReader::new(file).lines() {
        match line_r {
            Ok(line) => {
                if line.eq("$ cd ..") {
                    current = *nodes
                        .get(&current)
                        .unwrap()
                        .as_ref()
                        .parents
                        .last()
                        .unwrap();
                } else if regex_cd_in.is_match(&line) {
                    let dirname = regex_cd_in
                        .captures(&line)
                        .unwrap()
                        .get(1)
                        .unwrap()
                        .as_str();
                    current = find_dir(dirname, current, &nodes);
                } else if regex_file.is_match(&line) {
                    let parents = get_parents(current, &nodes);

                    let f = FSNode {
                        name: regex_file
                            .captures(&line)
                            .unwrap()
                            .get(2)
                            .unwrap()
                            .as_str()
                            .to_string(),
                        fs_type: FSType::File,
                        size: regex_file
                            .captures(&line)
                            .unwrap()
                            .get(1)
                            .unwrap()
                            .as_str()
                            .parse::<u32>()
                            .unwrap(),
                        parents,
                    };

                    let hash = f.calculate_hash();

                    if nodes.contains_key(&hash) {
                        continue;
                    }

                    update_sizes(f.size, &f.parents, &mut nodes);
                    nodes.insert(hash, Box::new(f));
                } else if regex_dir.is_match(&line) {
                    let name = regex_dir
                        .captures(&line)
                        .unwrap()
                        .get(1)
                        .unwrap()
                        .as_str()
                        .trim()
                        .to_string();
                    let parents = get_parents(current, &nodes);

                    let f = FSNode {
                        name,
                        fs_type: FSType::Directory,
                        size: 0,
                        parents,
                    };

                    let hash = f.calculate_hash();

                    if nodes.contains_key(&hash) {
                        continue;
                    }

                    nodes.insert(hash, Box::new(f));
                }
            }
            Err(_) => break,
        }
    }

    let mut sum = 0;

    for (_i, node) in nodes.iter() {
        match node.fs_type {
            FSType::File => {}
            FSType::Directory => {
                // println!("{} {} {}", i, node.name, node.size);
                if node.size < 100000 {
                    sum += node.size;
                }
            }
        }
    }

    println!("Part 1: {}", sum);

    let root = nodes.get(&find_dir("/", 0, &nodes)).unwrap();

    println!("Part 2:");
    println!("root size: {}", root.size);
    let unused_space = 70000000 - root.size;
    println!("unused space: {}", unused_space);
    let need_to_free = 30000000 - unused_space;
    println!("need to free: {}", need_to_free);

    let mut candidates: Vec<&FSNode> = nodes
        .iter()
        .filter(|(_i, n)| match n.fs_type {
            FSType::File => false,
            FSType::Directory => n.size >= need_to_free,
        })
        .map(|(_i, n)| n.as_ref())
        .collect();

    candidates.sort_by(|a, b| a.size.cmp(&b.size));

    candidates.reverse();

    println!(
        "delete folder with size: {}",
        candidates.last().unwrap().size
    );

    Ok(())
}

fn update_sizes(size: u32, parents: &[u64], nodes: &mut HashMap<u64, Box<FSNode>>) {
    for p in parents.iter() {
        let mut parent = nodes.get(p).unwrap().as_ref().to_owned();
        parent.size += size;

        nodes.insert(*p, Box::new(parent));
    }
}

fn find_dir(name: &str, parent: u64, nodes: &HashMap<u64, Box<FSNode>>) -> u64 {
    match nodes.iter().find(|n| match n.1.fs_type {
        FSType::File => false,
        FSType::Directory => match n.1.parents.last() {
            None => name == "/" && n.1.name == "/",
            Some(p) => *p == parent && n.1.name == *name,
        },
    }) {
        None => 0,
        Some(v) => *v.0,
    }
}

fn get_parents(current: u64, nodes: &HashMap<u64, Box<FSNode>>) -> Vec<u64> {
    match nodes.get(&current) {
        None => panic!("oh no"),
        Some(node) => {
            let mut parents = node.parents.clone();
            parents.push(current);

            parents
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{FSNode, FSType};

    #[test]
    fn test_hasher() {
        let node = FSNode {
            name: "test".to_string(),
            fs_type: FSType::Directory,
            size: 0,
            parents: vec![1234, 1111, 2222],
        };

        assert_eq!(1048965116848191627, node.calculate_hash());

        let node2 = FSNode {
            name: "test".to_string(),
            fs_type: FSType::File,
            size: 0,
            parents: vec![1234, 1111, 2222],
        };

        assert_ne!(node2.calculate_hash(), node.calculate_hash());
    }
}
