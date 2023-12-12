use std::fs::File;
use std::io::{self, prelude::*, BufReader};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Entry {
    Operational,
    Damaged,
    Unknown,
}

impl Entry {
    fn from_char(ch: char) -> Entry {
        match ch {
            '.' => Self::Operational,
            '#' => Self::Damaged,
            '?' => Self::Unknown,
            _ => panic!(),
        }
    }
}

#[derive(Debug, Clone)]
struct Record {
    entries: Vec<Entry>,
    groups: Vec<usize>,
}

impl Record {
    fn new(entries: Vec<Entry>, groups: Vec<usize>) -> Self {
        Self { entries, groups }
    }

    fn parse(record: &str) -> Self {
        let mut it = record.split_whitespace();

        let mut entries = it
            .next()
            .unwrap()
            .chars()
            .map(|ch| Entry::from_char(ch))
            .collect::<Vec<Entry>>();
        entries.push(Entry::Unknown);
        entries = entries.repeat(5);
        entries.pop();

        println!("{:?}", entries);

        let mut groups = it
            .next()
            .unwrap()
            .split(',')
            .map(|s| s.parse().unwrap())
            .collect::<Vec<usize>>();
        groups = groups.repeat(5);

        println!("{:?}", groups);

        Record::new(entries, groups)
    }

    fn validate(&self) -> bool {
        let res = self
            .entries
            .split(|e| *e == Entry::Operational)
            .filter(|s| !s.is_empty())
            .map(|s| s.len())
            .collect::<Vec<usize>>();
        self.groups == res
    }

    fn substitute(self, entry: Entry) -> Self {
        let mut entries = self.entries;
        for e in entries.iter_mut() {
            if *e == Entry::Unknown {
                *e = entry;
                break;
            }
        }

        Self::new(entries, self.groups)
    }
}

fn solve(n: usize, record: Record) -> usize {
    if !record.entries.contains(&Entry::Unknown) {
        if record.validate() {
            return n + 1;
        } else {
            return n;
        }
    }
    solve(n, record.clone().substitute(Entry::Operational))
        + solve(n, record.substitute(Entry::Damaged))
}

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let res = reader
        .lines()
        .filter_map(|line| line.ok())
        .map(|line| Record::parse(&line[..]))
        .map(|r| solve(0, r))
        .sum::<usize>();

    println!("{}", res);

    Ok(())
}
