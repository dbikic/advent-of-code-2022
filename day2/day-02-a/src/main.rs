// https://adventofcode.com/2022/day/2

const ROCK_VALUE: i32 = 1;
const PAPER_VALUE: i32 = 2;
const SCISSOR_VALUE: i32 = 3;
const WIN_SCORE: i32 = 6;
const DRAW_SCORE: i32 = 3;

struct Round {
    // A = rock ; B = paper ; C = scissors
    opponent_choice: char,
    // X = rock ; Y = paper ; Z = scissors
    my_choice: char,
}

impl Round {
    pub fn new(line: &str) -> Self {
        Round {
            opponent_choice: line.chars().nth(0).unwrap(),
            my_choice: line.chars().nth(2).unwrap(),
        }
    }
    pub(crate) fn calculate_score(&self) -> i32 {
        if self.my_choice == 'X' {
            ROCK_VALUE + match self.opponent_choice {
                'A' => DRAW_SCORE,
                'B' => 0,
                _ => WIN_SCORE,
            }
        } else if self.my_choice == 'Y' {
            PAPER_VALUE + match self.opponent_choice {
                'A' => WIN_SCORE,
                'B' => DRAW_SCORE,
                _ => 0,
            }
        } else {
            SCISSOR_VALUE + match self.opponent_choice {
                'A' => 0,
                'B' => WIN_SCORE,
                _ => DRAW_SCORE,
            }
        }
    }
}

fn main() {
    let mut score = 0;
    include_str!("../input.txt")
        .lines()
        .for_each(|x| {
            let round = Round::new(x);
            score += round.calculate_score();
        }
        );
    println!("{}", score);
}
