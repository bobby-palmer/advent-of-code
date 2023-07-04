fn get<'a>(input: &'a str, name: &str) -> &'a str {
    input
        .lines()
        .find(|line| {
            let (thisname, _) = line.split_at(4);
            thisname == name
        })
        .unwrap().split_at(6).1
}

fn parse(input: &str, name: &str) -> i32 {
    let val = get(input, name);
    println!("{val}");
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
}

