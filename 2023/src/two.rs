use crate::util;
use std::collections::HashMap;


pub fn solve() -> std::io::Result<()> {
    let input = util::read("day2.txt")?;
    let a = solve_a(&input);
    println!("Solution a : {a}");
    let b = solve_b(&input);
    println!("Solution b : {b}");
    Ok(())
}

fn map_a(t: (usize, &str)) -> i32 {
    let maxes: HashMap<char, i32> = HashMap::from([
        ('r', 12),
        ('g', 13),
        ('b', 14),
    ]);

   let (i, l) = t; 
   let mut it = l.split_whitespace();
   it.nth(1); // skip first two

   while let (Some(n), Some(c)) = (it.next(), it.next()) {
       let num = n.parse::<i32>().unwrap(); 
       if maxes[&c.chars().next().unwrap()] < num {
           return 0
       }
   }
   i as i32 + 1
}
fn solve_a(s: &str) -> i32 {
    util::sum_map(s, map_a)
}

fn map_b(t: (usize, &str)) -> i32 {
    let (_, l) = t;
    let mut it = l.split_whitespace();
    let (mut r, mut g, mut b) = (0, 0, 0);
    while let (Some(n), Some(c)) = (it.next(), it.next()) {
        match c.chars().next() {
            Some('r') => r = std::cmp::max(r, n.parse().unwrap()),
            Some('g') => g = std::cmp::max(g, n.parse().unwrap()),
            Some('b') => b = std::cmp::max(b, n.parse().unwrap()),
            _ => (),
        }
    }
    r * g * b
}
fn solve_b(s: &str) -> i32 {
    util::sum_map(s, map_b)
}

#[cfg(test)]
mod tests {
    use super::*;

    const S: &str = 
"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    #[test]
    fn test_a() {
       assert_eq!(solve_a(S), 8); 
    }

    #[test]
    fn test_b() {
        assert_eq!(solve_b(S), 2286);
    }
}
