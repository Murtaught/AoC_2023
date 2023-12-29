#![allow(dead_code)]
use std::{collections::HashMap, fs};

use crate::{graph::Graph, node::Id};

mod graph;
mod node;
mod signal;

fn main() {
    let content = fs::read_to_string("input").unwrap();
    let mut graph = Graph::parse(&content).unwrap();
    graph.add_node("button -> broadcaster");
    graph.prepare();

    graph.generate_dot("graph.dot").unwrap();

    println!("ans (p1): {}", solve_p1(graph.clone()));
    println!("ans (p2): {}", solve_p2(graph));
}

fn solve_p1(mut graph: Graph) -> usize {
    for _ in 0..1000 {
        graph.simulate();
    }

    graph.signals_fired.iter().cloned().product()
}

fn solve_p2(mut graph: Graph) -> usize {
    // Take a look at the graph with `dot -Tx11 graph.dot`.
    // It's not cheating, that's just how the input graph happens to be.
    const INTERESTING_NODES: &[&str] = &["vn", "ph", "kt", "hn"];

    let interesting_ids: Vec<Id> = INTERESTING_NODES
        .iter()
        .map(|&name| graph.map.get_id(name))
        .collect();

    let mut button_presses = 0_usize;
    let mut first_trigger = HashMap::<Id, usize>::new();

    while first_trigger.len() < interesting_ids.len() {
        button_presses += 1;
        graph
            .simulate()
            .into_iter()
            .filter(|id| interesting_ids.contains(id))
            .for_each(|id| {
                first_trigger.entry(id).or_insert(button_presses);
            });
    }

    // The interesting nodes just happen to be triggered once every this amount of button presses.
    // We need for all four to be triggered at the same time, so we compute the LCM.
    // In my case it is just product of all cycle length values.
    let cycle_lengths = first_trigger.values().copied();
    let g = cycle_lengths.clone().reduce(gcd).unwrap();
    assert_eq!(g, 1);

    cycle_lengths.product()
}

fn gcd(mut n: usize, mut m: usize) -> usize {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            std::mem::swap(&mut m, &mut n);
        }
        m %= n;
    }
    n
}
