// https://adventofcode.com/2022/day/11

use crate::monkey::Monkey;

mod operation;
mod monkey;


fn main() {
    let lines: Vec<&str> = include_str!("../input.txt")
        .lines()
        .filter(|x| !x.is_empty())
        .collect();
    let mut monkeys: Vec<Monkey> = vec![];
    for i in 0..lines.len() / 6 {
        monkeys.push(Monkey::new(&lines[i * 6..(i * 6) + 6]));
    }
    let product =  monkeys.iter().map(|m| m.test_divisible_by).product::<u64>();
    for i in 0..10000 {
        println!("Round: {}", i);
        for monkey in &monkeys {
            let result = monkey.process(product);
            for r in result {
                monkeys[r.0].items.borrow_mut().push(r.1);
            }
        }
        print_monkeys(&monkeys);
        println!();
        println!();
        println!();
    }
    let mut inspections = monkeys.iter()
        .map(|x| x.inspections.take())
        .collect::<Vec<u64>>();
    inspections.sort();
    println!("{}", inspections[inspections.len() - 1] * inspections[inspections.len() - 2])
}

fn print_monkeys(m: &Vec<Monkey>) {
    for i in 0..m.len() {
        println!("M{} : {}", i, m[i]);
    }
}
