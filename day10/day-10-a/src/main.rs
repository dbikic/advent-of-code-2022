// https://adventofcode.com/2022/day/10

enum Instruction {
    NOOP,
    ADDX(i32),
}

impl Instruction {
    pub fn new(input: &str) -> Self {
        if input == "noop" {
            Instruction::NOOP
        } else {
            Instruction::ADDX(
                input.split(" ").last().unwrap().parse().unwrap(),
            )
        }
    }
}

fn main() {
    let mut register = 1;
    let mut cycle = 0;
    let mut interesting_strengths: Vec<i32> = vec![];
    include_str!("../input.txt")
        .lines()
        .into_iter()
        .for_each(|x| {
            match Instruction::new(x) {
                Instruction::NOOP => {
                    cycle += 1;
                    process_cycle(register, cycle, &mut interesting_strengths);
                }
                Instruction::ADDX(val) => {
                    cycle += 1;
                    process_cycle(register, cycle, &mut interesting_strengths);
                    cycle += 1;
                    process_cycle(register, cycle, &mut interesting_strengths);
                    register += val;
                }
            }
        });
    println!("{}", interesting_strengths.iter().sum::<i32>());
}

fn process_cycle(register: i32, cycle: i32, interesting_strengths: &mut Vec<i32>) {
    if (cycle + 20) % 40 == 0 {
        interesting_strengths.push(register * cycle);
    }
}