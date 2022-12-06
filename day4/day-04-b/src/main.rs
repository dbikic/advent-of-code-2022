// https://adventofcode.com/2022/day/4

struct Assignment {
    start: i32,
    end: i32,
}

impl Assignment {
    pub fn new(line: &str) -> Self {
        let pair = line.split_at(line.chars().position(|x| x == '-').unwrap());
        Assignment {
            start: pair.0.parse::<i32>().unwrap(),
            end: pair.1.strip_prefix('-').unwrap().parse::<i32>().unwrap(),
        }
    }
    pub(crate) fn contains(&self, other: &Assignment) -> bool {
        let range = self.start..self.end + 1;
        if range.contains(&other.start) || range.contains(&other.end) {
            return true;
        }
        false
    }
}

struct Row {
    assignment1: Assignment,
    assignment2: Assignment,
}

impl Row {
    pub fn new(line: &str) -> Self {
        let pair = line.split_at(line.chars().position(|x| x == ',').unwrap());
        Row {
            assignment1: Assignment::new(pair.0),
            assignment2: Assignment::new(pair.1.strip_prefix(",").unwrap()),
        }
    }
    pub(crate) fn has_overlap(&self) -> bool {
        return if self.assignment1.contains(&self.assignment2) {
            true
        } else if self.assignment2.contains(&self.assignment1) {
            true
        } else {
            false
        };
    }
}

fn main() {
    let mut score = 0;
    include_str!("../input.txt")
        .lines()
        .for_each(|x| {
            let row = Row::new(x);
            if row.has_overlap() {
                score += 1;
            }
        }
        );
    println!("{}", score);
}