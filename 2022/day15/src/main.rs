use day15::Point;

const LINE: i32 = 2000000;
fn main() {
    let input = include_str!("../input");
    let points: Vec<(Point, Point)> = input
        .split('\n')
        .filter(|line| !line.is_empty())
        .map(|line| day15::line_to_points(line))
        .collect();
    let solution = day15::solve(&points, LINE);
    println!("found solution : {solution}");
    let solutionb = day15::solve_b(&points, 4000000);
    println!("found solution b : {solutionb}");
}
