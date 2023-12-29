#![allow(dead_code)]

use std::collections::HashSet;

use brick::Brick;

use crate::pos::Coord;

mod brick;
mod pos;

type LiesOn = Vec<Vec<usize>>;

fn main() {
    let content = std::fs::read_to_string("input").unwrap();
    let bricks: Vec<Brick> = content
        .lines()
        .map(|line| Brick::parse(line).unwrap())
        .collect();

    let stable = stabilize(bricks);
    let lies_on = build_graph(&stable);
    println!("ans (p1): {}", solve_p1(&lies_on));
    println!("ans (p2): {}", solve_p2(&lies_on));
}

fn stabilize(mut bricks: Vec<Brick>) -> Vec<Brick> {
    bricks.sort_by_key(|b| b.tl.z);
    let mut stable = Vec::new();
    for mut brick in bricks {
        assert!(!stable.iter().any(|sb| brick.intersects(sb)));
        while !stable.iter().any(|sb| brick.intersects(sb)) {
            brick.step(Coord::Z, -1);
            if brick.tl.z < 0 {
                break;
            }
        }
        brick.step(Coord::Z, 1);
        stable.push(brick);
    }
    stable
}

fn build_graph(stable: &[Brick]) -> LiesOn {
    let mut res = LiesOn::new();
    for i in 0..stable.len() {
        let mut fnd = Vec::<usize>::new();
        let b = stable[i].stepped(Coord::Z, -1);
        for (j, sb) in stable.iter().enumerate() {
            if j >= i {
                break;
            }
            if b.intersects(sb) {
                fnd.push(j);
            }
        }
        res.push(fnd);
    }
    res
}

fn solve_p1(lies_on: &LiesOn) -> usize {
    let mut ans_1 = 0_usize;
    for i in 0..lies_on.len() {
        // Is there at least one Brick that lies EXCLUSIVELY
        // on the current Brick `i`? If there is one, it's NOT safe to
        // disintegrate it.
        if !lies_on.iter().any(|fnd| fnd == &vec![i]) {
            ans_1 += 1;
        }
    }
    ans_1
}

type Cache = Vec<Option<usize>>;

fn solve_p2(lies_on: &LiesOn) -> usize {
    let mut ans = 0;

    for k in 0..lies_on.len() {
        let mut dis = HashSet::<usize>::new();
        dis.insert(k);

        for (i, fnd) in lies_on.iter().enumerate() {
            if i > k && !fnd.is_empty() && fnd.iter().all(|j| dis.contains(j)) {
                dis.insert(i);
            }
        }

        ans += dis.len() - 1;
    }

    ans
}
