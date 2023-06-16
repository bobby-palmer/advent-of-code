use common::math::Operator;
const ROUNDS: u8 = 20;
fn main() {
    let input = include_str!("../../input.in");
    let mut monkeys = Vec::new();
    // read in monkey states
    input.split("\n\n").for_each(|monkey| {
        let mut input = monkey.lines().skip(1);
        let items: Vec<i32> = input
            .next()
            .unwrap()
            .split_once(":")
            .unwrap()
            .1
            .split(",")
            .filter_map(|num| num.trim().parse::<i32>().ok())
            .collect();
        let operation =
            Operator::from_string(input.next().unwrap().split_once("=").unwrap().1.trim()).unwrap();
        let test = input
            .next()
            .unwrap()
            .split_whitespace()
            .last()
            .unwrap()
            .parse::<i32>()
            .unwrap();
        let truth = input
            .next()
            .unwrap()
            .split_whitespace()
            .last()
            .unwrap()
            .parse::<usize>()
            .unwrap();
        let non_truth = input
            .next()
            .unwrap()
            .split_whitespace()
            .last()
            .unwrap()
            .parse::<usize>()
            .unwrap();
        monkeys.push(Monkey {
            items,
            operation,
            test,
            truth,
            non_truth,
            inspected: 0,
        });
    });
    monkeys.iter().for_each(|monke| {
        dbg!(monke);
    });
    // simulate the monkeys
    for _ in 0..ROUNDS {
        for i in 0..monkeys.len() {
            let items = monkeys[i].consider_items();
            let test = monkeys[i].test;
            let truth_targ = monkeys[i].truth;
            let false_targ = monkeys[i].non_truth;
            monkeys[i].inspected += items.len() as u32;
            for item in items.iter() {
                let target = if item % test == 0 {
                    truth_targ
                } else {
                    false_targ
                };
                monkeys[target].items.push(*item);
                println!("monkey {i} threw {item} to monkey {target}");
            }
        }
    }
    // print stats
    monkeys.sort_by(|monkeya, monkeyb| monkeya.inspected.cmp(&monkeyb.inspected));
    let solution: u32 = monkeys
        .iter()
        .map(|monkey| monkey.inspected)
        .rev()
        .take(2)
        .product();
    println!("found solution : {solution}");
}
#[derive(Debug)]
struct Monkey {
    items: Vec<i32>,
    operation: Operator,
    test: i32,
    truth: usize,
    non_truth: usize,
    inspected: u32,
}
impl Monkey {
    fn new(
        items: Vec<i32>,
        operation: Operator,
        test: i32,
        truth: usize,
        non_truth: usize,
        inspected: u32,
    ) -> Self {
        Self {
            items,
            operation,
            test,
            truth,
            non_truth,
            inspected,
        }
    }

    fn consider_items(&mut self) -> Vec<i32> {
        self.items
            .drain(..)
            .map(|item| self.operation.eval(item) / 3)
            .collect()
    }
}
