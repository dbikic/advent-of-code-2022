// https://adventofcode.com/2022/day/2

const ROCK_VALUE: i32 = 1;
const PAPER_VALUE: i32 = 2;
const SCISSOR_VALUE: i32 = 3;
const DRAW_SCORE: i32 = 3;
const WIN_SCORE: i32 = 6;

struct Round {
    // A = rock ; B = paper ; C = scissors
    opponent_choice: char,
    // X = lose ; Y = draw ; Z = win
    outcome: char,
}

impl Round {
    pub fn new(line: &str) -> Self {
        Round {
            opponent_choice: line.chars().nth(0).unwrap(),
            outcome: line.chars().nth(2).unwrap(),
        }
    }
    pub(crate) fn calculate_score(&self) -> i32 {
        if self.outcome == 'X' {
            // lose
            match self.opponent_choice {
                'A' => SCISSOR_VALUE,
                'B' => ROCK_VALUE,
                _ => PAPER_VALUE,
            }
        } else if self.outcome == 'Y' {
            // draw
            DRAW_SCORE + match self.opponent_choice {
                'A' => ROCK_VALUE,
                'B' => PAPER_VALUE,
                _ => SCISSOR_VALUE,
            }
        } else {
            // win
            WIN_SCORE + match self.opponent_choice {
                'A' => PAPER_VALUE,
                'B' => SCISSOR_VALUE,
                _ => ROCK_VALUE,
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
