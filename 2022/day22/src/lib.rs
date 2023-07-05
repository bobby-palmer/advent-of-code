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
enum Direction {
    Up,
    Right,
    Down,
    Left,
}
struct State {
    facing: Direction,
    row: usize,
    col: usize,
    map: Map,
    instructions: &'static str,
}
impl State {
    fn build(input: &'static str) -> Self {
        let (map_str, instructions) = input.split_once("\n\n").unwrap();
        let map = Map::build(map_str);
        let (row, col) = map.get_tl();
        State {
            facing: Direction::Right,
            row,
            col,
            map,
            instructions,
        }
    }
    fn inc_row(&self) -> usize {
        if self.map.0[(self.row + 1).rem_euclid(self.map.0.len())][self.col].is_some() {
            self.row + 1
        } else {
            self.map.0.iter().position(|row| row[self.col].is_some()).unwrap()
        }
    }
    fn dec_row(&self) -> usize {
        if self.map.0[(self.row - 1).rem_euclid(self.map.0.len())][self.col].is_some() {
            self.row - 1
        } else {
            let mut row = self.row;
            while self.map.0[row][self.col].is_some(){
                continue;
            }
            row - 1
        }
    }
    fn inc_col(&self) -> usize {
        if self.map.0[self.row][(self.col + 1)].is_some() {
            self.col + 1
        } else {
            self.map.0.iter().position(|row| row[self.col].is_some()).unwrap()
        }
    }
    fn dec_col(&self) -> usize {

    }
    fn do_move(&mut self, direction: Direction) {
        let (row, col) = match direction {
            Direction::Up => (self.dec_row(), self.col),
            Direction::Down => (self.inc_row(), self.col),
            Direction::Right => (self.row, self.inc_col()),
            Direction::Left => (self.row, self.dec_col()),
        };
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
