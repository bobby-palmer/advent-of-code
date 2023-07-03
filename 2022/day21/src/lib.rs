fn get<'a>(input: &'a str, name: &str) -> &'a str {
    input
        .lines()
        .find(|line| {
            let (thisname, _) = line.split_at(4);
            thisname == name
        })
        .unwrap()
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn getter() {
        let input = include_str!("../testinput");
        let example = "drzm: hmdt - zczc";
        assert_eq!(example, get(input, "drzm"));
    }
}

