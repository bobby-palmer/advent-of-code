fn main() {
    let input = include_str!("../../input.in");
    let mut total_strength = 0;
    let mut cycle: i32 = 0;
    let mut register: i32 = 1;
    input
        .lines()
        .filter(|line| !line.is_empty())
        .for_each(|line| {
            if line.chars().nth(0).unwrap() == 'n' {
                cycle += 1;
                if check_cycle(cycle) {
                    total_strength += cycle * register;
                }
            } else {
                let (_, addition) = line.split_once(" ").unwrap();
                // skip one cycle
                cycle += 1;

                // check if at 20, 60, 100, 140 ...
                if check_cycle(cycle) {
                    total_strength += cycle * register;
                }

                cycle += 1;
                if check_cycle(cycle) {
                    total_strength += cycle * register;
                }
                // perform addition and cycle
                register += addition.parse::<i32>().unwrap();
            }
            // check if at 20, 60, 100, 140 ...
        });
    println!("found solution : {total_strength}");
}
fn check_cycle(cur_cycle: i32) -> bool {
    (cur_cycle - 20) % 40 == 0
}
