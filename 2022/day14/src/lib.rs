use std::collections::HashSet;

enum Fall {
    Down,
    Left,
    Right,
    Stay,
}

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
struct Rock(i32, i32);
impl Rock {
    // returns true if the pebble was actually able to move
    fn make_move(&mut self, direction: Fall) -> bool {
        match direction {
            Fall::Stay => false,
            direction => {
                self.1 += 1;
                match direction {
                    Fall::Left => self.0 -= 1,
                    Fall::Right => self.0 += 1,
                    _ => (),
                }
                true
            }
        }
    }
}
pub struct Cave {
    rocks: HashSet<Rock>,
    cemented: u32,
    pebble: Rock,
    lowest: u32,
}
impl Cave {
    fn generate_move(&self, rock: &Rock) -> Fall {
        let &Rock(x, y) = rock;
        // first check beneath
        if !self.rocks.contains(&Rock(x, y + 1)) {
            Fall::Down
        } else if !self.rocks.contains(&Rock(x - 1, y + 1)) {
            Fall::Left
        } else if !self.rocks.contains(&Rock(x + 1, y + 1)) {
            Fall::Right
        } else {
            Fall::Stay
        }
    }
    fn too_far_gone(&self, rock: &Rock) -> bool {
        let &Rock(_, y) = rock;
        y > self.lowest as i32
    }
    fn spawn_rock(&mut self) {
        self.pebble = Rock(500, 0);
    }
    fn cement(&mut self) {
        self.rocks.insert(self.pebble.clone());
        self.cemented += 1;
        self.spawn_rock();
    }
    fn build_line(input: &str, rocks: &mut HashSet<Rock>) {
        let mut iter = input.split(" -> ");
        let mut prev = iter.next().unwrap();
        while let Some(current) = iter.next() {
            build_range(prev, current).into_iter().for_each(|rock| {
                rocks.insert(rock);
            });
            prev = current;
        }
    }
    pub fn build(input: &str) -> Self {
        let mut rocks = HashSet::new();
        input
            .split('\n')
            .filter(|line| !line.is_empty())
            .for_each(|line| {
                Cave::build_line(line, &mut rocks);
            });
        let lowest = *rocks.iter().map(|Rock(_, y)| y).max().unwrap() as u32;
        Cave {
            rocks,
            cemented: 0,
            pebble: Rock(500, 0),
            lowest,
        }
    }
    pub fn simulate(&mut self) -> u32 {
        loop {
            while self.pebble.make_move(self.generate_move(&self.pebble)) {
                if self.too_far_gone(&self.pebble) {
                    return self.cemented;
                }
            }
            self.cement();
        }
    }
    pub fn simulate_b(&mut self) -> u32 {
        loop {
            while self.pebble.make_move(self.generate_move(&self.pebble)) {
                if self.too_far_gone(&self.pebble) {
                    break;
                }
            }
            if self.pebble == Rock(500, 0) {
                return self.cemented + 1;
            }
            self.cement();
        }
    }
}
fn build_range(start: &str, end: &str) -> Vec<Rock> {
    let (startx, starty) = start.split_once(',').unwrap();
    let (endx, endy) = end.split_once(',').unwrap();
    let mut rocks = Vec::new();
    for x in startx.parse().unwrap()..=endx.parse().unwrap() {
        for y in starty.parse().unwrap()..=endy.parse().unwrap() {
            rocks.push(Rock(x, y));
        }
    }
    for x in endx.parse().unwrap()..=startx.parse().unwrap() {
        for y in endy.parse().unwrap()..=starty.parse().unwrap() {
            rocks.push(Rock(x, y));
        }
    }
    rocks
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build() {
        let input = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";
        let cave = Cave::build(input);
        assert!(cave.rocks.contains(&Rock(498, 4)));
        assert!(cave.rocks.contains(&Rock(498, 5)));
        assert!(cave.rocks.contains(&Rock(498, 6)));
        assert!(cave.rocks.contains(&Rock(497, 6)));
        assert!(cave.rocks.contains(&Rock(496, 6)));

        assert!(cave.rocks.contains(&Rock(503, 4)));
        assert!(cave.rocks.contains(&Rock(502, 4)));
        assert!(cave.rocks.contains(&Rock(502, 5)));
        assert!(cave.rocks.contains(&Rock(502, 6)));
        assert!(cave.rocks.contains(&Rock(502, 7)));
        assert!(cave.rocks.contains(&Rock(502, 8)));
        assert!(cave.rocks.contains(&Rock(502, 9)));
        assert!(cave.rocks.contains(&Rock(501, 9)));
        assert!(cave.rocks.contains(&Rock(500, 9)));
        assert!(cave.rocks.contains(&Rock(499, 9)));
        assert!(cave.rocks.contains(&Rock(498, 9)));
        assert!(cave.rocks.contains(&Rock(497, 9)));
        assert!(cave.rocks.contains(&Rock(496, 9)));
        assert!(cave.rocks.contains(&Rock(495, 9)));
        assert!(cave.rocks.contains(&Rock(494, 9)));
    }
    #[test]
    fn test_size() {
        let input = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";
        let cave = Cave::build(input);
        assert_eq!(cave.rocks.len(), 20);
    }
    #[test]
    fn test_ranges() {
        let rock1 = "498,6";
        let rock2 = "496,6";
        let range = build_range(rock1, rock2);
        assert_eq!(range.len(), 3);
    }
}
