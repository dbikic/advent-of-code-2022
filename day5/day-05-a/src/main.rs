struct Instruction {
    count: usize,
    from: usize,
    to: usize,
}

impl Instruction {
    pub fn new(x: &str) -> Self {
        let mut stripped = x.strip_prefix("move ").unwrap();
        let count = stripped[0..stripped.find(" ").unwrap()].parse::<usize>().unwrap();
        stripped = stripped.strip_prefix(&format!("{} from ", count)).unwrap();
        let from = stripped[0..stripped.find(" ").unwrap()].parse::<usize>().unwrap();
        stripped = stripped.strip_prefix(&format!("{} to ", from)).unwrap();
        let to = stripped.parse::<usize>().unwrap();
        Instruction {
            count,
            from: from - 1,
            to: to - 1,
        }
    }
}

fn main() {
    let mut stacks: Vec<Vec<char>> = vec![];
    let mut inputs: Vec<&str> = vec![];
    let mut i = 0;
    let mut start_index = 0;
    let mut read_instruction = false;
    include_str!("../input.txt")
        .lines()
        .into_iter()
        .for_each(|x| {
            if x.starts_with(" 1") {
                start_index = i;
                let stacks_count = x.chars().filter(|x| x.is_digit(10)).count();
                for _ in 0..stacks_count {
                    stacks.push(vec![]);
                }
                for x in &inputs {
                    for j in 0..stacks_count {
                        let item = x.as_bytes()[j * 4 + 1];
                        if (item as char) != ' ' {
                            stacks[j].push(item as char);
                        }
                    }
                }
                i += 1;
            } else if x.is_empty() {
                read_instruction = true;
                for x in &mut stacks {
                    x.reverse();
                }
                i += 1;
            } else if read_instruction {
                let instruction = Instruction::new(x);
                for _ in 0..instruction.count {
                    let pop = stacks[instruction.from].pop().unwrap();
                    stacks[instruction.to].push(pop);
                }
            } else {
                inputs.push(x);
                i += 1;
            }
        }
        );
    let top: String = stacks.into_iter().map(|mut s| s.pop().unwrap()).collect();
    println!("{}", top);
}