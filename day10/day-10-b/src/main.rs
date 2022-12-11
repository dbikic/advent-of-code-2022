// https://adventofcode.com/2022/day/11

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
    let mut image: Vec<Vec<char>> = vec![vec![]; 6];
    include_str!("../input.txt")
        .lines()
        .into_iter()
        .for_each(|x| {
            match Instruction::new(x) {
                Instruction::NOOP => {
                    cycle += 1;
                    process_cycle(register, cycle, &mut image);
                }
                Instruction::ADDX(val) => {
                    cycle += 1;
                    process_cycle(register, cycle, &mut image);
                    cycle += 1;
                    process_cycle(register, cycle, &mut image);
                    register += val;
                }
            }
        });
    for i in 0..6 {
        image.push(vec![]);
        for j in 0..40 {
            print!("{}", image[i][j]);
        }
        println!();
    }
}

fn process_cycle(register: i32, cycle: i32, image: &mut Vec<Vec<char>>) {
    let x = ((cycle - 1) / 40) as usize;
    let c = cycle % 40;
    let char = if (register - 1..register + 2).contains(&(c - 1)) {
        '#'
    } else {
        ' '
    };
    image[x].push(char);
}