fn main() {
    let input = include_str!("../../input.in");
    let game = common::Board::new(input);
    let solution = game.get_solution_b();
    println!(" found solution : {solution}");
}
