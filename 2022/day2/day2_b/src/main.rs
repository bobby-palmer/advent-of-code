fn main() {
    let input = include_str!("../../input.in");
    let total: u32 = input.lines().map(|line| {
        day2_b::get_round(line).score_round()
    }).sum();
    println!("Solution: {}", total);
}
