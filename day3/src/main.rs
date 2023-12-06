use std::collections::HashMap;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

use itertools::Itertools;

#[derive(Debug, PartialEq)]
enum Token {
    Number((usize, usize), usize),
    Gear(Option<usize>),
}

fn build_frame(pos: (usize, usize), width: usize) -> Vec<(usize, usize)> {
    let (row, col) = (pos.0 as i32, pos.1 as i32);
    let range = 0..140;

    itertools::iproduct!(row - 1..row + 2, col - 1..col + width as i32 + 1)
        .filter_map(|(i, j)| {
            if range.contains(&i) && range.contains(&j) {
                Some((i as usize, j as usize))
            } else {
                None
            }
        })
        .collect()
}

fn validate_token(tokens: &HashMap<(usize, usize), Token>, pos: (usize, usize)) -> Option<Token> {
    match tokens.get(&pos) {
        Some(Token::Gear(None)) => {
            let frame = build_frame(pos, 1);
            let vals = frame
                .iter()
                .filter_map(|fpos| {
                    if let Some(Token::Number(pos, val)) = tokens.get(&fpos) {
                        Some((pos, val))
                    } else {
                        None
                    }
                })
                .unique_by(|data| data.0)
                .map(|data| data.1.clone())
                .collect::<Vec<usize>>();
            if vals.len() == 2 {
                Some(Token::Gear(Some(vals.into_iter().product())))
            } else {
                None
            }
        }
        _ => None,
    }
}

fn parse_line(row: usize, line: &str) -> Vec<((usize, usize), Token)> {
    let mut it = line.char_indices();

    let mut digit = String::new();
    let mut digits = Vec::new();

    while let Some((i, ch)) = it.next() {
        if ch.is_ascii_digit() {
            digit.push(ch);
        } else if let Ok(d) = digit.parse::<usize>() {
            for j in 1..=digit.len() {
                let pos = (row, i - digit.len());
                digits.push(((row, i - j), Token::Number(pos, d)));
            }
            digit.clear();
        }
        if ch.eq(&'*') {
            digits.push(((row, i), Token::Gear(None)));
        }
    }

    if let Ok(d) = digit.parse::<usize>() {
        for j in 1..=digit.len() {
            let pos = (row, 140 - digit.len());
            digits.push(((row, 140 - j), Token::Number(pos, d)));
        }
    }

    digits
}

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let schematic = reader
        .lines()
        .filter_map(|line| line.ok())
        .enumerate()
        .fold(HashMap::new(), |acc, (row, line)| {
            let mut acc = acc;
            acc.extend(parse_line(row, &line[..]));
            acc
        });

    let sum = schematic
        .iter()
        .filter_map(|(pos, _)| validate_token(&schematic, pos.clone()))
        .fold(0, |acc, t| match t {
            Token::Gear(Some(val)) => acc + val,
            _ => acc,
        });

    println!("{}", sum);

    Ok(())
}
