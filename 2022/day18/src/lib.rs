use itertools::Itertools;

fn parse(input: &str) -> (u16, u16, u16) {
    input
        .split(',')
        .map(|num| num.parse::<u16>().unwrap())
        .collect_tuple()
        .unwrap()
}
