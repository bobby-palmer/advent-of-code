use day20::{solve, solve_b};
fn main() {
    let input = include_str!("../input");
    let solution = solve(input);
    println!("found solution : {solution}");
    let sol_b = solve_b(input);
    println!("found solution_b : {sol_b}");
}
