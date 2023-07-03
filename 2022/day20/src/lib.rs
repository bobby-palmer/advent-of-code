const FIRST: usize = 1000;
const SECOND: usize = 2000;
const THIRD: usize = 3000;
const KEY: i64 = 811589153;

#[derive(Debug)]
struct File(Vec<Item>);
impl File {
    fn mix(&mut self) {
        for i in 0..self.0.len() {
            self.mix_one(self.find_index(i));
        }
    }
    fn mix_one(&mut self, index: usize) {
        let item = self.0.remove(index);
        let new_index: usize =
            ((index as i32 + item.value).rem_euclid(self.0.len() as i32)) as usize;
        self.0.insert(new_index, item)
    }
    fn find_index(&self, index: usize) -> usize {
        self.0
            .iter()
            .enumerate()
            .find(|(_, item)| item.index == index)
            .unwrap()
            .0
    }
    fn find_value(&self, value: i32) -> usize {
        self.0
            .iter()
            .enumerate()
            .find(|(_, item)| item.value == value)
            .unwrap()
            .0
    }
    fn build(input: &str) -> Self {
        Self(
            input
                .lines()
                .filter(|line| !line.is_empty())
                .map(|num| num.parse().unwrap())
                .enumerate()
                .map(|pair| Item::build(pair))
                .collect(),
        )
    }
    fn get_coords(&self) -> i32 {
        let base = self.find_value(0);
        let first = (base + FIRST) % self.0.len();
        let second = (base + SECOND) % self.0.len();
        let third = (base + THIRD) % self.0.len();
        self.0[first].value + self.0[second].value + self.0[third].value
    }
}
#[derive(Debug)]
struct Item {
    index: usize,
    value: i32,
}
impl Item {
    fn build(tuple: (usize, i32)) -> Self {
        Item {
            index: tuple.0,
            value: tuple.1,
        }
    }
}
pub fn solve(input: &str) -> i32 {
    let mut file = File::build(input);
    file.mix();
    file.get_coords()
}
pub fn solve_b(input: &str) -> i64 {
    let mut file = File::build(input);
    for _ in 0..10 {
        file.mix();
    }
    file.get_coords() as i64 * KEY
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn finder() {
        let input = include_str!("../testinput");
        let file = File::build(input);
        assert_eq!(file.find_index(0), 0);
        assert_eq!(file.find_index(1), 1);
        assert_eq!(file.find_index(2), 2);
        assert_eq!(file.find_index(3), 3);
        assert_eq!(file.find_index(4), 4);
    }
    #[test]
    fn unit_mixer() {
        let input = include_str!("../testinput");
        let mut file = File::build(input);
        file.mix_one(0);
        assert_eq!(file.find_index(0), 1);
    }
    #[test]
    fn mix_whole() {
        let input = include_str!("../testinput");
        let mut file = File::build(input);
        file.mix();
        let example = vec![-2, 1, 2, -3, 4, 0, 3];
        assert!(file
            .0
            .iter()
            .zip(example.iter())
            .all(|(act, ex)| { act.value == *ex }));
    }
    #[test]
    fn decrypt() {
        let input = include_str!("../testinput");
        let mut file = File::build(input);
        file.mix();
        assert_eq!(file.get_coords(), 3);
    }
    #[test]
    fn decrypt_10() {
        let input = include_str!("../testinput");
        assert_eq!(solve_b(input), 1623178306);
    }
}
