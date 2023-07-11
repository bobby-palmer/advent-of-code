#[derive(Debug, PartialEq, Eq)]
enum Spot {
    Open,
    Blocked,
}
struct Map(Vec<Vec<Option<Spot>>>);
impl Map {
    fn build(input: &'static str) -> Self {
            let map = input.lines().map(|line| {
            line.chars().map(|letter| {
                match letter {
                    ' ' => None,
                    '.' => Some(Spot::Open),
                    '#' => Some(Spot::Blocked),
                    _ => unreachable!(),
                }
            }).collect()
        }).collect();

        Map(map)
    }
    fn get_tl(&self) -> (usize, usize) {
        (0, self.0[0].iter().position(|spot| *spot == Some(Spot::Open)).unwrap())
    }
    fn height(&self) -> usize {
        self.0.len()
    }
    fn width(&self) -> usize {
        self.0[0].len()
    }
    fn get(&self, row:usize, col:usize) -> Option<&Spot> {
        match self.0.get(row).unwrap().get(col) {
            None => None,
            Some(None) => None,
            Some(x) => x.as_ref(),
        }
    }
}
#[derive(Debug,PartialEq, Eq, Clone)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}
impl Direction {
    fn turn_right(self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
    fn turn_left(&self) -> Self {
        match self {
            Direction::Up => Direction::Left,
            Direction::Right => Direction::Up,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
        }
    }
    fn to_num(&self) -> usize {
        match self {
            Direction::Up => 3,
            Direction::Right => 0,
            Direction::Down => 1,
            Direction::Left => 2,
        }
    }
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
    fn do_move(&mut self, direction: &Direction) {
        println!("{}", self.col);
        let (row, col) = match direction {
            Direction::Up => {
                let mut row = self.row.checked_sub(1).unwrap_or_else(|| self.map.height() - 1);
                while self.map.get(row, self.col).is_none() {
                    row = row.checked_sub(1).unwrap_or_else(|| self.map.height() - 1);
                }
                match self.map.get(row,self.col) {
                    Some(Spot::Open) => (row, self.col),
                    Some(Spot::Blocked) => (self.row, self.col),
                    _ => unreachable!(),
                }
            },
            Direction::Down => {
                let mut row = (self.row + 1).rem_euclid(self.map.height());
                while self.map.get(row, self.col).is_none() {
                    row = (row + 1).rem_euclid(self.map.height());
                }
                match self.map.get(row, self.col) {
                    Some(Spot::Open) => (row, self.col),
                    Some(Spot::Blocked) => (self.row, self.col),
                    _ => unreachable!(),
                }
            },
            Direction::Right => {
                let mut col = (self.col + 1).rem_euclid(self.map.width());
                while self.map.get(self.row,col).is_none() {
                    col = (col + 1).rem_euclid(self.map.width());
                }
                match self.map.get(self.row,col) {
                    Some(Spot::Open) => (self.row, col),
                    Some(Spot::Blocked) => (self.row, self.col),
                    _ => unreachable!(),
                }
            },
            Direction::Left => {
                let mut col = self.col.checked_sub(1).unwrap_or_else(|| self.map.width());
                while self.map.get(self.row,col).is_none() {
                    col = col.checked_sub(1).unwrap_or_else(|| self.map.width() - 1);
                }
                match self.map.get(self.row,col) {
                    Some(Spot::Open) => (self.row, col),
                    Some(Spot::Blocked) => (self.row, self.col),
                    _ => unreachable!(),
                }
            },
        };
        self.row = row;
        self.col = col;
    }

    fn follow(&mut self) {
        let mut counts: Vec<usize> = self.instructions.trim_end().split(|c: char| c == 'R' || c == 'L' ).map(|num| num.parse::<usize>().unwrap()).rev().collect();
        let mut turns: Vec<char> = self.instructions.trim_end().chars().rev().filter(|c| !c.is_digit(10)).collect();
        while let Some(amount) = counts.pop() {
            for _ in 0..amount {

                self.do_move(&self.facing.clone());
            }
            if let Some(letter) = turns.pop() {
                match letter {
                    'R' => self.turn_right(),
                    'L' => self.turn_left(),
                    _ => unreachable!(),
                }
            }
        }
    }
    fn turn_right(&mut self) {
        self.facing = self.facing.clone().turn_right();
    }
    fn turn_left(&mut self) {
        self.facing = self.facing.turn_left();
    }
}
pub fn solve(input: &'static str) -> usize {
    let mut state = State::build(input);
    state.follow();
    1000 * (state.row + 1) + 4 * (state.col + 1) + state.facing.to_num()
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
    #[test]
    fn test_moving() {
        let input = include_str!("../testinput");
        let mut state = State::build(input);
        state.do_move(&Direction::Left);
        assert_eq!((state.row, state.col),(0, 8));
        state.do_move(&Direction::Right);
        assert_eq!((state.row,state.col),(0, 9));
        state.do_move(&Direction::Right);
        assert_eq!((state.row,state.col),(0, 10));
        state.do_move(&Direction::Right);
        assert_eq!((state.row,state.col),(0, 10));

        state.do_move(&Direction::Down);
        assert_eq!(state.row, 1);
        state.do_move(&Direction::Up);
        state.do_move(&Direction::Up);
        assert_eq!(state.row, 11);
    }
    #[test]
    fn test_turning() {
        assert_eq!(Direction::Up.turn_right(),Direction::Right);
        assert_eq!(Direction::Up.turn_left(),Direction::Left);
    }
    #[test]
    fn test_simulate() {
        let input = include_str!("../testinput");
        let mut state = State::build(input);
        state.follow();
        assert_eq!(state.facing.to_num(), 0);
        assert_eq!(state.col, 7);
        assert_eq!(state.row, 5);
    }
    #[test]
    fn test_solver() {
        let input = include_str!("../testinput");
        assert_eq!(solve(input), 6032);
    }
    #[test]
    fn test_actual() {
        let input = include_str!("../input");
        assert_eq!(solve(input), 89224);
    }
}
