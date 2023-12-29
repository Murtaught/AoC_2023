use std::{fmt, collections::HashMap};
use std::cmp::Ordering;
use crate::card::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Hand {
    cards: [Card; 5],
    tp: Type,
    bid: u64
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Type {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl Hand {
    pub fn parse(s: &str) -> Option<Hand> {
        let mut it = s.split_whitespace();
        let s = it.next()?;
        let bid: u64 = it.next()?.parse().ok()?;
        if it.next().is_some() {
            return None;
        }

        if s.len() != 5 {
            return None;
        }

        let cards: [Card; 5] = s
            .chars()
            .map(Card::parse)
            .collect::<Option<Vec<Card>>>()?
            .try_into()
            .ok()?;

        let tp = Self::determine_type(cards);

        Some(Hand { cards, tp, bid })
    }

    fn determine_type(cards: [Card; 5]) -> Type {
        let mut count = HashMap::<Card, usize>::new();
        for card in cards {
            *count.entry(card).or_default() += 1;
        }
        
        if count.len() == 1 {
            return Type::FiveOfAKind;
        }
        
        if count.values().any(|&v| v == 4) {
            assert_eq!(count.len(), 2);
            return Type::FourOfAKind;
        }

        if count.values().any(|&v| v == 3) {
            if count.len() == 2 {
                return Type::FullHouse;
            }
            else {
                assert_eq!(count.len(), 3);
                return Type::ThreeOfAKind;
            }
        }

        let mut pairs_count = 0;
        for &v in count.values() {
            if v >= 2 {
                pairs_count += 1;
            }
        }

        match pairs_count {
            2 => Type::TwoPair,
            1 => Type::OnePair,
            0 => Type::HighCard,
            _ => panic!("Unexpected `pairs_count` value!")
        }
    }

    pub fn transform_jokers_to_wildcards(&mut self) {
        for card in &mut self.cards {
            if *card == JOKER {
                *card = WILDCARD;
            }
        }

        self.tp = Self::determine_type_with_wildcards(self.cards);
    }

    fn determine_type_with_wildcards(cards: [Card; 5]) -> Type {
        fn rec_helper(mut template: [Card; 5], start: usize) -> Type {
            if start >= template.len() {
                return Hand::determine_type(template);
            }

            if template[start] != WILDCARD {
                return rec_helper(template, start + 1);
            }

            Card::all_possible_cards()
                .map(|replacement| {
                    template[start] = replacement;
                    rec_helper(template, start + 1)
                })
                .max()
                .unwrap()
        }

        rec_helper(cards, 0)
    }

    pub fn bid(&self) -> u64 {
        self.bid
    }
}

impl fmt::Display for Hand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Hand(")?;
        for card in &self.cards {
            write!(f, "{}", card.as_char())?;
        }
        write!(f, ", {})", self.bid)
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.tp.cmp(&other.tp) {
            Ordering::Equal => {}
            ord => return ord,
        }
        match self.cards.cmp(&other.cards) {
            Ordering::Equal => {}
            ord => return ord,
        }
        self.bid.cmp(&other.bid)
    }
}
