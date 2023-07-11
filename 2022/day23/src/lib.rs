use std::collections::BTreeSet;

const INPUT: &str = include_str!("../input");
const TEST: &str = include_str!("../testinput");

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
fn surrounded(elf: &Elf, elves: &BTreeSet<Elf>) -> bool {
    for row in -1..=1 {
        for col in -1..=1 {
            if !(row == 0 && col == 0) && elves.contains(&Elf {
                row: elf.row + row,
                col: elf.col + col,
            }) {
                return true
            }
        }
    }
    false
}
fn simulate_round(input: &mut BTreeSet<Elf>)  {
    unimplemented!();
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_parse() {
        let tree = parse(TEST);
        assert!(tree.contains(&Elf {
            row: 1,
            col: 2,
        }));
    }
    #[test]
    fn test_surroundings() {
       let tree = parse(TEST);
        let test_elf = &Elf {
            row:1,
            col:2,
        };
        assert!(surrounded(test_elf, &tree));
    }
}
