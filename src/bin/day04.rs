use std::{os::unix::prelude::MetadataExt, str::FromStr, string::ParseError};

#[derive(Debug)]
struct Section {
    start: u32,
    end: u32,
}

impl FromStr for Section {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let pair: Vec<&str> = s.split('-').collect();
        Ok(Section {
            start: pair[0].parse().unwrap(),
            end: pair[1].parse().unwrap(),
        })
    }
}

#[derive(Debug)]
struct Pair {
    first: Section,
    second: Section,
}

impl Pair {
    fn fully_contained(&self) -> bool {
        let (earlier, later) = if self.first.start < self.second.start {
            (&self.first, &self.second)
        } else {
            if self.first.start == self.second.start {
                if self.first.end - self.first.start > self.second.end - self.second.start {
                    (&self.first, &self.second)
                } else {
                    (&self.second, &self.first)
                }
            } else {
                (&self.second, &self.first)
            }
        };

        if earlier.end >= later.end {
            return true;
        }
        false
    }

    fn overlap(&self) -> bool {
        let (earlier, later) = if self.first.start < self.second.start {
            (&self.first, &self.second)
        } else {
            if self.first.start == self.second.start {
                if self.first.end - self.first.start > self.second.end - self.second.start {
                    (&self.first, &self.second)
                } else {
                    (&self.second, &self.first)
                }
            } else {
                (&self.second, &self.first)
            }
        };

        if earlier.end < later.start {
            return false;
        }
        true
    }
}

fn main() {
    let input = include_str!("./input.prod");
    let pairs = input.lines().map(|line| {
        if let Some((a, b)) = line.split_once(',') {
            Pair {
                first: Section::from_str(a).unwrap(),
                second: Section::from_str(b).unwrap(),
            }
        } else {
            panic!()
        }
    });

    for pair in pairs.clone() {
        println!("{:?}: {}", pair, pair.fully_contained());
    }
    let num = pairs.clone().filter(|pair| pair.fully_contained()).count();
    println!("{}", num);

    for pair in pairs.clone() {
        println!("{:?}: {}", pair, pair.overlap());
    }
    let num = pairs.filter(|pair| pair.overlap()).count();
    println!("{}", num)
}
