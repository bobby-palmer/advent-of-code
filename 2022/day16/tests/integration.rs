use day16::*;

#[test]
fn test_all() {
    let input = include_str!("../testinput");
    let network = Network::from_input(input);
    let state = State::parse(30, &network);
    let solution = state.simulate();
    assert_eq!(solution, 1651);
}
