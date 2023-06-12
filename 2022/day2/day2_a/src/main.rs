use day2_a::Round;

fn main() {
    let input = include_str!("../../input.in");
    let total_score: u32 = input.lines().map(|line| {
        Round::new(line).score_round()
    }).sum();

    println!("solution :{}", total_score);
}
