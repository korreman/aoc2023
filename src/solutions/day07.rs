use std::{collections::HashMap, marker::PhantomData};

use itertools::Itertools;

pub fn run(input: &str) -> (u32, u32) {
    let mut deals1 = Vec::new();
    let mut deals2 = Vec::new();
    for (hand, bid) in input.split_ascii_whitespace().tuples() {
        let hand1 = Hand::<One>::parse(hand.as_bytes().try_into().unwrap(), b"23456789TJQKA");
        let hand2 = Hand::<Two>::parse(hand.as_bytes().try_into().unwrap(), b"J23456789TQKA");
        let bid = bid.parse::<u32>().unwrap();
        deals1.push((hand1, bid));
        deals2.push((hand2, bid));
    }

    deals1.sort_by_key(|x| x.0.clone());
    let part1 = deals1
        .iter()
        .enumerate()
        .map(|(i, x)| (i + 1) as u32 * x.1)
        .sum();

    deals2.sort_by_key(|x| x.0.clone());
    let part2 = deals2
        .iter()
        .enumerate()
        .map(|(i, x)| (i + 1) as u32 * x.1)
        .sum();

    (part1, part2)
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
struct Card(u8);

impl Card {
    fn parse(c: u8, ranking: &[u8]) -> Option<Self> {
        Some(Self(ranking.iter().position(|&b| b == c)? as u8))
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum HandKind {
    High,
    Pair,
    TwoPair,
    Three,
    House,
    Four,
    Five,
}

#[derive(PartialEq, Eq, Clone)]
struct Hand<P>(PhantomData<P>, [Card; 5]);

#[derive(PartialEq, Eq, Clone, Copy)]
struct One;

#[derive(PartialEq, Eq, Clone, Copy)]
struct Two;

impl<P> Hand<P> {
    fn parse(cards: &[u8; 5], ranking: &[u8]) -> Self {
        Self(
            PhantomData,
            cards.map(|c| Card::parse(c, ranking).unwrap()),
        )
    }

    fn kind1(&self) -> HandKind {
        let mut counts = HashMap::new();
        for card in &self.1 {
            *counts.entry(card).or_default() += 1;
        }
        let mut kind = HandKind::High;
        for count in counts.values() {
            match count {
                1 => {}
                2 => match kind {
                    HandKind::High => kind = HandKind::Pair,
                    HandKind::Pair => kind = HandKind::TwoPair,
                    HandKind::Three => kind = HandKind::House,
                    _ => panic!(),
                },
                3 => {
                    if kind == HandKind::Pair {
                        kind = HandKind::House;
                    } else {
                        kind = HandKind::Three;
                    }
                }
                4 => kind = HandKind::Four,
                5 => kind = HandKind::Five,
                _ => panic!(),
            }
        }
        kind
    }

    fn kind2(&self) -> HandKind {
        let mut counts = HashMap::new();
        let mut jokers = 0;
        for card in &self.1 {
            if card.0 == 0 {
                jokers += 1;
            } else {
                *counts.entry(card).or_default() += 1;
            }
        }
        if jokers == 5 {
            return HandKind::Five;
        }
        let mut kind = HandKind::High;
        let mut counts: Vec<u32> = counts.values().cloned().collect();
        counts.sort();
        for count in counts.iter().rev() {
            match *count + jokers {
                1 => {}
                2 => match kind {
                    HandKind::High => kind = HandKind::Pair,
                    HandKind::Pair => kind = HandKind::TwoPair,
                    HandKind::Three => kind = HandKind::House,
                    _ => {}
                },
                3 => {
                    if kind == HandKind::Pair {
                        kind = HandKind::House;
                    } else {
                        kind = HandKind::Three;
                    }
                }
                4 => kind = HandKind::Four,
                5 => kind = HandKind::Five,
                _ => panic!(),
            }
            jokers = 0;
        }
        kind
    }
}

impl PartialOrd for Hand<One> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand<One> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (self.kind1(), self.1).cmp(&(other.kind1(), other.1))
    }
}

impl PartialOrd for Hand<Two> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand<Two> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (self.kind2(), self.1).cmp(&(other.kind2(), other.1))
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn sample() {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
        assert_eq!(super::run(input), (6440, 5905));
    }
}
