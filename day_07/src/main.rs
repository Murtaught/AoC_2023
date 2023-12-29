use hand::Hand;
use std::fs;

mod card;
mod hand;

fn main() {
    let content = fs::read_to_string("input").unwrap();
    let hands: Vec<Hand> = content
        .lines()
        .map(Hand::parse)
        .collect::<Option<_>>()
        .unwrap();

    println!("ans (p1): {}", solve_p1(hands.clone()));
    println!("ans (p2): {}", solve_p2(hands));
}

fn solve_p1(mut hands: Vec<Hand>) -> u64 {
    hands.sort();
    hands
        .iter()
        .zip(1..)
        .map(|(hand, rank)| rank * hand.bid())
        .sum()
}

fn solve_p2(mut hands: Vec<Hand>) -> u64 {
    for hand in &mut hands {
        hand.transform_jokers_to_wildcards();
    }
    solve_p1(hands)
}
