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
        Vec::new()
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
    fn get_max(&self, bp: &Blueprint, cache: &mut HashSet<State>) -> u8 {
        if self.time == 0 {
            self.geodes
        } else if self.can_build(bp).contains(&Robot::Geode) {
            self.step().build(&Robot::Geode, bp).get_max(bp, cache)
        } else {
            let mut result = 0;
            if cache.insert(self.clone()) {
                result = self.step().get_max(bp, cache);
                result = std::cmp::max(
                    result,
                    self.can_build(bp)
                        .iter()
                        .map(|bot| self.step().build(bot, bp).get_max(bp, cache))
                        .max()
                        .unwrap_or_default(),
                );
            }
            result
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
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
        assert_eq!(state.get_max(&bp, &mut cache), 9);
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
        assert_eq!(state.get_max(&bp, &mut cache), 12);
    }
}
