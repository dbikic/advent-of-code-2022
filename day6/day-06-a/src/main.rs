// https://adventofcode.com/2022/day/6

use std::collections::HashSet;

fn main() {
    include_str!("../input.txt")
        .lines()
        .into_iter()
        .for_each(|x| {
            let chars: Vec<char> = x.chars().collect();
            for i in 0..chars.len() {
                for j in 3..chars.len() {
                    if j < chars.len() {
                        if are_different(&chars[i], &chars[i + 1], &chars[i + 2], &chars[i + 3]) {
                            println!("Index = {}", i + 4);
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

fn are_different(c0: &char, c1: &char, c2: &char, c3: &char) -> bool {
    let mut set: HashSet<&char> = HashSet::new();
    set.insert(c0);
    set.insert(c1);
    set.insert(c2);
    set.insert(c3);
    set.len() == 4
}