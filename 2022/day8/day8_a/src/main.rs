use std::collections::BTreeSet;
struct Location(usize, usize);

fn main() {
    let input = include_str!("../../input.in");
    let width: usize = input.lines().next().unwrap().len();
    let height: usize = input.lines().count();
    let mut count_visible = 0;
    for row in 0..height {
        for col in 0..width {
            let spot = Location(row, col);
            if check_tree(input, spot, height, width) {
                count_visible += 1;
            }
        }
    }
    println!("found solution : {count_visible}");
}
fn get_tree(forest: &str, spot: Location) -> u32 {
    let Location(row, col) = spot;
    forest
        .lines()
        .nth(row)
        .unwrap()
        .chars()
        .nth(col)
        .unwrap()
        .to_digit(10)
        .unwrap()
}
fn check_tree(forest: &str, spot: Location, height: usize, width: usize) -> bool {
    let Location(row, col) = spot;
    let tree_height = get_tree(forest, spot);
    //check to the left
    let vis_left = (0..col).all(|col| {
        let spot = Location(row, col);
        get_tree(forest, spot) < tree_height
    });
    //check to the right
    let vis_right = (col + 1..width).all(|col| {
        let spot = Location(row, col);
        get_tree(forest, spot) < tree_height
    });
    //check to the left
    let vis_up = (0..row).all(|row| {
        let spot = Location(row, col);
        get_tree(forest, spot) < tree_height
    });
    //check to the left
    let vis_down = (row + 1..height).all(|row| {
        let spot = Location(row, col);
        get_tree(forest, spot) < tree_height
    });
    vis_left || vis_right || vis_up || vis_down
}
