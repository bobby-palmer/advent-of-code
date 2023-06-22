use std::collections::{HashMap, HashSet};

pub struct Network(HashMap<String, Valve>);
impl Network {
    pub fn from_input(input: &'static str) -> Self {
        let valves: HashMap<String, Valve> = input
            .lines()
            .map(move |line| {
                let name = &line[6..=7];
                let valve = Valve::parse(line.split_once('=').unwrap().1);
                (name.to_string(), valve)
            })
            .collect();
        Network(valves)
    }
}

#[derive(Clone)]
pub struct State<'a> {
    valves: &'a Network,
    current: &'a str,
    released: u32,
    flow_rate: u32,
    time_left: u8,
    opened: HashSet<&'a str>,
}
impl<'a> State<'a> {
    pub fn parse(timelimit: u8, valves: &'a Network) -> Self {
        let current = "AA";
        let released = 0;
        let flow_rate = 0;
        State {
            valves,
            current,
            released,
            flow_rate,
            time_left: timelimit,
            opened: HashSet::new(),
        }
    }
    fn current(&self) -> &Valve {
        self.valves.0.get(self.current).unwrap()
    }
    fn dist_to(&self, other: &str) -> u32 {
        if self
            .current()
            .neighbors
            .iter()
            .filter(|name| **name == other)
            .next()
            == None
        {
            1
        } else {
            1 + self
                .current()
                .neighbors
                .iter()
                .map(|name| self.dist_to(name))
                .min()
                .unwrap()
        }
    }
    // returns a copy of the current cave with the move
    // applied to it
    fn do_move(&self, valve: &'a str) -> Self {
        let mut next = self.clone();
        let moves = self.dist_to(valve) + 1;
        if moves > self.time_left.into() {
            next.time_left = 0;
            next.released += self.flow_rate * self.time_left as u32;
        } else {
            next.time_left = self.time_left - moves as u8;
            next.flow_rate += self.valves.0.get(valve).unwrap().rate;
            next.released += self.flow_rate * moves;
            next.current = valve.clone();
            next.opened.insert(next.current);
        }
        next
    }
    fn get_moves(&self) -> Vec<&str> {
        self.valves
            .0
            .iter()
            .filter(|(name, valve)| {
                (!self.opened.contains(&name[..])) && (valve.rate > 0) && (**name != self.current)
            })
            .map(|(name, _)| &name[..])
            .collect()
    }
    pub fn simulate(&self) -> u32 {
        println!("turn num : {}", self.time_left);
        let actions = self.get_moves();
        if self.time_left == 0 {
            self.released
        } else if actions.len() == 0 {
            self.time_left as u32 * self.flow_rate
        } else {
            actions
                .iter()
                .map(|action| {
                    let new = Box::new(self.do_move(action.to_owned()));
                    new.simulate()
                })
                .max()
                .unwrap()
        }
    }
}
struct Valve {
    neighbors: Vec<&'static str>,
    rate: u32,
}
impl Valve {
    fn parse(input: &'static str) -> Self {
        let rate: u32 = input.split_once(';').unwrap().0.parse().unwrap();
        let neighbors: Vec<&str> = input
            .split_once("valves ")
            .unwrap_or_else(|| input.split_once("valve ").unwrap())
            .1
            .split(',')
            .map(|each| each.trim())
            .collect();
        Valve { rate, neighbors }
    }
}
