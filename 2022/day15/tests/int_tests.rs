use day15::Point;
#[test]
fn test_integration() {
    let input = include_str!("../inputtest");
    let points: Vec<(Point, Point)> = input
        .split('\n')
        .filter(|line| !line.is_empty())
        .map(|line| day15::line_to_points(line))
        .collect();
    let solution = day15::solve(&points, 10);
    assert_eq!(solution, 26);
    let solution2 = day15::solve_b(&points, 20);
    assert_eq!(solution2, 56000011);
}
