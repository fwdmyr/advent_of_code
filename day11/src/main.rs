use std::fs::File;
use std::io::{self, prelude::*, BufReader};

#[derive(Debug)]
struct Position(usize, usize);

impl Position {
    fn distance_to(&self, other: &Position) -> usize {
        self.0.abs_diff(other.0) + self.1.abs_diff(other.1)
    }
}

#[derive(Debug)]
struct Galaxy {
    pos: Position,
}

impl Galaxy {
    fn new(pos: Position) -> Self {
        Self { pos }
    }

    fn distance_to(&self, other: &Galaxy) -> usize {
        self.pos.distance_to(&other.pos)
    }

    fn shift_horizontal(&mut self, vec: &Vec<usize>) {
        let offset = vec
            .iter()
            .filter(|x| x < &&self.pos.1)
            .fold(0, |acc, _| acc + 999_999);
        self.pos.1 += offset;
    }

    fn shift_vertical(&mut self, vec: &Vec<usize>) {
        let offset = vec
            .iter()
            .filter(|x| x < &&self.pos.0)
            .fold(0, |acc, _| acc + 999_999);
        self.pos.0 += offset;
    }
}

fn shift(galaxies: &mut Vec<Galaxy>) {
    let empty_horizontal = empty_columns(galaxies);
    let empty_vertical = empty_rows(galaxies);
    galaxies.iter_mut().for_each(|g| {
        g.shift_horizontal(&empty_horizontal);
        g.shift_vertical(&empty_vertical);
    })
}

fn empty_rows(galaxies: &Vec<Galaxy>) -> Vec<usize> {
    (0..140)
        .filter(|i| !galaxies.iter().any(|g| i.eq(&g.pos.0)))
        .collect()
}

fn empty_columns(galaxies: &Vec<Galaxy>) -> Vec<usize> {
    (0..140)
        .filter(|j| !galaxies.iter().any(|g| j.eq(&g.pos.1)))
        .collect()
}

fn pairs(n: usize) -> Vec<(usize, usize)> {
    (0..n).fold(Vec::new(), |mut oacc, i| {
        oacc.extend((i + 1..n).fold(Vec::new(), |mut iacc, j| {
            iacc.push((i, j));
            iacc
        }));
        oacc
    })
}

fn distances(galaxies: &Vec<Galaxy>) -> Vec<usize> {
    pairs(galaxies.len())
        .into_iter()
        .map(|(i, j)| galaxies[i].distance_to(&galaxies[j]))
        .collect()
}

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut galaxies = reader
        .lines()
        .filter_map(|line| line.ok())
        .enumerate()
        .fold(Vec::new(), |mut acc, (i, line)| {
            line.chars().enumerate().for_each(|(j, ch)| {
                if ch.eq(&'#') {
                    acc.push(Galaxy::new(Position(i, j)));
                }
            });
            acc
        });

    shift(&mut galaxies);

    println!("{:?}", distances(&galaxies).into_iter().sum::<usize>());

    Ok(())
}
