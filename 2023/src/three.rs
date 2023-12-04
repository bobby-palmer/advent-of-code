use std::{io, collections::HashSet};

use crate::util;

pub fn solve() -> io::Result<()> {
    let input = util::read("day3.txt")?;
    let a = solve_a(&input);
    println!("Solution to part a : {a}");
    Ok(())
}

const DIR: [(i32, i32); 8] = [
    (-1, -1),
    (0, -1),
    (1, -1),
    (-1, 0),
    (1, 0),
    (-1, 1),
    (0, 1),
    (1, 1)
];

fn solve_a(s: &str) -> u32 {
    let mut good = HashSet::new();
    s.lines().enumerate().for_each(|(row, l)| {
        l.char_indices().for_each(|(col, c)| {
            if !c.is_digit(10) && !(c == '.') {
                for (dr, dc) in DIR {
                    good.insert((row as i32 + dr, col as i32 + dc));
                } 
            }
        });
    });

    s.lines().enumerate().map(|(row, l)| {
        l.char_indices().filter_map(|(col, c)| {
            if good.contains(&(row as i32, col as i32)) {
                c.to_digit(10)
            } else {
                None
            }
        }).sum::<u32>()
    }).sum()
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_a() {
       let input = util::read("test3.txt").unwrap();
       let res = solve_a(&input);
       assert_eq!(res, 4361);
    }
}
