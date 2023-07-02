use std::collections::HashSet;

#[derive(PartialEq, Eq)]
enum Robot {
    Ore,
    Clay,
    Obsidian,
    Geode,
}
struct Blueprint {
    id: u8,
    ore: u8,
    clay: u8,
    obsidian: (u8, u8),
    geode: (u8, u8),
}
impl Blueprint {
    fn build(input: &str) -> Vec<Self> {
        input
            .lines()
            .filter(|line| !line.is_empty())
            .map(|line| {
                let mut tokens = line.split_whitespace();
                let id = trim_end(tokens.nth(1).unwrap()).parse().unwrap();
                Blueprint {
                    id,
                    ore: tokens.nth(4).unwrap().parse().unwrap(),
                    clay: tokens.nth(5).unwrap().parse().unwrap(),
                    obsidian: (
                        tokens.nth(5).unwrap().parse().unwrap(),
                        tokens.nth(2).unwrap().parse().unwrap(),
                    ),
                    geode: (
                        tokens.nth(5).unwrap().parse().unwrap(),
                        tokens.nth(2).unwrap().parse().unwrap(),
                    ),
                }
            })
            .collect()
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
struct State {
    time: u8,
    orebots: u8,
    claybots: u8,
    obsidianbots: u8,
    geodebots: u8,
    ore: u8,
    clay: u8,
    obsidian: u8,
    geodes: u8,
}
impl State {
    fn new(time: u8) -> Self {
        Self {
            time,
            orebots: 1,
            claybots: 0,
            obsidianbots: 0,
            geodebots: 0,
            ore: 0,
            clay: 0,
            obsidian: 0,
            geodes: 0,
        }
    }
    fn can_build(&self, bp: &Blueprint) -> Vec<Robot> {
        let mut result = Vec::new();
        if self.ore >= bp.ore {
            result.push(Robot::Ore);
        }
        if self.ore >= bp.clay {
            result.push(Robot::Clay);
        }
        {
            let (ore, clay) = bp.obsidian;
            if self.ore >= ore && self.clay >= clay {
                result.push(Robot::Obsidian);
            }
        }
        {
            let (ore, obsidian) = bp.geode;
            if self.ore >= ore && self.obsidian >= obsidian {
                result.push(Robot::Geode);
            }
        }
        result
    }
    fn build(mut self, robot: &Robot, bp: &Blueprint) -> Self {
        match robot {
            Robot::Ore => {
                self.ore -= bp.ore;
                self.orebots += 1;
            }
            Robot::Clay => {
                self.ore -= bp.clay;
                self.claybots += 1;
            }
            Robot::Obsidian => {
                let (ore, clay) = bp.obsidian;
                self.ore -= ore;
                self.clay -= clay;
                self.obsidianbots += 1;
            }
            Robot::Geode => {
                let (ore, obsidian) = bp.geode;
                self.ore -= ore;
                self.obsidian -= obsidian;
                self.geodebots += 1;
            }
        }
        self
    }
    fn step(&self) -> Self {
        let mut new = self.clone();
        new.time -= 1;
        new.ore += new.orebots;
        new.clay += new.claybots;
        new.obsidian += new.obsidianbots;
        new.geodes += self.geodebots;
        new
    }
    fn can_beat(&self, score: u8) -> bool {
        let max_future: u16 =
            (self.geodebots * self.time) as u16 + self.time as u16 / 2 * (self.time as u16 - 1);
        max_future + self.geodes as u16 > score.into()
    }
    fn get_max(&self, bp: &Blueprint, cache: &mut HashSet<State>, best: &mut u8) {
        if self.time == 0 {
            if self.geodes > *best {
                *best = self.geodes;
            }
        } else if !self.can_beat(*best) {
            return;
        } else if self.can_build(bp).contains(&Robot::Geode) {
            self.step()
                .build(&Robot::Geode, bp)
                .get_max(bp, cache, best);
        } else {
            if cache.insert(self.clone()) {
                self.step().get_max(bp, cache, best);
                self.can_build(bp)
                    .iter()
                    .for_each(|bot| self.step().build(bot, bp).get_max(bp, cache, best));
            }
        }
    }
}
fn trim_end(input: &str) -> &str {
    &input[0..input.len() - 1]
}
pub fn solve(input: &str) -> u16 {
    let state = State::new(24);
    Blueprint::build(input)
        .iter()
        .map(|bp| {
            let mut cache = HashSet::new();
            let mut best = 0;
            state.get_max(bp, &mut cache, &mut best);
            (best * bp.id) as u16
        })
        .sum()
}
pub fn solveb(input: &str) -> u16 {
    let state = State::new(32);
    Blueprint::build(input)
        .iter()
        .enumerate()
        .filter(|(index, _)| *index < 3)
        .map(|(_, bp)| {
            let mut cache = HashSet::new();
            let mut best = 0;
            state.get_max(bp, &mut cache, &mut best);
            best as u16
        })
        .product()
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn trimming() {
        let input = "blah:";
        assert_eq!(trim_end(input), "blah");
    }
    #[test]
    fn build_bp() {
        let input = include_str!("../testinput");
        let bps = Blueprint::build(input);
        assert_eq!(bps.len(), 2);
    }
    #[test]
    fn bp_values() {
        let input = include_str!("../testinput");
        let bps = Blueprint::build(input);
        let bp = &bps[0];
        assert_eq!(bp.ore, 4);
        assert_eq!(bp.clay, 2);
        assert_eq!(bp.obsidian, (3, 14));
        assert_eq!(bp.geode, (2, 7));
    }
    #[test]
    fn get_max() {
        let bp = Blueprint {
            id: 1,
            ore: 4,
            clay: 2,
            obsidian: (3, 14),
            geode: (2, 7),
        };
        let state = State::new(24);
        let mut cache = HashSet::new();
        let mut best = 0;
        state.get_max(&bp, &mut cache, &mut best);
        assert_eq!(best, 9);
    }
    #[test]
    fn get_max2() {
        let bp = Blueprint {
            id: 1,
            ore: 2,
            clay: 3,
            obsidian: (3, 8),
            geode: (3, 12),
        };
        let state = State::new(24);
        let mut cache = HashSet::new();
        let mut best = 0;
        state.get_max(&bp, &mut cache, &mut best);
        assert_eq!(best, 12);
    }
    #[test]
    fn solve_test() {
        let input = include_str!("../testinput");
        assert_eq!(solve(input), 33);
    }
}
