// https://adventofcode.com/2022/day/9

use std::collections::HashSet;

enum Direction { U, D, R, L }

impl Direction {
    pub fn new(input: &str) -> Self {
        match input {
            "U" => Direction::U,
            "D" => Direction::D,
            "R" => Direction::R,
            "L" => Direction::L,
            _ => panic!("This won't happen"),
        }
    }
}

struct Step {
    direction: Direction,
    distance: i32,
}

impl Step {
    pub fn new(input: &str) -> Self {
        let split: Vec<&str> = input.split(" ").collect();
        Step {
            direction: Direction::new(split.first().unwrap()),
            distance: split.last().unwrap().parse::<i32>().unwrap(),
        }
    }
}

fn main() {
    let mut tail_positions: HashSet<(i32, i32)> = HashSet::new();
    let mut head: (i32, i32) = (1000, 1000);
    let mut tail: (i32, i32) = (1000, 1000);
    tail_positions.insert((1000, 1000));
    include_str!("../input.txt")
        .lines()
        .into_iter()
        .for_each(|x| {
            let step = Step::new(x);
            for _ in 0..step.distance {
                match &step.direction {
                    Direction::U => head.0 -= 1,
                    Direction::D => head.0 += 1,
                    Direction::L => head.1 -= 1,
                    Direction::R => head.1 += 1,
                }
                if !are_touching(&head, &tail) {
                    let tail_move = get_tail_movement(&head, &tail);
                    tail.0 += tail_move.0;
                    tail.1 += tail_move.1;
                }
                tail_positions.insert(tail);
            }
        });
    println!("{:?}", tail_positions.len());
}

fn are_touching(head: &(i32, i32), tail: &(i32, i32)) -> bool {
    (head.0 - tail.0).abs() <= 1 && (head.1 - tail.1).abs() <= 1
}

fn get_tail_movement(head: &(i32, i32), tail: &(i32, i32)) -> (i32, i32) {
    let mut row_diff = head.0 - tail.0;
    let mut col_diff = head.1 - tail.1;
    if row_diff < -1 {
        row_diff = -1
    } else if row_diff > 1 {
        row_diff = 1
    }
    if col_diff < -1 {
        col_diff = -1
    } else if col_diff > 1 {
        col_diff = 1
    }
    (row_diff, col_diff)
}
