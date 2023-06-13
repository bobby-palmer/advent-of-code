

fn main() {
    let input = include_str!("../../input.in");
    let solution: u16= input.lines().map(|line| {
       let (range1, range2) = day4_a::get_ranges(line);
        day4_a::one_contains_other(range1, range2) as u16
    }).sum();
    println!("Solution : {}", solution);
}
