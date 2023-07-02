use day19::{solve, solveb};
fn main() {
    let input = include_str!("../input");
    let mut solution = solve(input);
    println!("found solution : {solution}");
    solution = solveb(input);
    println!("found part b solution: {solution}");
}
