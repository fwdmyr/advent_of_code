use std::collections::HashMap;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::str::FromStr;

#[derive(Debug)]
enum Attribute {
    Seed,
    Soil,
    Fertilizer,
    Water,
    Light,
    Temperature,
    Humidity,
    Location,
}

#[derive(Debug)]
struct ParseAttributeError;

impl FromStr for Attribute {
    type Err = ParseAttributeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "seed" => Ok(Attribute::Seed),
            "soil" => Ok(Attribute::Soil),
            "fertilizer" => Ok(Attribute::Fertilizer),
            "water" => Ok(Attribute::Water),
            "light" => Ok(Attribute::Light),
            "temperature" => Ok(Attribute::Temperature),
            "humidity" => Ok(Attribute::Humidity),
            "location" => Ok(Attribute::Location),
            _ => Err(ParseAttributeError),
        }
    }
}

#[derive(Debug)]
struct AttributeRange {
    dest: usize,
    src: usize,
    len: usize,
}

impl AttributeRange {
    fn contains(&self, val: usize) -> bool {
        (self.src..self.src + self.len).contains(&val)
    }

    fn transform(&self, val: usize) -> usize {
        self.dest + val - self.src
    }
}

#[derive(Debug)]
struct AttributeMap {
    from: Attribute,
    to: Attribute,
    data: Vec<AttributeRange>,
}

impl AttributeMap {
    fn map(&self, val: usize) -> usize {
        if let Some(range) = self.data.iter().find(|r| r.contains(val)) {
            range.transform(val)
        } else {
            val
        }
    }
}

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut it = reader.lines().filter_map(|line| line.ok());

    let seeds = it
        .next()
        .unwrap()
        .split(' ')
        .filter_map(|x| x.parse::<usize>().ok())
        .collect::<Vec<usize>>();

    let mut new_block = false;

    let mut attrs = Vec::new();

    while let Some(line) = it.next() {
        if line.is_empty() {
            new_block = true;
        } else {
            if new_block {
                let mut it = line.split_whitespace().next().unwrap().split('-');
                let data = Vec::new();
                let from = Attribute::from_str(it.next().unwrap()).unwrap();
                it.next();
                let to = Attribute::from_str(it.next().unwrap()).unwrap();
                attrs.push(AttributeMap { from, to, data });
            } else {
                let mut it = line.split_whitespace();
                let dest = it.next().unwrap().parse().unwrap();
                let src = it.next().unwrap().parse().unwrap();
                let len = it.next().unwrap().parse().unwrap();
                let range = AttributeRange { dest, src, len };
                attrs.last_mut().unwrap().data.push(range);
            }
            new_block = false;
        }
    }

    let res = seeds
        .iter()
        .map(|s| s.clone())
        .map(|s| attrs.iter().fold(s, |acc, m| m.map(acc)))
        .min()
        .unwrap();

    println!("{}", res);

    Ok(())
}
