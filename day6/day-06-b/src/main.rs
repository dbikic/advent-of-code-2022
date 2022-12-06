// https://adventofcode.com/2022/day/6

use std::collections::HashSet;

fn main() {
    include_str!("../input.txt")
        .lines()
        .into_iter()
        .for_each(|x| {
            let chars: Vec<char> = x.chars().collect();
            for i in 0..chars.len() {
                for j in 13..chars.len() {
                    if j < chars.len() {
                        if are_different(&chars[i..i + 14]) {
                            println!("Index = {}", i + 14);
                            return;
                        }
                    } else {
                        return;
                    }
                }
            }
        }
        );
}

fn are_different(vec: &[char]) -> bool {
    let mut set: HashSet<char> = HashSet::new();
    for x in vec {
        set.insert(x.clone());
    }
    vec.len() == set.len()
}