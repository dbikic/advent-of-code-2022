// https://adventofcode.com/2022/day/3

use std::collections::HashSet;

fn main() {
    let mut score = 0;
    include_str!("../input.txt")
        .lines()
        .for_each(|x| {
            score += calculate_score(x);
        }
        );
    println!("{}", score);
}

fn calculate_score(line: &str) -> i32 {
    let mut set: HashSet<char> = HashSet::new();
    let middle = line.len() / 2;
    for i in 0..middle {
        let char = line.chars().nth(i).unwrap();
        set.insert(char);
    }
    for i in middle..line.len() {
        let char = line.chars().nth(i).unwrap();
        if set.contains(&char) {
            return get_char_score(&char);
        }
    }
    0
}

fn get_char_score(char: &char) -> i32 {
    if char.is_uppercase() {
        char.clone() as i32 - 38
    } else {
        char.clone() as i32 - 96
    }
}
