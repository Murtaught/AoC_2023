#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Property {
    X,
    M,
    A,
    S,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Part {
    pub x: i64,
    pub m: i64,
    pub a: i64,
    pub s: i64,
}

impl Property {
    pub fn parse(s: &str) -> Option<Property> {
        let c = s.chars().next()?;
        match c.to_ascii_lowercase() {
            'x' => Some(Property::X),
            'm' => Some(Property::M),
            'a' => Some(Property::A),
            's' => Some(Property::S),
            _ => None,
        }
    }
}

impl Part {
    pub fn parse(s: &str) -> Option<Part> {
        if !s.starts_with('{') || !s.ends_with('}') {
            return None;
        }

        let mut part = Part::default();
        let s = &s[1..(s.len() - 1)];
        for vs in s.split(',') {
            assert_eq!(&vs[1..2], "=");
            let prop = Property::parse(&vs[0..1])?;
            let value: i64 = vs[2..].parse().ok()?;
            *part.get_mut(prop) = value;
        }

        Some(part)
    }

    pub fn get(&self, prop: Property) -> &i64 {
        match prop {
            Property::X => &self.x,
            Property::M => &self.m,
            Property::A => &self.a,
            Property::S => &self.s,
        }
    }

    pub fn get_mut(&mut self, prop: Property) -> &mut i64 {
        match prop {
            Property::X => &mut self.x,
            Property::M => &mut self.m,
            Property::A => &mut self.a,
            Property::S => &mut self.s,
        }
    }

    pub fn sum(&self) -> i64 {
        self.x + self.m + self.a + self.s
    }
}
