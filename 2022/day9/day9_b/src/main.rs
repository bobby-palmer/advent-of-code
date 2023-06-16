use std::{borrow::BorrowMut, collections::BTreeSet};

const LENGTH: u8 = 10;

fn main() {
    let mut rope = Rope::new();
    let input = include_str!("../../input.in");
    let mut visited = BTreeSet::new();
    input
        .lines()
        .filter(|line| !line.is_empty())
        .for_each(|line| {
            let (direction, count) = line.split_once(' ').unwrap();
            let vec = match direction {
                "U" => Vector(0, 1),
                "D" => Vector(0, -1),
                "L" => Vector(-1, 0),
                "R" => Vector(1, 0),
                _ => Vector(0, 0), // error
            };
            for _ in 0..count.parse().unwrap() {
                rope.pull(&vec);
                visited.insert(rope.get_last());
            }
        });
    let solution = visited.len();
    println!("found solution : {solution}");
}
struct Knot(i32, i32);
impl Knot {
    fn new() -> Knot {
        Knot(0, 0)
    }
    fn vector_to(&self, other: &Knot) -> Vector {
        let &Knot(x, y) = other;
        Vector(x - self.0, y - self.1)
    }
    fn translate(&mut self, direction: &Vector) {
        let Vector(x, y) = direction;
        self.0 += x;
        self.1 += y;
    }
    fn stretched(&self, other: &Knot) -> bool {
        let &Knot(x, y) = other;
        match (self.0 - x, self.1 - y) {
            (2, _) => true,
            (_, 2) => true,
            (-2, _) => true,
            (_, -2) => true,
            _ => false,
        }
    }
    fn to_tuple(&self) -> (i32, i32) {
        (self.0, self.1)
    }
    fn follow(&mut self, other: &Knot) {
        if self.stretched(other) {
            let mut vec = self.vector_to(other);
            vec.unitize();
            self.translate(&vec);
        }
    }
}
struct Vector(i32, i32);
impl Vector {
    // maintains the vectors general direction but reduces magnitude to at most one for each
    // component
    fn unitize(&mut self) {
        self.0 = self.0.signum();
        self.1 = self.1.signum();
    }
}
struct Rope(Vec<Knot>);
impl Rope {
    fn new() -> Self {
        let mut rope = Vec::new();
        for _ in 0..LENGTH {
            rope.push(Knot::new());
        }
        Rope(rope)
    }
    fn pull(&mut self, direction: &Vector) {
        let mut iter = self.0.iter_mut();
        let mut prev = iter.next().unwrap();
        prev.translate(direction);

        for knot in iter {
            knot.follow(prev);
            prev = knot;
        }
    }
    fn get_last(&self) -> (i32, i32) {
        self.0.last().unwrap().to_tuple()
    }
}
