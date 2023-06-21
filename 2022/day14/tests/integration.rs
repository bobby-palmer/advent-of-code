use day14::Cave;

#[test]
fn test_all() {
    let input = include_str!("../testinput");
    let mut cave = Cave::build(input);
    assert_eq!(24, cave.simulate());
}
