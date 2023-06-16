use std::collections::BTreeSet;

fn main() {
    let input = include_str!("../../input.in");
    let mut head = End(0, 0);
    let mut tail = End(0, 0);
    let mut visited = BTreeSet::new();
    visited.insert(tail.to_tuple());
    input.lines().for_each(|line| {
        let (direction, count) = line.split_once(' ').unwrap();
        let dir_vec = match direction {
            "U" => Vector(0, 1),
            "D" => Vector(0, -1),
            "L" => Vector(-1, 0),
            "R" => Vector(1, 0),
            _ => Vector(0, 0), // error
        };
        for _ in 0..(count.parse().unwrap()) {
            head.translate(&dir_vec);

            // only move if stretched
            if head.stretched(&tail) {
                let mut dir_vec = tail.vector_to(&head);
                dir_vec.unitize();
                tail.translate(&dir_vec);
                visited.insert(tail.to_tuple());
            }
        }
    });
    let solution = visited.len();
    println!("found solution : {solution}");
}
struct End(i32, i32);
impl End {
    fn vector_to(&self, other: &End) -> Vector {
        let &End(x, y) = other;
        Vector(x - self.0, y - self.1)
    }
    fn translate(&mut self, direction: &Vector) {
        let Vector(x, y) = direction;
        self.0 += x;
        self.1 += y;
    }
    fn stretched(&self, other: &End) -> bool {
        let &End(x, y) = other;
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
