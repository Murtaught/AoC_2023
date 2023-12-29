use std::collections::HashMap;

use crate::{
    multipart::{MultiPart, NumericSet},
    part::{Part, Property},
};
use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Operator {
    Lt,
    Gt,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Check {
    prop: Property,
    op: Operator,
    value: i64,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Verdict {
    Accept,
    Reject,
    GoTo(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Rule {
    pub check: Option<Check>,
    pub verdict: Verdict,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Workflow {
    pub name: String,
    pub rules: Vec<Rule>,
}

impl Operator {
    pub fn parse(s: &str) -> Option<Operator> {
        match s {
            "<" => Some(Operator::Lt),
            ">" => Some(Operator::Gt),
            _ => None,
        }
    }
}

lazy_static! {
    static ref CHECK_RE: Regex = Regex::new(r"^([xmas])(<|>)(-?\d+)$").unwrap();
}

impl Check {
    pub fn parse(s: &str) -> Option<Check> {
        let caps = CHECK_RE.captures(s)?;
        let prop = Property::parse(&caps[1])?;
        let op = Operator::parse(&caps[2])?;
        let value = caps[3].parse().ok()?;
        Some(Check { prop, op, value })
    }

    pub fn passes(&self, part: &Part) -> bool {
        self.test_value(*part.get(self.prop))
    }

    pub fn passes_mp(&self, mut mp: MultiPart) -> (MultiPart, MultiPart) {
        let mut passes = mp.clone();

        let positive_set = passes.get_mut(self.prop);
        positive_set.clear();

        let mut negative_set = NumericSet::new();

        for &i in mp.get(self.prop) {
            if self.test_value(i as i64) {
                positive_set.insert(i);
            }
            else {
                negative_set.insert(i);
            }
        }

        *mp.get_mut(self.prop) = negative_set;
        (mp, passes)
    }

    fn test_value(&self, part_value: i64) -> bool {
        match self.op {
            Operator::Lt => part_value < self.value,
            Operator::Gt => part_value > self.value,
        }
    }
}

impl Verdict {
    pub fn parse(s: &str) -> Verdict {
        match s {
            "R" => Verdict::Reject,
            "A" => Verdict::Accept,
            _ => Verdict::GoTo(s.to_string()),
        }
    }
}

impl Rule {
    pub fn parse(s: &str) -> Option<Rule> {
        if s.contains(':') {
            let ps: Vec<_> = s.split(':').collect();
            if ps.len() != 2 {
                return None;
            }

            let check = Check::parse(ps[0])?;
            let verdict = Verdict::parse(ps[1]);
            Some(Rule {
                check: Some(check),
                verdict,
            })
        } else {
            let verdict = Verdict::parse(s);
            Some(Rule {
                check: None,
                verdict,
            })
        }
    }

    pub fn apply(&self, part: &Part) -> Option<Verdict> {
        if let Some(check) = &self.check {
            return if check.passes(part) {
                Some(self.verdict.clone())
            } else {
                None
            };
        }
        Some(self.verdict.clone())
    }

    pub fn apply_mp(&self, mp: MultiPart) -> HashMap<Option<Verdict>, MultiPart> {
        let mut res = HashMap::new();
        if let Some(check) = &self.check {
            let (fails, passes) = check.passes_mp(mp);

            {
                let prop = check.prop;
                let mut it = passes.get(prop).intersection(fails.get(prop));
                assert_eq!(it.next(), None)
            }

            res.insert(None, fails);
            res.insert(Some(self.verdict.clone()), passes);
        }
        else {
            res.insert(Some(self.verdict.clone()), mp);
        }
        res
    }
}

lazy_static! {
    static ref WORKFLOW_RE: Regex = Regex::new(r"^(\w+)\{(.*)\}$").unwrap();
}

impl Workflow {
    pub fn parse(s: &str) -> Option<Workflow> {
        let caps = WORKFLOW_RE.captures(s)?;
        let name = caps[1].to_string();
        let rules_str = &caps[2];
        let rules: Vec<Rule> = rules_str
            .split(',')
            .map(Rule::parse)
            .collect::<Option<_>>()?;

        assert!(!rules.is_empty());
        assert!(rules.last().unwrap().check.is_none());

        Some(Workflow { name, rules })
    }

    pub fn process(&self, part: &Part) -> Verdict {
        for rule in &self.rules {
            if let Some(verdict) = rule.apply(part) {
                return verdict;
            }
        }
        unreachable!()
    }

    pub fn process_mp(&self, starting_mp: MultiPart) -> Vec<(Verdict, MultiPart)> {
        let mut undecided_mp = starting_mp;
        let mut res = Vec::new();

        for rule in &self.rules {
            if undecided_mp.is_empty() {
                break;
            }

            for (maybe_verdict, mp) in rule.apply_mp(std::mem::take(&mut undecided_mp)) {
                if let Some(verdict) = maybe_verdict {
                    res.push((verdict, mp));
                } else {
                    undecided_mp = mp;
                }
            }
        }

        assert!(undecided_mp.x.is_empty());
        assert!(undecided_mp.m.is_empty());
        assert!(undecided_mp.a.is_empty());
        assert!(undecided_mp.s.is_empty());

        res
    }
}
