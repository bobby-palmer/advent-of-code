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
    0
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn getter() {
        let input = include_str!("../testinput");
        let example = "hmdt - zczc";
        assert_eq!(example, get(input, "drzm"));
    }
}

