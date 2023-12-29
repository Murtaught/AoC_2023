use std::collections::HashSet;

use crate::part::Property;

pub type NumericSet = HashSet<u16>;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct MultiPart {
    pub x: NumericSet,
    pub m: NumericSet,
    pub a: NumericSet,
    pub s: NumericSet,
}

impl MultiPart {
    pub fn full() -> Self {
        let mut set = HashSet::<u16>::new();
        for i in 1..=4000 {
            set.insert(i);
        }

        let x = set.clone();
        let m = set.clone();
        let a = set.clone();
        let s = set;

        Self { x, m, a, s }
    }

    #[allow(dead_code)]
    pub fn get(&self, prop: Property) -> &NumericSet {
        match prop {
            Property::X => &self.x,
            Property::M => &self.m,
            Property::A => &self.a,
            Property::S => &self.s,
        }
    }

    pub fn get_mut(&mut self, prop: Property) -> &mut NumericSet {
        match prop {
            Property::X => &mut self.x,
            Property::M => &mut self.m,
            Property::A => &mut self.a,
            Property::S => &mut self.s,
        }
    }

    pub fn is_empty(&self) -> bool {
        [&self.x, &self.m, &self.a, &self.s]
            .into_iter()
            .any(HashSet::is_empty)
    }

    pub fn power(&self) -> u64 {
        [&self.x, &self.m, &self.a, &self.s]
            .into_iter()
            .map(|x| x.len() as u64)
            .product()
    }
}
