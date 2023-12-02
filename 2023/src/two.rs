use crate::util;

pub fn solve() -> std::io::Result<()> {
    let input = util::read("day2.txt")?;
    let a = solve_a(&input);
    println!("Solution a : {a}");
    let b = solve_b(&input);
    println!("Solution b : {b}");
    Ok(())
}

fn solve_a(s: &str) -> u32 {
    s.lines().enumerate().map(|(i, l)| {
        let mut it = l.split_whitespace();
        it.next();
        it.next();
        while let Some(num) = it.next() {
            let color = it.next().unwrap();
            let n = num.parse::<i32>().unwrap();
            if color.contains("red") && n > 12 {
               return 0u32 
            } else if color.contains("green") && n > 13 {
                return 0u32
            } else if color.contains("blue") && n > 14 {
                return 0u32
            }
        }
        i as u32 + 1
    }).sum()
}

fn solve_b(s: &str) -> u32 {
    s.lines().map(|l| {
        let mut it = l.split_whitespace();
        it.next();
        it.next();
        let (mut r, mut g, mut b) = (0, 0, 0);
        while let Some(num) = it.next() {
            let c = it.next().unwrap();
            let v = num.parse().unwrap();
            if c.contains("red") {
                r = std::cmp::max(r, v);
            } else if c.contains("blue") {
                b = std::cmp::max(b, v);
            } else {
                g = std::cmp::max(g, v);
            }
        }
        r * g * b
    }).sum()
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
