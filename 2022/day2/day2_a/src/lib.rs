#[derive(PartialEq, Clone)]
pub enum Move {
    Rock,
    Paper,
    Scissors,
}
#[derive(PartialEq, Clone)]
pub enum Outcome {
    Win,
    Tie,
    Loss,
}
pub struct Round{
    yours: Move,
    opponents: Move,
}
impl Round {
    pub fn score_round(&self) -> u32 {
        (match Self::play_round(self) {
            Outcome::Win => 6,
            Outcome::Loss => 0,
            Outcome::Tie => 3,
        } + self.yours.base_score())
    }
    fn play_round(&self) -> Outcome {
        play_round(&self.yours, &self.opponents)
    }
    pub fn new(moves: &str) -> Round {
        let mut moves = moves.chars().filter_map(|letter| {
            get_move(&letter)
        });
        Round{
            opponents: moves.next().unwrap(),
            yours: moves.next().unwrap(),
        }
    }
    pub fn from_fields(mine: Move, opponents: Move) -> Round {
        Round {
            yours: mine,
            opponents,
        }
    }   
}
impl Move {
    fn base_score(&self) -> u32 {
        match self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        }
    }
}
fn get_move(input: &char) -> Option<Move> {
    match input {
        'A' | 'X' => Some(Move::Rock),
        'B' | 'Y' => Some(Move::Paper),
        'C' | 'Z' => Some(Move::Scissors),
        _ => None,
    }
}
pub fn play_round(your_move: &Move, opponent: &Move) -> Outcome {
    if your_move == opponent {
        Outcome::Tie
    } else {
        match (your_move, opponent) {
            (Move::Rock, Move::Paper) => Outcome::Loss,
            (Move::Paper, Move::Scissors) => Outcome::Loss,
            (Move::Scissors, Move::Rock) => Outcome::Loss,
            _ => Outcome::Win,
        }
    }
}
