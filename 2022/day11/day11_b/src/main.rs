use common::math::Operator;
const ROUNDS: u32 = 10000;
fn main() {
    let input = include_str!("../../input.in");
    let mut monkeys = Vec::new();
    // read in monkey states
    input.split("\n\n").for_each(|monkey| {
        let mut input = monkey.lines().skip(1);
        let items: Vec<i64> = input
            .next()
            .unwrap()
            .split_once(":")
            .unwrap()
            .1
            .split(",")
            .filter_map(|num| num.trim().parse::<i64>().ok())
            .collect();
        let operation =
            Operator::from_string(input.next().unwrap().split_once("=").unwrap().1.trim()).unwrap();
        let test = input
            .next()
            .unwrap()
            .split_whitespace()
            .last()
            .unwrap()
            .parse::<i64>()
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
    let base: i64 = monkeys.iter().map(|m| m.test).product();
    // simulate the monkeys
    for _ in 0..ROUNDS {
        for i in 0..monkeys.len() {
            let items = monkeys[i].consider_items(base);
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
    let solution: u64 = monkeys
        .iter()
        .map(|monkey| monkey.inspected as u64)
        .rev()
        .take(2)
        .product();
    println!("found solution : {solution}");
}
#[derive(Debug)]
struct Monkey {
    items: Vec<i64>,
    operation: Operator,
    test: i64,
    truth: usize,
    non_truth: usize,
    inspected: u32,
}
impl Monkey {
    fn consider_items(&mut self, base: i64) -> Vec<i64> {
        self.items
            .drain(..)
            .map(|item| self.operation.eval(item as i64) % base)
            .collect()
    }
}
