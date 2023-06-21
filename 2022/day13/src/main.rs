use std::cmp::Ordering;

fn bool_to_order(tf: bool) -> Ordering {
    if tf {
        Ordering::Less
    } else {
        Ordering::Greater
    }
}
fn main() {
    let input = include_str!("../input");
    let solution: u32 = input
        .split("\n\n")
        .filter(|pair| !pair.is_empty())
        .enumerate()
        .map(|(index, pair)| {
            if let Some((mut first, mut second)) = pair.split_once('\n') {
                first = first.trim_end();
                second = second.trim_end();
                if day13::in_correct_order(first, second) {
                    1 + index as u32
                } else {
                    0
                }
            } else {
                println!("couldn't parse this pair");
                0
            }
        })
        .sum();
    println!("found solution a : {solution}");
    let mut packets: Vec<&str> = input.split('\n').filter(|line| !line.is_empty()).collect();
    let first_marker = "[[2]]";
    let second_marker = "[[6]]";
    packets.push(first_marker);
    packets.push(second_marker);
    packets.sort_by(|packet1, packet2| bool_to_order(day13::in_correct_order(packet1, packet2)));
    let solution_b: usize = packets
        .iter()
        .enumerate()
        .filter_map(|(index, packet)| {
            if packet == &first_marker {
                Some(index + 1)
            } else if packet == &second_marker {
                Some(index + 1)
            } else {
                None
            }
        })
        .product();
    println!("found solution b : {solution_b}");
}
