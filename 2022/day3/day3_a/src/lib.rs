fn split_line(line: &str) -> (&str, &str) {
    let index: usize = line.len() / 2;
    line.split_at(index)
}
pub fn find_common_letter(mut lines: Vec<&str>) -> Option<char> {
    let primary_line = lines.pop().unwrap().chars();
    for letter in primary_line {
        if lines.iter().all(|line| {
            line.contains(letter)
        }) {
            return Some(letter)
        }
    }
    None
}
pub fn get_priority(letter: char) -> u8 {
    if letter.is_uppercase() {
        letter as u8 - 'A' as u8 + 27
    } else {
        letter as u8 - 'a' as u8 + 1
    }
}
pub fn get_line_priority(line: &str) -> u8 {
    let (line1, line2) = split_line(line);
    let lines = vec!(line1, line2);
    get_priority(find_common_letter(lines).unwrap())
}
#[cfg(test)]
mod test {
    use crate::{find_common_letter, get_priority};

    #[test]
    fn test_common() {
        let s1 = "vJrwpWtwJgWrhcsFMMfFFhFp";
        let s2 = "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL";
        let s3 = "PmmdzqPrVvPwwTWBwg";
        let strings = vec!(s3,s2,s1);
        assert_eq!('r', find_common_letter(strings).unwrap());
        assert_eq!(18, get_priority('r'));
    }
}
