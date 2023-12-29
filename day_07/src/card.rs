use std::fmt;

#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Card(u8);

impl Card {
    pub fn parse(c: char) -> Option<Card> {
        let value = match c {
            'A' => 14,
            'K' => 13,
            'Q' => 12,
            'J' => 11,
            'T' => 10,
            '2'..='9' => (c as u8) - b'0',
            '?' => 1,
            _ => return None,
        };
        Some(Card(value))
    }

    pub fn as_char(&self) -> char {
        match self.0 {
            14 => 'A',
            13 => 'K',
            12 => 'Q',
            11 => 'J',
            10 => 'T',
            2..=9 => (self.0 + b'0') as char,
            1 => '?',
            _ => panic!("Unexpected Card value!"),
        }
    }

    pub fn all_possible_cards() -> impl Iterator<Item = Card> {
        (2..=14).map(Card)
    }
}

pub const JOKER: Card = Card(11);
pub const WILDCARD: Card = Card(1);

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_char())
    }
}

impl fmt::Debug for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Card({})", self)
    }
}
