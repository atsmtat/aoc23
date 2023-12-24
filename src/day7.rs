use std::cmp::Ord;
use std::str::FromStr;
use std::{cmp::Ordering, collections::HashMap};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Card {
    C1,
    C2,
    C3,
    C4,
    C5,
    C6,
    C7,
    C8,
    C9,
    T,
    J,
    Q,
    K,
    A,
}

#[derive(Debug)]
enum HandParseErr {
    Unexpected(char),
    Eof,
}

impl FromStr for Card {
    type Err = HandParseErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.chars().next() {
            None => Err(HandParseErr::Eof),
            Some(c) => {
                let card = match c {
                    '1' => Card::C1,
                    '2' => Card::C2,
                    '3' => Card::C3,
                    '4' => Card::C4,
                    '5' => Card::C5,
                    '6' => Card::C6,
                    '7' => Card::C7,
                    '8' => Card::C8,
                    '9' => Card::C9,
                    'T' => Card::T,
                    'J' => Card::J,
                    'Q' => Card::Q,
                    'K' => Card::K,
                    'A' => Card::A,
                    _ => {
                        return Err(HandParseErr::Unexpected(c));
                    }
                };
                Ok(card)
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Kind {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOf,
    FullHouse,
    FourOf,
    FiveOf,
}

#[derive(Debug)]
struct Hand {
    cards: Vec<Card>,
}

impl Hand {
    pub fn kind(&self) -> Kind {
        let mut card_freq = HashMap::new();

        for card in &self.cards {
            card_freq.entry(card).and_modify(|e| *e += 1).or_insert(1);
        }

        let mut freq: Vec<_> = card_freq.values().collect();
        freq.sort();

        match freq.len() {
            1 => Kind::FiveOf,
            2 => match (freq[0], freq[1]) {
                (1, 4) => Kind::FourOf,
                (2, 3) => Kind::FullHouse,
                _ => unreachable!(),
            },
            3 => match (freq[0], freq[1], freq[2]) {
                (1, 1, 3) => Kind::ThreeOf,
                (1, 2, 2) => Kind::TwoPair,
                _ => unreachable!(),
            },
            4 => Kind::OnePair,
            5 => Kind::HighCard,
            _ => unreachable!(),
        }
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Hand) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Eq for Hand {}

impl Ord for Hand {
    fn cmp(&self, other: &Hand) -> Ordering {
        let kind_ord = self.kind().cmp(&other.kind());
        match kind_ord {
            Ordering::Equal => self.cards.cmp(&other.cards),
            _ => kind_ord,
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Hand) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl FromStr for Hand {
    type Err = HandParseErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut cards = vec![];
        for i in 0..s.len() {
            cards.push(s[i..].parse::<Card>()?);
        }
        Ok(Hand { cards })
    }
}

pub fn part_one(input: &str) -> usize {
    let mut hands: Vec<_> = input
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| {
            let (hand, bid) = l.split_once(' ').unwrap();
            let hand = hand.parse::<Hand>().unwrap();
            let bid = bid.parse::<usize>().unwrap();
            (hand, bid)
        })
        .collect();

    hands.sort_by(|lh, rh| lh.0.cmp(&rh.0));

    hands
        .iter()
        .enumerate()
        .fold(0, |sum, (i, h)| sum + (h.1) * (i + 1))
}

pub fn part_two(input: &str) -> u64 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::{part_one, part_two};

    #[test]
    fn test_puzzle_example() {
        let input = r"
32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
";
        assert_eq!(6440, part_one(input));
        //assert_eq!(71503, part_two(input));
    }
}
