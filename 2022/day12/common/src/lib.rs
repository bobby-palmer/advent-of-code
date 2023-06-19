use std::{
    collections::{BTreeSet, VecDeque},
    u32::MAX,
};

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct Position(usize, usize); // row, col
impl Position {
    fn get_surroundings(&self) -> Vec<Position> {
        let Position(row, col) = *self;
        let row_signed = row as i32;
        let col_signed = col as i32;
        [
            Position::new(row_signed - 1, col_signed),
            Position::new(row_signed + 1, col_signed),
            Position::new(row_signed, col_signed - 1),
            Position::new(row_signed, col_signed + 1),
        ]
        .into_iter()
        .filter(|x| x.is_some())
        .map(|x| x.unwrap())
        .collect()
    }
    fn new(row: i32, col: i32) -> Option<Self> {
        if (row < 0) || (col < 0) {
            None
        } else {
            Some(Self(row as usize, col as usize))
        }
    }
}
pub struct Board {
    start: Position,
    end: Position,
    board: Vec<&'static str>,
}
impl Board {
    pub fn new(input: &'static str) -> Self {
        let board: Vec<&str> = input
            .split(|letter| letter == '\n')
            .filter(|line| !line.is_empty())
            .collect();
        let start = find_letter(&board, 'S').pop().unwrap();
        let end = find_letter(&board, 'E').pop().unwrap();
        Board { start, end, board }
    }
    fn get_surroundings(&self, spot: &Position) -> Vec<Position> {
        let cur_height = self
            .get_height(spot)
            .expect("called get surroundings on invalid spot!");
        spot.get_surroundings()
            .into_iter()
            .filter(|neighbor| !(self.get_height(neighbor).unwrap_or_else(|| MAX) > cur_height + 1))
            .collect()
    }
    fn get_height(&self, spot: &Position) -> Option<u32> {
        let &Position(row, col) = spot;
        if let Some(line) = self.board.get(row) {
            if let Some(letter) = line.chars().nth(col) {
                Some(match letter {
                    'S' => 'a' as u32,
                    'E' => 'z' as u32,
                    _ => letter as u32,
                })
            } else {
                None
            }
        } else {
            None
        }
    }
    fn shortest_path(&self, start: &Position, end: &Position) -> u32 {
        let mut queue = Vec::new();
        let mut next_queue = BTreeSet::new();
        let mut distance = 0;
        queue.push(*start);
        while !queue.is_empty() {
            while let Some(spot) = queue.pop() {
                if &spot == end {
                    return distance;
                }
                let mut neighbors = self.get_surroundings(&spot);
                while let Some(neighbor) = neighbors.pop() {
                    next_queue.insert(neighbor);
                }
            }
            distance += 1;
            while let Some(spot) = next_queue.pop_first() {
                queue.push(spot);
            }
        }
        return 0;
    }
    fn shortest_trail(&self) -> u32 {
        let starts = find_letter(&self.board, 'a');
        let shortest_one = starts
            .iter()
            .map(|spot| self.shortest_path(spot, &self.end))
            .min();
        shortest_one.unwrap()
    }
    pub fn get_solution_a(&self) -> u32 {
        self.shortest_path(&self.start, &self.end)
    }
    pub fn get_solution_b(&self) -> u32 {
        std::cmp::min(self.get_solution_a(), self.shortest_trail())
    }
}
fn find_letter(haystack: &Vec<&str>, needle: char) -> Vec<Position> {
    haystack
        .iter()
        .enumerate()
        .filter_map(|(row, line)| {
            if let Some(col) = line.chars().position(|letter| letter == needle) {
                Some(Position(row, col))
            } else {
                None
            }
        })
        .collect()
}
