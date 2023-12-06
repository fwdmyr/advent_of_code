use std::collections::HashSet;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

#[derive(Debug)]
struct Card {
    winners: HashSet<usize>,
    numbers: HashSet<usize>,
}

impl Card {
    pub fn new(winners: HashSet<usize>, numbers: HashSet<usize>) -> Self {
        Self { winners, numbers }
    }
}

#[derive(Debug)]
struct CardResult {
    matches: usize,
    cardinality: usize,
}

impl CardResult {
    pub fn new(matches: usize, cardinality: usize) -> Self {
        Self {
            matches,
            cardinality,
        }
    }
}

fn parse_line(line: &str) -> Card {
    let start = line.find(':').unwrap();
    parse_card(&line[start + 1..])
}

fn parse_card(card: &str) -> Card {
    let mut it = card.split('|');
    let winners = parse_numbers(it.next().unwrap());
    let numbers = parse_numbers(it.next().unwrap());
    Card::new(winners, numbers)
}

fn parse_numbers(numbers: &str) -> HashSet<usize> {
    numbers
        .trim()
        .split(' ')
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<usize>().unwrap())
        .collect()
}

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut results = reader
        .lines()
        .filter_map(|line| line.ok())
        .map(|line| parse_line(&line[..]))
        .map(|card| CardResult::new(card.winners.intersection(&card.numbers).count(), 1))
        .collect::<Vec<CardResult>>();

    for i in 0..results.len() {}

    todo!("Collect into vector and process card-by-card");

    // println!("{}", sum);

    Ok(())
}
