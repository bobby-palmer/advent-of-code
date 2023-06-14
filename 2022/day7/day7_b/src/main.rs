const TOTAL_MEMORY: u64 = 70000000;
const REQUIRED_MEMORY: u64 = 30000000;

use std::iter::Peekable;

fn main() {
    let input = include_str!("../../input.in");
    let mut iter = input.lines().peekable();
    let mut dir_sizes = Vec::new();
    let used_storage = sum_dirs(&mut iter, &mut dir_sizes);
    let useable_storage = TOTAL_MEMORY - REQUIRED_MEMORY;
    let needs_deleted = used_storage - useable_storage;
    let solution = dir_sizes
        .iter()
        .filter(|el| *el > &needs_deleted)
        .min()
        .unwrap();
    println!("found solution : {solution}");
}

fn sum_dirs(
    input: &mut Peekable<impl Iterator<Item = &'static str>>,
    vector: &mut Vec<u64>,
) -> u64 {
    let mut sum: u64 = 0;
    while let Some(line) = input.next() {
        match line {
            "$ cd .." => break,
            _ if &line[0..3] == "$ l" => {
                sum += std::iter::from_fn(|| input.next_if(|i| i.chars().next().unwrap() != '$'))
                    .filter_map(|line| line.split_whitespace().next().unwrap().parse::<u64>().ok())
                    .sum::<u64>();
            }
            _ => sum += sum_dirs(input, vector),
        }
    }
    vector.push(sum);
    sum
}
