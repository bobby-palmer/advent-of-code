fn main() {
    let input = include_str!("../../input.in");
    let solution: u64 = input.lines().map(|line| {
        let (range1, range2) = day4_a::get_ranges(line);
        range1.intersects(&range2) as u64
    }).sum();
    println!("found solution : {}", solution);
}
