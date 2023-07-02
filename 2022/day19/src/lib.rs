use std::collections::BTreeMap;

struct Cost {
    ore: u8,
    clay: u8,
    obsidian: u8,
}
enum Robot {
    Ore,
    Clay,
    Obsidian,
    Geode,
}
struct Blueprint {
    map: BTreeMap<Robot, Cost>,
}

#[derive(Clone)]
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
    fn build(input: &str) -> Self {}
    fn get_max(&self, blueprint: &Blueprint) -> u8 {}
}
