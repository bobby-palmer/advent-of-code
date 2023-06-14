const NUM_CHARS: usize = 4;

fn main() {
    let input = include_str!("../../input.in");
    let input_length: usize = input.len();
    for index in 0..input_length - NUM_CHARS {
        if all_different(get_slice(input, index)) {
            println!(" Solution : {}", index + NUM_CHARS);
            return ();
        }
    }
}
fn get_slice(longstring: &str, index: usize) -> &str {
    let start = index;
    let end = start + NUM_CHARS;
    &longstring[start..end]
}
fn all_different(slice: &str) -> bool {
    slice.chars().enumerate().all(|(index1, letter1)| {
        slice
            .chars()
            .enumerate()
            .filter(|(index2, _)| index1 != *index2)
            .all(|(_, letter2)| letter1 != letter2)
    })
}
