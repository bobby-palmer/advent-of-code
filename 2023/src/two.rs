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
