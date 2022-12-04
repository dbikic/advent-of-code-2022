use std::collections::HashSet;

fn main() {
    let mut score = 0;
    let mut index = 0;
    let mut set1: HashSet<char> = HashSet::new();
    let mut set2: HashSet<char> = HashSet::new();
    let mut set3: HashSet<char> = HashSet::new();
    include_str!("../input.txt")
        .lines()
        .for_each(|x| {
            if index == 2 {
                insert_values(&mut set3, x);
                score += calculate_score(&set1, &set2, &set3);
                set1.clear();
                set2.clear();
                set3.clear();
                index = 0;
            } else if index == 1 {
                insert_values(&mut set2, x);
                index += 1;
            } else {
                insert_values(&mut set1, x);
                index += 1;
            }
        }
        );
    println!("{}", score);
}

fn insert_values(set: &mut HashSet<char>, line: &str) {
    for x in line.chars() {
        set.insert(x);
    }
}

fn calculate_score(set1: &HashSet<char>, set2: &HashSet<char>, set3: &HashSet<char>) -> i32 {
    let intersection = set1.intersection(set2);
    let mut intersection_set: HashSet<char> = HashSet::new();
    intersection.for_each(|x| {
        intersection_set.insert(x.clone());
    });
    get_char_score(set3.intersection(&intersection_set).min().unwrap())
}

fn get_char_score(char: &char) -> i32 {
    if char.is_uppercase() {
        char.clone() as i32 - 38
    } else {
        char.clone() as i32 - 96
    }
}
