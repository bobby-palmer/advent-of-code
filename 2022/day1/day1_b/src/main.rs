use day1_b::{separate_inventory, n_max};


fn main() {
    let input = include_str!("../input.txt");
    let maxes = n_max(separate_inventory(input), 3);

    println!("Solution: {}", maxes.iter().sum::<u32>());
}
