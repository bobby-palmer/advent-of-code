fn main() {
    let input = include_str!("../input");
    let mut cave = day14::Cave::build(input);
    let solution_a = cave.simulate();
    println!("found solution : {solution_a}");
    // rebuild cave and simulate with different rules
    let mut caveb = day14::Cave::build(input);
    let solution_b = caveb.simulate_b();
    println!("found solution : {solution_b}");
}
