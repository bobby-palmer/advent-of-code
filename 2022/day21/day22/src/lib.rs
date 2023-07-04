#[derive(Debug, PartialEq, Eq)]
enum Spot {
    Open,
    Blocked,
}
struct Map(Vec<Vec<Option<Spot>>>);
impl Map {
    fn build(input: &'static str) -> Self {
            Map(input.lines().map(|line| {
            line.chars().map(|letter| {
                match letter {
                    ' ' => None,
                    '.' => Some(Spot::Open),
                    '#' => Some(Spot::Blocked),
                    _ => unreachable!(),
                }
            }).collect()
        }).collect())
    }
    fn get_tl(&self) -> (usize, usize) {
        (0, self.0[0].iter().position(|spot| *spot == Some(Spot::Open)).unwrap())
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_build() {
        let input = include_str!("../testinput").split_once("\n\n").unwrap().0;
        let map = Map::build(input);
        assert_eq!(map.0[0][0], None);
        assert_eq!(map.0[4][0], Some(Spot::Open));
        assert_eq!(map.0[4][3], Some(Spot::Blocked));
    }
    #[test]
    fn test_gettl() {
        let input = include_str!("../testinput").split_once("\n\n").unwrap().0;
        let map = Map::build(input);
        assert_eq!(map.get_tl(), (0,8));
    }
}
