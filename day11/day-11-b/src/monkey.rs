use std::cell::RefCell;
use std::fmt;

use crate::operation::Operation;


#[derive(Clone)]
pub struct Monkey {
    pub items: RefCell<Vec<u64>>,
    operation: Operation,
    pub test_divisible_by: u64,
    test_true_moneky: usize,
    test_false_monkey: usize,
    pub inspections: RefCell<u64>,
}

impl Monkey {
    pub fn new(slice: &[&str]) -> Self {
        Monkey {
            items: RefCell::new(
                slice[1].strip_prefix("  Starting items: ")
                    .unwrap()
                    .split(", ")
                    .map(|x| x.parse::<u64>().unwrap())
                    .collect()
            ),
            operation: Operation::new(
                slice[2].strip_prefix("  Operation: new = old ")
                    .unwrap()
            ),
            test_divisible_by: slice[3].strip_prefix("  Test: divisible by ")
                .unwrap()
                .parse::<u64>()
                .unwrap(),
            test_true_moneky: slice[4].strip_prefix("    If true: throw to monkey ")
                .unwrap()
                .parse::<usize>()
                .unwrap(),
            test_false_monkey: slice[5].strip_prefix("    If false: throw to monkey ")
                .unwrap()
                .parse::<usize>()
                .unwrap(),
            inspections: RefCell::new(0),
        }
    }

    pub fn process(&self, product: u64) -> Vec<(usize, u64)> {
        let mut items_to_throw: Vec<(usize, u64)> = vec![];
        self.items.borrow().iter().for_each(|x| {
            *self.inspections.borrow_mut() += 1;
            let new_value = match self.operation {
                Operation::Plus(i) => x + i,
                Operation::Multiply(i) => x * i,
                Operation::Square => {
                    println!("x: {}", x);
                    x * x
                },
            } % product;
            if new_value % self.test_divisible_by == 0 {
                items_to_throw.push((self.test_true_moneky, new_value));
            } else {
                items_to_throw.push((self.test_false_monkey, new_value));
            }
        }
        );
        self.items.borrow_mut().clear();
        items_to_throw
    }
}

impl fmt::Display for Monkey {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // write!(f, "Starting items:").unwrap();
        // self.items.borrow().iter().for_each(|x| {
        //     write!(f, "[{}] ", x).unwrap();
        // }
        // );
        // writeln!(f).unwrap();
        // Ok(())
        // writeln!(f, "Operation:{}", &self.operation).unwrap();
        // writeln!(f, "Divisible by: {}", &self.test_divisible_by).unwrap();
        // writeln!(f, "True: m{}", &self.test_true_moneky).unwrap();
        // writeln!(f, "False: m{}", &self.test_false_monkey).unwrap();
        writeln!(f, "Inspections: {}", &self.inspections.borrow())
    }
}