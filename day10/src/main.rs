use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::slice::Iter;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
struct Position(usize, usize);

impl Position {
    fn transform(self, orientation: Orientation) -> Self {
        match orientation {
            Orientation::North => Self((self.0 - 1).clamp(0, self.0), self.1),
            Orientation::East => Self(self.0, self.1 + 1),
            Orientation::South => Self(self.0 + 1, self.1),
            Orientation::West => Self(self.0, (self.1 - 1).clamp(0, self.1)),
        }
    }
}

#[derive(Debug)]
struct Maze {
    tiles: Vec<Vec<Pipe>>,
}

impl Maze {
    fn new(tiles: Vec<Vec<Pipe>>) -> Self {
        Self { tiles }
    }

    fn solve(&mut self) -> usize {
        let (mut p, mut o) = self.initialize();

        (p, o) = self.step(p, o);
        self.mark_at(p);

        let mut steps = 1;

        while self.at(p).ne(&Pipe::Start) {
            (p, o) = self.step(p, o);
            self.mark_at(p);

            steps += 1
        }

        steps / 2
    }

    fn reduce(&mut self) {
        self.replace_start();
        self.remove_horizontal_pipes();
        self.simplify_bends();
    }

    fn enclosed(&self) -> usize {
        self.tiles
            .iter()
            .fold(0, |acc, row| Maze::enclosed_on_line(row, acc))
    }

    fn initialize(&mut self) -> (Position, Orientation) {
        let mut p = self.start_position().unwrap();
        let os = self.start_orientations(p).unwrap();
        self.mark_at(p);

        p = p.transform(os.0);
        self.mark_at(p);

        (p, os.0)
    }

    fn replace_start(&mut self) {
        let pos = self.start_position().unwrap();
        let orientations = self.start_orientations(pos).unwrap();

        self.tiles[pos.0][pos.1] = match orientations {
            (Orientation::North, Orientation::East) => Pipe::NorthEastBend(true),
            (Orientation::East, Orientation::North) => Pipe::NorthEastBend(true),
            (Orientation::East, Orientation::South) => Pipe::SouthEastBend(true),
            (Orientation::South, Orientation::East) => Pipe::SouthEastBend(true),
            (Orientation::South, Orientation::West) => Pipe::SouthWestBend(true),
            (Orientation::West, Orientation::South) => Pipe::SouthWestBend(true),
            (Orientation::West, Orientation::North) => Pipe::NorthWestBend(true),
            (Orientation::North, Orientation::West) => Pipe::NorthWestBend(true),
            _ => panic!(),
        }
    }

    fn at(&self, pos: Position) -> Pipe {
        self.tiles[pos.0][pos.1]
    }

    fn mark_at(&mut self, pos: Position) {
        self.tiles[pos.0][pos.1] = self.tiles[pos.0][pos.1].mark();
    }

    fn remove_horizontal_pipes(&mut self) {
        self.tiles = self.tiles.iter().fold(Vec::new(), |mut outer_acc, vec| {
            let row = vec.iter().fold(Vec::new(), |mut inner_acc, tile| {
                if let Pipe::Horizontal(true) = tile {
                    inner_acc
                } else {
                    inner_acc.push(*tile);
                    inner_acc
                }
            });
            outer_acc.push(row);
            outer_acc
        });
    }

    fn simplify_bends(&mut self) {
        self.tiles = self.tiles.iter().fold(Vec::new(), |mut acc, row| {
            let mut elements = Vec::new();
            let mut count = 0;
            while count < row.len() - 1 {
                match (row[count], row[count + 1]) {
                    (Pipe::NorthEastBend(true), Pipe::NorthWestBend(true)) => {
                        count += 2;
                    }
                    (Pipe::SouthEastBend(true), Pipe::SouthWestBend(true)) => {
                        count += 2;
                    }
                    (Pipe::SouthEastBend(true), Pipe::NorthWestBend(true)) => {
                        elements.push(Pipe::Vertical(true));
                        count += 2;
                    }
                    (Pipe::NorthEastBend(true), Pipe::SouthWestBend(true)) => {
                        elements.push(Pipe::Vertical(true));
                        count += 2;
                    }
                    _ => {
                        elements.push(row[count]);
                        count += 1;
                    }
                }
            }

            if let Some(pipe) = row.last() {
                match pipe {
                    Pipe::Ground | Pipe::Vertical(_) => {
                        elements.push(*pipe);
                    }
                    _ => {}
                }
            }

            acc.push(elements);
            acc
        });
    }

    fn compute_parity(vec: &Vec<Pipe>) -> Vec<u32> {
        vec.iter()
            .scan(0, |state, tile| {
                if let Pipe::Vertical(true) = tile {
                    *state += 1;
                }
                Some(*state)
            })
            .collect()
    }

    fn enclosed_on_line(vec: &Vec<Pipe>, total: usize) -> usize {
        let parity = Maze::compute_parity(vec);

        vec.iter()
            .enumerate()
            .fold(total, |acc, (i, tile)| match tile {
                Pipe::Ground
                | Pipe::Vertical(false)
                | Pipe::Horizontal(false)
                | Pipe::NorthEastBend(false)
                | Pipe::NorthWestBend(false)
                | Pipe::SouthEastBend(false)
                | Pipe::SouthWestBend(false)
                    if parity[i] % 2 != 0 =>
                {
                    acc + 1
                }
                _ => acc,
            })
    }

    fn step(&self, pos: Position, orn: Orientation) -> (Position, Orientation) {
        let next_orn = orn.transform(self.at(pos));
        let next_pos = pos.transform(next_orn);
        (next_pos, next_orn)
    }

    fn start_position(&self) -> Option<Position> {
        for (i, row) in self.tiles.iter().enumerate() {
            for (j, tile) in row.iter().enumerate() {
                if let Pipe::Start = tile {
                    return Some(Position(i, j));
                }
            }
        }
        None
    }

    fn start_orientations(&self, start: Position) -> Option<(Orientation, Orientation)> {
        let vec = Orientation::iterator()
            .into_iter()
            .filter_map(|o| o.valid(self.at(start.transform(*o))))
            .collect::<Vec<Orientation>>();

        if let (Some(first), Some(second)) = (vec.first(), vec.last()) {
            Some((*first, *second))
        } else {
            None
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
enum Orientation {
    North,
    East,
    South,
    West,
}

impl Orientation {
    pub fn iterator() -> Iter<'static, Orientation> {
        static DIRECTIONS: [Orientation; 4] = [
            Orientation::North,
            Orientation::East,
            Orientation::South,
            Orientation::West,
        ];
        DIRECTIONS.iter()
    }

    fn transform(self, pipe: Pipe) -> Self {
        match pipe {
            Pipe::Vertical(_) => self,
            Pipe::Horizontal(_) => self,
            Pipe::NorthEastBend(_) if self.eq(&Orientation::South) => Orientation::East,
            Pipe::NorthEastBend(_) if self.eq(&Orientation::West) => Orientation::North,
            Pipe::NorthWestBend(_) if self.eq(&Orientation::South) => Orientation::West,
            Pipe::NorthWestBend(_) if self.eq(&Orientation::East) => Orientation::North,
            Pipe::SouthEastBend(_) if self.eq(&Orientation::North) => Orientation::East,
            Pipe::SouthEastBend(_) if self.eq(&Orientation::West) => Orientation::South,
            Pipe::SouthWestBend(_) if self.eq(&Orientation::North) => Orientation::West,
            Pipe::SouthWestBend(_) if self.eq(&Orientation::East) => Orientation::South,
            _ => panic!("Bad transform {:?} given {:?}", self, pipe),
        }
    }

    fn valid(&self, pipe: Pipe) -> Option<Orientation> {
        match self {
            Orientation::North => Orientation::valid_north(pipe),
            Orientation::East => Orientation::valid_east(pipe),
            Orientation::South => Orientation::valid_south(pipe),
            Orientation::West => Orientation::valid_west(pipe),
        }
    }

    fn valid_north(pipe: Pipe) -> Option<Orientation> {
        match pipe {
            Pipe::Vertical(_) | Pipe::SouthEastBend(_) | Pipe::SouthWestBend(_) => {
                Some(Orientation::North)
            }
            _ => None,
        }
    }

    fn valid_east(pipe: Pipe) -> Option<Orientation> {
        match pipe {
            Pipe::Horizontal(_) | Pipe::NorthWestBend(_) | Pipe::SouthWestBend(_) => {
                Some(Orientation::East)
            }
            _ => None,
        }
    }

    fn valid_south(pipe: Pipe) -> Option<Orientation> {
        match pipe {
            Pipe::Vertical(_) | Pipe::NorthEastBend(_) | Pipe::NorthWestBend(_) => {
                Some(Orientation::South)
            }
            _ => None,
        }
    }

    fn valid_west(pipe: Pipe) -> Option<Orientation> {
        match pipe {
            Pipe::Horizontal(_) | Pipe::NorthEastBend(_) | Pipe::SouthEastBend(_) => {
                Some(Orientation::West)
            }
            _ => None,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Pipe {
    Vertical(bool),
    Horizontal(bool),
    NorthEastBend(bool),
    NorthWestBend(bool),
    SouthEastBend(bool),
    SouthWestBend(bool),
    Ground,
    Start,
}

impl Pipe {
    fn from_char(ch: char) -> Self {
        match ch {
            '|' => Self::Vertical(false),
            '-' => Self::Horizontal(false),
            'L' => Self::NorthEastBend(false),
            'J' => Self::NorthWestBend(false),
            '7' => Self::SouthWestBend(false),
            'F' => Self::SouthEastBend(false),
            '.' => Self::Ground,
            'S' => Self::Start,
            _ => panic!(),
        }
    }

    fn mark(self) -> Self {
        match self {
            Pipe::Vertical(_) => Pipe::Vertical(true),
            Pipe::Horizontal(_) => Pipe::Horizontal(true),
            Pipe::NorthEastBend(_) => Pipe::NorthEastBend(true),
            Pipe::NorthWestBend(_) => Pipe::NorthWestBend(true),
            Pipe::SouthEastBend(_) => Pipe::SouthEastBend(true),
            Pipe::SouthWestBend(_) => Pipe::SouthWestBend(true),
            _ => self,
        }
    }
}

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut maze = Maze::new(
        reader
            .lines()
            .filter_map(|line| line.ok())
            .map(|line| line.chars().map(|ch| Pipe::from_char(ch)).collect())
            .collect::<Vec<Vec<Pipe>>>(),
    );

    let _ = maze.solve();

    maze.reduce();

    let res = maze.enclosed();

    println!("{}", res);

    Ok(())
}
