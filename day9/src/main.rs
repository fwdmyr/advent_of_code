use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn difference(vec: Vec<i32>) -> Vec<i32> {
    vec.windows(2).map(|w| w[1] - w[0]).collect()
}

fn recurse(vec: Vec<i32>) -> i32 {
    println!("{:?}", vec);
    if vec.iter().any(|v| v.clone() != 0) {
        vec.first().unwrap().clone() - recurse(difference(vec))
    } else {
        vec.first().unwrap().clone()
    }
}

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let res = reader
        .lines()
        .filter_map(|line| line.ok())
        .map(|line| {
            line.split_whitespace()
                .map(|ch| ch.parse().unwrap())
                .collect::<Vec<i32>>()
        })
        .fold(0, |acc, seq| acc + recurse(seq));

    println!("{}", res);

    Ok(())
}
