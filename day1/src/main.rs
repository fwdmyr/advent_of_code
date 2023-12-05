use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn to_digit(s: &str) -> u32 {
    match s {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        _ => s.parse::<u32>().unwrap(),
    }
}

fn parse_line(line: String) -> (u32, u32) {
    let words = vec![
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    let digits = vec!["1", "2", "3", "4", "5", "6", "7", "8", "9"];

    let (_, p) = words
        .iter()
        .chain(digits.iter())
        .map(|p| (line.find(p), p.to_string()))
        .filter_map(|(i, p)| if let Some(i) = i { Some((i, p)) } else { None })
        .min_by(|(i, _), (j, _)| i.cmp(j))
        .unwrap();

    let (_, q) = words
        .iter()
        .chain(digits.iter())
        .map(|p| (line.rfind(p), p.to_string()))
        .filter_map(|(i, p)| if let Some(i) = i { Some((i, p)) } else { None })
        .max_by(|(i, p), (j, q)| (i + &p.len()).cmp(&(j + q.len())))
        .unwrap();

    (to_digit(&p[..]), to_digit(&q[..]))
}

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let sum = reader.lines().fold(0, |acc, line| {
        let (first, last) = parse_line(line.unwrap_or("".to_string()));
        acc + 10 * first + last
    });

    println!("{}", sum);

    Ok(())
}

