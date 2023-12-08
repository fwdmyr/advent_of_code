use num::integer;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

#[derive(Debug, Clone)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug, Clone)]
struct Directions {
    sequence: Vec<Direction>,
    i: usize,
}

impl Directions {
    fn new(sequence: Vec<Direction>) -> Self {
        Self { sequence, i: 0 }
    }

    fn gen(&mut self) -> Direction {
        let direction = self.sequence[self.i].clone();
        self.i += 1;
        self.i %= self.sequence.len();
        direction
    }

    fn reset(&mut self) {
        self.i = 0;
    }
}

#[derive(Debug)]
struct Next {
    left: String,
    right: String,
}

fn parse_line(line: &str) -> (String, Next) {
    let mut it = line.split(" = ");
    let key = it.next().unwrap().to_string();
    let value = parse_next(it.next().unwrap());
    (key, value)
}

fn parse_next(next: &str) -> Next {
    let mut it = next.trim_matches(|ch| ch == '(' || ch == ')').split(", ");
    let left = it.next().unwrap().to_string();
    let right = it.next().unwrap().to_string();
    Next { left, right }
}

fn parse_directions(directions: &str) -> Directions {
    let sequence = directions
        .chars()
        .map(|ch| match ch {
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => panic!(),
        })
        .collect();
    Directions::new(sequence)
}

fn validate_key(key: &String) -> bool {
    key.ends_with("Z")
}

fn step(nodes: &HashMap<String, Next>, key: String, direction: Direction) -> String {
    match direction {
        Direction::Left => nodes.get(&key).unwrap().left.clone(),
        Direction::Right => nodes.get(&key).unwrap().right.clone(),
    }
}

fn find_cycle(nodes: &HashMap<String, Next>, key: String, directions: Directions) -> u64 {
    let mut key = key;
    let mut directions = directions;
    directions.reset();

    let mut counter = 0;

    while !validate_key(&key) {
        let direction = directions.gen();

        key = step(&nodes, key, direction);

        counter += 1;
    }

    counter
}

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut it = reader.lines().filter_map(|line| line.ok());

    let directions = parse_directions(&it.next().unwrap()[..]);
    it.next();

    let nodes = it
        .map(|line| parse_line(&line[..]))
        .collect::<HashMap<String, Next>>();

    let counts = nodes
        .iter()
        .filter_map(|(k, _)| k.ends_with("A").then(|| k.clone()))
        .map(|k| find_cycle(&nodes, k, directions.clone()))
        .collect::<Vec<u64>>();

    let res = counts
        .into_iter()
        .reduce(|acc, count| integer::lcm(acc, count))
        .unwrap();

    println!("{:?}", res);

    Ok(())
}
