// https://adventofcode.com/2022/day/6

use std::collections::HashSet;

fn main() {
    include_str!("../input.txt")
        .lines()
        .into_iter()
        .for_each(|x| {
            for i in 0..x.len() {
                let chars: HashSet<char> = x[i..i + 4].chars().collect();
                if chars.len() == 4 {
                    println!("Index = {}", i + 4);
                    return;
                }
            }
        }
        );
}