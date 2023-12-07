use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::ops::AddAssign;
use std::str::FromStr;

use itertools::Itertools;

#[derive(Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
enum Card {
    Joker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Queen,
    King,
    Ace,
}

#[derive(Debug)]
struct ParseCardError;

impl FromStr for Card {
    type Err = ParseCardError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "2" => Ok(Card::Two),
            "3" => Ok(Card::Three),
            "4" => Ok(Card::Four),
            "5" => Ok(Card::Five),
            "6" => Ok(Card::Six),
            "7" => Ok(Card::Seven),
            "8" => Ok(Card::Eight),
            "9" => Ok(Card::Nine),
            "T" => Ok(Card::Ten),
            "J" => Ok(Card::Joker),
            "Q" => Ok(Card::Queen),
            "K" => Ok(Card::King),
            "A" => Ok(Card::Ace),
            _ => Err(ParseCardError),
        }
    }
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug)]
enum Strength {
    HighCard,
    Pair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug)]
struct Hand {
    cards: Vec<Card>,
    strength: Strength,
    bid: u32,
}

fn evaluate_strength(cards: &Vec<Card>) -> Strength {
    let mut unique_cards = cards.iter().fold(HashMap::new(), |mut acc, card| {
        *acc.entry(card).or_insert(0) += 1;
        acc
    });

    if let Some(joker) = unique_cards.remove(&Card::Joker) {
        if let Some((k, _)) = unique_cards
            .iter()
            .max_by(|(_, lhs), (_, rhs)| lhs.cmp(rhs))
        {
            unique_cards.entry(k).and_modify(|v| v.add_assign(&joker));
        } else {
            unique_cards.insert(&Card::Joker, 5);
        }
    }

    match unique_cards.len() {
        1 => Strength::FiveOfAKind,
        2 => match unique_cards.iter().map(|(_, count)| count).product::<u32>() {
            4 => Strength::FourOfAKind,
            6 => Strength::FullHouse,
            _ => panic!(),
        },
        3 => match unique_cards.iter().map(|(_, count)| count).product::<u32>() {
            3 => Strength::ThreeOfAKind,
            4 => Strength::TwoPair,
            _ => panic!(),
        },
        4 => Strength::Pair,
        5 => Strength::HighCard,
        _ => panic!(),
    }
}

fn parse_hand(hand: &str) -> Hand {
    let mut hand = hand.split_whitespace();
    let cards = hand
        .next()
        .unwrap()
        .chars()
        .map(|ch| Card::from_str(&ch.to_string()[..]).unwrap())
        .collect();
    let strength = evaluate_strength(&cards);
    let bid = hand.next().unwrap().parse::<u32>().unwrap();
    Hand {
        cards,
        strength,
        bid,
    }
}

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let res = reader
        .lines()
        .filter_map(|line| line.ok())
        .map(|line| parse_hand(&line[..]))
        .sorted_by(|lhs, rhs| match lhs.strength.cmp(&rhs.strength) {
            Ordering::Less => Ordering::Less,
            Ordering::Greater => Ordering::Greater,
            Ordering::Equal => lhs.cards.cmp(&rhs.cards),
        })
        .enumerate()
        // .for_each(|hand| {
        // println!("{:?}", hand)
        // });
        .fold(0, |acc, (i, hand)| acc + (i as u32 + 1) * hand.bid);

    println!("{}", res);

    Ok(())
}
