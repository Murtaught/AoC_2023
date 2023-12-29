use crate::dir::Dir;
use lazy_static::lazy_static;
use regex::Regex;


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Instr {
    pub dir: Dir,
    pub steps: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParseMode {
    Normal,
    Color,
}

lazy_static! {
    static ref RE: Regex = Regex::new(r"^([URDL]) (\d+) \(#(\w{5})(\w)\)$").unwrap();
}

impl Instr {
    pub fn parse(s: &str, mode: ParseMode) -> Option<Instr> {
        let caps = RE.captures(s)?;
        let dir = match &caps[1] {
            "U" => Dir::U,
            "R" => Dir::R,
            "D" => Dir::D,
            "L" => Dir::L,
            _ => return None,
        };
        let steps: u64 = caps[2].parse().ok()?;
        
        Some(match mode {
            ParseMode::Normal => Instr { dir, steps },
            ParseMode::Color => {
                let dir = match &caps[4] {
                    "0" => Dir::R,
                    "1" => Dir::D,
                    "2" => Dir::L,
                    "3" => Dir::U,
                    _ => return None,
                };
                let steps = u64::from_str_radix(&caps[3], 16).ok()?;
                Instr { dir, steps }
            }
        })
    }
}
