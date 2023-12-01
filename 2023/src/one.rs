use std::{fs::File, io::{BufRead, self}};
use std::iter::Iterator;

pub fn solve(in_file: &str) -> std::io::Result<()> {
    let f = File::open(in_file)?;
    let r = io::BufReader::new(f);

    let ans_a: u32 = r.lines().map(|l| l.unwrap()).map(|l| {
        let mut it = l.chars().filter(|c| c.is_digit(10));
        let first = it.next().unwrap();
        let second = it.last().unwrap_or(first);
        first.to_digit(10).unwrap() * 10 + second.to_digit(10).unwrap() 
    }).sum();

    println!("Solution a : {ans_a}");

    let f2 = File::open(in_file)?;
    let r2 = io::BufReader::new(f2);

    let ans_b: u32 = r2.lines().map(|l| num_map(&l.unwrap())).map(|l| {
        let mut it = l.chars().filter(|c| c.is_digit(10));
        let first = it.next().unwrap();
        let second = it.last().unwrap_or(first);
        let res = first.to_digit(10).unwrap() * 10 + second.to_digit(10).unwrap();
        res
    }).sum();

    println!("Solution b : {ans_b}");
    Ok(())
}

fn num_map(s: &str) -> String {
    s.replace("one", "o1e")
     .replace("two", "t2")
     .replace("three", "t3e")
     .replace("four", "4")
     .replace("five", "5e")
     .replace("six", "6")
     .replace("seven", "7n")
     .replace("eight", "e8")
     .replace("nine", "9")
}

