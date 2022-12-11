use std::collections::VecDeque;
use num_bigint::{BigUint, ToBigUint};

pub struct Monkey {
    item_queue: VecDeque<BigUint>,
    square: bool,
    operation_multiplier: u32,
    operation_adder: u32,
    pub test_divider: u32,
    true_monkey: usize,
    false_monkey: usize,
}

impl Monkey {
    pub fn parse(lines: &[&str]) -> Monkey {
        let item_strings = lines[1].split("items: ").skip(1).next().unwrap();
        let items: VecDeque<BigUint> = item_strings.split(", ").map(|s| s.parse().unwrap()).collect();

        let mut operation_lines = lines[2].trim().split_whitespace().skip(4);
        let operation = operation_lines.next().unwrap();
        let operator_string = operation_lines.next().unwrap();
        let (square, operator) = match operator_string {
            "old" => (true, 1),
            _ => (false, operator_string.parse().unwrap()),
        };

        let divider: u32 = lines[3].trim().split_whitespace().skip(3).next().unwrap().parse().unwrap();

        let true_monkey: usize = lines[4].trim().split_whitespace().skip(5).next().unwrap().parse().unwrap();
        let false_monkey: usize = lines[5].trim().split_whitespace().skip(5).next().unwrap().parse().unwrap();

        if square {
            return Monkey::square_monkey(items, divider, true_monkey, false_monkey);
        }

        return match operation {
            "*" => Monkey::multiply_monkey(items, operator, divider, true_monkey, false_monkey),
            "+" => Monkey::adding_monkey(items, operator, divider, true_monkey, false_monkey),
            _ => panic!()
        };
    }

    fn square_monkey(item_queue: VecDeque<BigUint>, divider: u32, true_monkey: usize, false_monkey: usize) -> Monkey {
        Self {
            item_queue,
            square: true,
            operation_multiplier: 1,
            operation_adder: 0,
            test_divider: divider,
            true_monkey,
            false_monkey,
        }
    }

    fn adding_monkey(item_queue: VecDeque<BigUint>, adder: u32, divider: u32, true_monkey: usize, false_monkey: usize) -> Monkey {
        Self {
            item_queue,
            square: false,
            operation_multiplier: 1,
            operation_adder: adder,
            test_divider: divider,
            true_monkey,
            false_monkey,
        }
    }

    fn multiply_monkey(item_queue: VecDeque<BigUint>, multiplier: u32, divider: u32, true_monkey: usize, false_monkey: usize) -> Monkey {
        Self {
            item_queue,
            square: false,
            operation_multiplier: multiplier,
            operation_adder: 0,
            test_divider: divider,
            true_monkey,
            false_monkey,
        }
    }

    pub fn inspect_items(&mut self, calm_down: &impl Fn(&BigUint) -> BigUint) -> Vec<(usize, BigUint)> {
        let mut result = vec!();
        for mut item in self.item_queue.drain(0..) {
            if self.square {
                item = &item * &item;
            }
            item =&item * self.operation_multiplier + self.operation_adder;

            item = calm_down(&item);

            if &item % self.test_divider == ToBigUint::to_biguint(&0).unwrap() {
                result.push((self.true_monkey.clone(), item));
            }
            else {
                result.push((self.false_monkey.clone(), item))
            }
        }
        return result;
    }

    pub fn catch_item(&mut self, item: BigUint) {
        self.item_queue.push_back(item);
    }
}
