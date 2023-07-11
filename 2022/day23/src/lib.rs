use std::collections::BTreeSet;

#[derive(PartialEq, PartialOrd, Eq, Ord)]
struct Elf {
    row: i64,
    col: i64,
}
fn parse(input: &str) -> BTreeSet<Elf> {
    input.lines().enumerate().map(|(row, line)| {
        line.chars().enumerate().filter_map(move |(col, letter)| {
            match letter {
                '#' => Some(Elf{
                    row: row as i64,
                    col: col as i64,
                }),
                _ => None
            }
        })
    }).flatten().collect()
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_parse() {
        let input = include_str!("../testinput");
        let tree = parse(input);
        assert!(tree.contains(&Elf {
            row: 1,
            col: 2,
        }));
    }
}
