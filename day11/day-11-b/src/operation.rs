use std::fmt;

use crate::operation::Operation::{Multiply, Plus, Square};

#[derive(Clone, Copy)]
pub enum Operation {
    Plus(u64),
    Multiply(u64),
    Square,
}

impl Operation {
    pub fn new(input: &str) -> Self {
        let split: Vec<&str> = input.split(" ").collect();
        match split.first().unwrap() {
            &"+" => Plus(split.last().unwrap().parse().unwrap()),
            _ => if split.last().unwrap() == &"old" {
                Square
            } else {
                Multiply(split.last().unwrap().parse().unwrap())
            },
        }
    }
}

impl fmt::Display for Operation {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Plus(i) => write!(f, " + {}", i).unwrap(),
            Multiply(i) => write!(f, " * {}", i).unwrap(),
            Square => write!(f, " Square").unwrap(),
        }
        Ok(())
    }
}