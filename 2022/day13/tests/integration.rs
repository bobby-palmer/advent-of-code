#[test]
fn test_on_small() {
    let input = include_str!("../testinput");
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
    assert_eq!(solution, 13);
}
