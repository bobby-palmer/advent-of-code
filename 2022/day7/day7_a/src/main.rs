use std::iter::Peekable;

const MAX_DIR_SIZE: u64 = 100000;

fn main() {
    let mut input = include_str!("../../input.in").lines().peekable();
    let mut solution = 0;
    sum_dirs(&mut input, &mut solution);
    println!("found solution: {solution}");
}

fn sum_dirs(input: &mut Peekable<impl Iterator<Item = &'static str>>, total: &mut u64) -> u64 {
    let mut sum: u64 = 0;
    while let Some(line) = input.next() {
        match line {
            "$ cd .." => break,
            _ if &line[0..3] == "$ l" => {
                sum += std::iter::from_fn(|| input.next_if(|i| i.chars().next().unwrap() != '$'))
                    .filter_map(|line| line.split_whitespace().next().unwrap().parse::<u64>().ok())
                    .sum::<u64>();
            }
            _ => sum += sum_dirs(input, total),
        }
    }
    if sum < MAX_DIR_SIZE {
        *total += sum;
    }
    sum
}
