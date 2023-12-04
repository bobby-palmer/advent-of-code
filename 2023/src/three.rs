use std::{io, collections::HashSet};

use crate::util;

pub fn solve() -> io::Result<()> {
    let input = util::read("day3.txt")?;
    let a = solve_a(&input);
    println!("Solution to part a : {a}");
    Ok(())
}

fn solve_a(s: &str) -> i32 {
    let v: Vec<&str> = s.lines().collect();
}
