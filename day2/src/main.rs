use std::fs::File;
use std::io::{self, prelude::*, BufReader};

#[derive(Debug)]
struct Cubes {
    red: u32,
    green: u32,
    blue: u32,
}

impl Default for Cubes {
    fn default() -> Self {
        Self {
            red: 0,
            green: 0,
            blue: 0,
        }
    }
}

impl Cubes {
    fn new(red: u32, green: u32, blue: u32) -> Self {
        Self { red, green, blue }
    }
}

fn required_cubes(game: &Vec<Cubes>) -> Cubes {
    Cubes {
        red: game
            .iter()
            .max_by(|lhs, rhs| lhs.red.cmp(&rhs.red))
            .map(|color| color.red)
            .unwrap(),
        green: game
            .iter()
            .max_by(|lhs, rhs| lhs.green.cmp(&rhs.green))
            .map(|color| color.green)
            .unwrap(),
        blue: game
            .iter()
            .max_by(|lhs, rhs| lhs.blue.cmp(&rhs.blue))
            .map(|color| color.blue)
            .unwrap(),
    }
}

fn parse_line(line: &str) -> Vec<Cubes> {
    let start = line.find(':').unwrap();
    parse_game(&line[start + 1..])
}

fn parse_game(game: &str) -> Vec<Cubes> {
    game.split(';').map(|turn| parse_turn(turn)).collect()
}

fn parse_turn(turn: &str) -> Cubes {
    turn.split(',')
        .fold(Cubes::default(), |acc, cube| parse_cube(acc, cube))
}

fn parse_cube(cubes: Cubes, cube: &str) -> Cubes {
    let mut it = cube.trim().split(' ');

    let count = it.next().unwrap().parse::<u32>().unwrap();

    match it.next() {
        Some("red") => Cubes::new(count, cubes.green, cubes.blue),
        Some("green") => Cubes::new(cubes.red, count, cubes.blue),
        Some("blue") => Cubes::new(cubes.red, cubes.green, count),
        _ => cubes,
    }
}

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let sum = reader
        .lines()
        .filter_map(|line| line.ok())
        .map(|line| parse_line(&line[..]))
        .map(|game| required_cubes(&game))
        .fold(0, |acc, cubes| acc + cubes.red * cubes.green * cubes.blue);

    println!("{}", sum);

    Ok(())
}
