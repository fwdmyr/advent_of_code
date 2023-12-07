use std::fs::File;
use std::io::{self, prelude::*, BufReader};

#[derive(Debug)]
struct Race {
    time: f32,
    distance: f32,
}

impl Race {
    fn get_roots(&self) -> Option<(f32, f32)> {
        let root_expr = 0.25 * self.time * self.time - self.distance;
        if root_expr >= 0.0 {
            Some((
                0.5 * self.time - root_expr.sqrt(),
                0.5 * self.time + root_expr.sqrt(),
            ))
        } else {
            None
        }
    }
}

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut it = reader.lines().filter_map(|line| line.ok());

    let times = it
        .next()
        .unwrap()
        .split(' ')
        .filter_map(|x| x.parse::<f32>().ok())
        .collect::<Vec<f32>>();

    let distances = it
        .next()
        .unwrap()
        .split(' ')
        .filter_map(|x| x.parse::<f32>().ok())
        .collect::<Vec<f32>>();

    let races = times
        .iter()
        .zip(distances.iter())
        .map(|(t, d)| Race {
            time: t.clone(),
            distance: d.clone(),
        })
        .collect::<Vec<Race>>();

    let res = races
        .iter()
        .filter_map(|r| r.get_roots())
        .map(|r| r.1.floor() as u32 - r.0.ceil() as u32 + 1)
        .product::<u32>();

    let time = times
        .iter()
        .map(|t| t.to_string())
        .collect::<String>()
        .parse::<f32>()
        .unwrap();
    let distance = distances
        .iter()
        .map(|t| t.to_string())
        .collect::<String>()
        .parse::<f32>()
        .unwrap();

    let the_race = Race { time, distance };

    println!("{:?}", the_race);

    let res = the_race.get_roots().unwrap();

    let res = res.1.floor() as u32 - res.0.ceil() as u32;

    println!("{:?}", res);

    Ok(())
}
