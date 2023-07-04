use std::collections::HashSet;

fn get<'a>(input: &'a str, name: &str) -> &'a str {
    input
        .lines()
        .find(|line| {
            let (thisname, _) = line.split_at(4);
            thisname == name
        })
        .unwrap().split_at(6).1
}

fn parse(input: &str, name: &str) -> i64 {
    let val = get(input, name);
    if let Some(num) = val.parse().ok() {
        num
    } else {
        let (lhs, rest) = val.split_once(' ').unwrap();
        let (oper, rhs) = rest.split_once(' ').unwrap();
        let leftval = parse(input, lhs);
        let rightval = parse(input, rhs);
        match oper {
            "*" => leftval * rightval,
            "+" => leftval + rightval,
            "-" => leftval - rightval,
            "/" => leftval / rightval,
            _ => unreachable!(),
        }
    }
}
fn contains<'a>(input: &'static str, head: &'a str, name: &str, set: &mut HashSet<&'a str>) {
    let line = get(input, head);
    if name == head {
        set.insert(head);
    } else if line.parse::<i32>().is_ok() {
        return;
    } else {
        let (lhs, rest) = line.split_once(' ').unwrap();
        let (_, rhs) = rest.split_once(' ').unwrap();
        contains(input, lhs, name, set);
        contains(input, rhs, name, set);
        if set.contains(lhs) || set.contains(rhs) {
            set.insert(head);
        }
    }
}
fn reverse(input: &str, mut constant: i64, var_head: &str, var_name: &str, branch: &HashSet<&str>) -> i64 {
    if var_head == var_name {
        constant
    } else {
        let val = get(input, var_head);
        let (lhs, rest) = val.split_once(' ').unwrap();
        let (oper, rhs) = rest.split_once(' ').unwrap();
        let var_head;
        if branch.contains(lhs) {
            var_head = lhs;
            let value = parse(input, rhs);
            constant = match oper {
                "*" => constant / value,
                "+" => constant - value,
                "-" => constant + value,
                "/" => constant * value,
                _ => unreachable!(),
            };
        } else {
            var_head = rhs;
            let value = parse(input, lhs);
            constant = match oper {
                "*" => constant / value,
                "+" => constant - value,
                "-" => value - constant,
                "/" => value / constant,
                _ => unreachable!(),
            };
        }
        reverse(input, constant, var_head, var_name, branch)
    }
}
pub fn solve() {
    let input = include_str!("../input");
    let solution = parse(input, "root");
    println!("{solution}");
    let solution_b = solve_b(input);
    println!("found solution b : {solution_b}");

}
pub fn solve_b(input: &'static str) -> i64 {
    let line = get(input, "root");
    let (lhs, rest) = line.split_once(' ').unwrap();
    let (_, rhs) = rest.split_once(' ').unwrap();
    let mut set = HashSet::new();
    contains(input, "root", "humn", &mut set);
    let result = if set.contains(lhs) {
        let constant = parse(input, rhs);
        reverse(input, constant, lhs, "humn", &set)
    } else {
        let constant = parse(input, lhs);
        reverse(input, constant, rhs, "humn", &set)
    };
    result
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn get_func() {
        let input = include_str!("../testinput");
        let example = "hmdt - zczc";
        assert_eq!(example, get(input, "drzm"));
    }
    #[test]
    fn get_num() {
        let input = include_str!("../testinput");
        let example = "32";
        assert_eq!(example, get(input, "hmdt"));
    }
    #[test]
    fn parsenum() {
        let input = include_str!("../testinput");
        assert_eq!(parse(input, "hmdt"), 32);
    }
    #[test]
    fn parse_mindepth() {
        let input = include_str!("../testinput");
        assert_eq!(parse(input, "drzm"), 30);
    }
    #[test]
    fn parseroot() {
        let input = include_str!("../testinput");
        let solution = parse(input, "root");
        assert_eq!(solution, 152);
    }
    #[test]
    fn get_containing() {
        let input = include_str!("../testinput");
        let mut set = HashSet::new();
        contains(input, "root", "humn", &mut set);
        assert!(set.contains("humn"));
        assert!(set.contains("ptdq"));
        assert!(set.contains("lgvd"));
        assert!(set.contains("cczh"));
        assert!(set.contains("pppw"));
        assert!(set.contains("root"));
        assert_eq!(set.len(),6);
    }
    #[test]
    fn solve_pt2() {
        let input = include_str!("../testinput");
        assert_eq!(solve_b(input),301);
    }
}

