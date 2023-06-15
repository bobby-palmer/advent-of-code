const WIDTH: usize = 99;
const HEIGHT: usize = 99;
fn main() {
    let input = include_bytes!("../../input.in");
    let mut solution = 0;
    let mut forest = Vec::new();
    input
        .split(|byte| byte == &b'\n')
        .filter(|line| !line.is_empty())
        .for_each(|line| {
            forest.push(line);
        });
    for row in 0..HEIGHT {
        for col in 0..WIDTH {
            let current = forest[row][col];
            //check up
            let thisscore = (((1..row)
                .map(|y| forest[row - y][col])
                .position(|h| h >= current)
                .unwrap_or_else(|| row.wrapping_sub(1))
                .wrapping_add(1))
            // check down
            * ((row + 1..HEIGHT).map(|y| forest[y][col]).position(|h| h >= current).unwrap_or_else(|| (HEIGHT - row).wrapping_sub(2)).wrapping_add(1))
            * ((1..col).map(|x| forest[row][col - x]).position(|h| h >= current).unwrap_or_else(||
                col.wrapping_sub(1)).wrapping_add(1)) 
            * ((col + 1..WIDTH).map(|x| forest[row][x]).position(|h| h >= current).unwrap_or_else(|| (WIDTH - col).wrapping_sub(2)).wrapping_add(1)));
            if thisscore > solution {
                solution = thisscore;
            }
        }
    }
    println!("found solution : {solution}");
}
