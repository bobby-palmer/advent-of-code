fn main() {
    let mut input = include_str!("../../input.in").lines();
    let mut total = 0;
    while let Some(line) = input.next() {
        let mut group = Vec::new();
        group.push(line);
        group.push(input.next().unwrap());
        group.push(input.next().unwrap());
        let letter = day3_a::find_common_letter(group)
            .expect("No common letter");
        total += day3_a::get_priority(letter) as u32;
    }
    println!("Solution : {}", total);
}
