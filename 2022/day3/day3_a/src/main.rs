fn main() {
    let input = include_str!("../../input.in");
    let sum: u32 = input.lines().map(|line| {
        day3_a::get_line_priority(line) as u32
    }).sum();
    println!("Solution :{}", sum);
}
