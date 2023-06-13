const NUM_STACKS: usize = 9;
const SPACING: usize = 4;

fn main() {
    let input = include_str!("../../input.in");
    let (setup, actions) = input.split_once("\n\n").unwrap();
    let mut stacks: [String; NUM_STACKS] = Default::default();
    setup.lines().rev().skip(1).for_each(|line| {
        line.chars()
            .skip(1)
            .step_by(SPACING)
            .enumerate()
            .filter(|(_, letter)| letter.is_alphabetic())
            .for_each(|(index, letter)| {
                stacks[index].push(letter);
            })
    });
    actions.lines().for_each(|line| {
        let mut numbers = line
            .split_whitespace()
            .skip(1)
            .step_by(2)
            .filter_map(|letter| Some(letter.parse::<usize>().unwrap()));
        let count = numbers.next().unwrap();
        let source = numbers.next().unwrap() - 1;
        let destination = numbers.next().unwrap() - 1;
        let mut group = String::new();
        for _ in 0..count {
            let item = stacks[source].pop().unwrap();
            group.push(item);
        }
        for _ in 0..count {
            stacks[destination].push(group.pop().unwrap());
        }
    });

    stacks.iter().enumerate().for_each(|(index, stack)| {
        println!(
            "Stack number {} ended with {} on top",
            index + 1,
            stack.chars().last().unwrap()
        );
    });
}
