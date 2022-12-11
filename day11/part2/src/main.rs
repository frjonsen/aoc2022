use std::{cell::RefCell, cmp::Reverse, collections::VecDeque};

enum Operation {
    Add(u64),
    Multiply(u64),
    Square,
}

impl Operation {
    fn act(&self, item: Item) -> Item {
        match self {
            Self::Add(m) => Item(item.0 + m),
            Self::Multiply(m) => Item(item.0 * m),
            Self::Square => Item(item.0 * item.0),
        }
    }
}

#[derive(Eq, PartialEq, Debug)]
struct Item(u64);

struct Monkey {
    items: VecDeque<Item>,
    operation: Operation,
    test: u64,
    false_target: usize,
    true_target: usize,
    actions_taken: u128,
}

fn parse_operation(operation: &str) -> Operation {
    let (_, eq) = operation.split_once("= ").unwrap();
    let mut parts = eq.split_whitespace().skip(1);
    let operation = parts.next().unwrap();
    let operand = parts.next().unwrap();

    let Ok(operand) = operand.parse::<u64>() else {
        return Operation::Square;
    };

    match operation {
        "*" => Operation::Multiply(operand),
        "+" => Operation::Add(operand),
        _ => panic!("Unexpected operation"),
    }
}

fn parse_monkey(input: &str) -> Monkey {
    let mut lines = input.lines().skip(1).map(str::trim);
    let (_, items) = lines.next().unwrap().split_once(": ").unwrap();
    let items: VecDeque<Item> = items
        .split(", ")
        .map(|i| i.parse().unwrap())
        .map(Item)
        .collect();

    let operation = parse_operation(lines.next().unwrap());

    let test = lines
        .next()
        .and_then(|l| l.split_whitespace().last())
        .and_then(|l| l.parse().ok())
        .unwrap();

    let true_target = lines
        .next()
        .and_then(|l| l.split_whitespace().last())
        .and_then(|l| l.parse().ok())
        .unwrap();
    let false_target = lines
        .next()
        .and_then(|l| l.split_whitespace().last())
        .and_then(|l| l.parse().ok())
        .unwrap();

    Monkey {
        items,
        operation,
        test,
        true_target,
        false_target,
        actions_taken: 0,
    }
}

fn parse_input(input: &str) -> Vec<RefCell<Monkey>> {
    input
        .trim()
        .split("\n\n")
        .map(str::trim)
        .map(parse_monkey)
        .map(RefCell::new)
        .collect()
}

fn run(mut monkeys: Vec<RefCell<Monkey>>, total_test: u64) -> u128 {
    for _ in 0..10000 {
        for monkey in &monkeys {
            let mut monkey = monkey.borrow_mut();
            while let Some(i) = monkey.items.pop_front() {
                let worry = monkey.operation.act(i);
                let target = if worry.0 % monkey.test == 0 {
                    &monkeys[monkey.true_target]
                } else {
                    &monkeys[monkey.false_target]
                };
                let worry = Item(worry.0 % total_test);
                target.borrow_mut().items.push_back(worry);
                monkey.actions_taken += 1;
            }
        }
    }

    monkeys.sort_by_key(|m| Reverse(m.borrow().actions_taken));

    monkeys[..=1]
        .iter()
        .map(|m| m.borrow().actions_taken)
        .product()
}

fn main() {
    let input = include_str!("../../input.txt");
    let monkeys = parse_input(input);
    let total_test: u64 = monkeys.iter().map(|c| c.borrow().test).product();
    let res = run(monkeys, total_test);

    println!("{res}")
}

#[cfg(test)]
mod tests {
    use crate::{parse_input, run, Item};

    #[test]
    fn test_sample() {
        let input = include_str!("../sample.txt");
        let monkeys = parse_input(input);
        let total_test: u64 = monkeys.iter().map(|c| c.borrow().test).product();

        let monkey_one = &monkeys[0];
        assert!(monkey_one.borrow().items == [Item(79), Item(98)]);
        assert_eq!(23, monkey_one.borrow().test);
        assert_eq!(19, monkey_one.borrow().operation.act(Item(1)).0);
        assert_eq!(3, monkey_one.borrow().false_target);
        assert_eq!(2, monkey_one.borrow().true_target);

        let res = run(monkeys, total_test);
        assert_eq!(2713310158, res);
    }
}
