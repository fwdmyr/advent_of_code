use std::collections::HashMap;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

#[derive(Debug, PartialEq)]
enum Token {
    Number(usize, usize),
    Symbol,
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
        Some(Token::Number(value, width)) => {
            let frame = build_frame(pos, *width);
            frame
                .iter()
                .any(|fpos| tokens.get(&fpos).is_some_and(|t| t.eq(&Token::Symbol)))
                .then(|| Token::Number(value.clone(), width.clone()))
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
            digits.push(((row, i - digit.len()), Token::Number(d, digit.len())));
            digit.clear();
        }
        if !ch.is_ascii_digit() && ch.ne(&'.') {
            digits.push(((row, i), Token::Symbol));
        }
    }

    if let Ok(d) = digit.parse::<usize>() {
        digits.push(((row, 140 - digit.len()), Token::Number(d, digit.len())));
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
            Token::Number(val, _) => {
                acc + val
            }
            _ => acc,
        });

    println!("{}", sum);

    Ok(())
}
