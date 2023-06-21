#[test]
fn main() {
    let input = include_str!("../testinput");
    // rebuild cave and simulate with different rules
    let mut caveb = day14::Cave::build(input);
    let solution_b = caveb.simulate_b();
    assert_eq!(solution_b, 93);
}
