use crate::{
    multipart::MultiPart,
    part::Part,
    workflow::{Verdict, Workflow},
};
use std::{collections::HashMap, fs};

mod multipart;
mod part;
mod workflow;

fn main() {
    let content = fs::read_to_string("input").unwrap();

    let mut seen_empty_line = false;
    let mut parts = Vec::<Part>::new();
    let mut workflows = HashMap::<String, Workflow>::new();

    for line in content.lines() {
        if seen_empty_line {
            parts.push(Part::parse(line).unwrap());
        } else if line.is_empty() {
            seen_empty_line = true;
        } else {
            let workflow = Workflow::parse(line).unwrap();
            workflows.insert(workflow.name.clone(), workflow);
        }
    }

    println!("ans (p1): {}", solve_p1(&parts, &workflows));
    println!(
        "ans (p2): {}",
        solve_p2(MultiPart::full(), "in", &workflows)
    );
}

fn solve_p1(parts: &[Part], workflows: &HashMap<String, Workflow>) -> i64 {
    let mut ans = 0_i64;

    for part in parts {
        let mut verdict = Verdict::GoTo("in".to_string());
        while let Verdict::GoTo(wf_name) = verdict {
            let wf = workflows.get(&wf_name).unwrap();
            verdict = wf.process(part);
        }

        if verdict == Verdict::Accept {
            ans += part.sum();
        }
    }

    ans
}

fn solve_p2(mp: MultiPart, wf_name: &str, workflows: &HashMap<String, Workflow>) -> u64 {
    let mut res = 0;
    let wf = workflows.get(wf_name).unwrap();
    for (verdict, next) in wf.process_mp(mp) {
        match verdict {
            Verdict::Accept => {
                // eprintln!(
                // "> {{{}, {}, {}, {}}}: +{}",
                // next.x.len(),
                // next.m.len(),
                // next.a.len(),
                // next.s.len(),
                // next.power()
                // );
                res += next.power();
            }
            Verdict::Reject => {}
            Verdict::GoTo(next_name) => {
                res += solve_p2(next, &next_name, workflows);
            }
        }
    }
    res
}
