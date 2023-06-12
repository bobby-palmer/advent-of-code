use day2_a::Round;
use day2_a::Move;
use day2_a::Outcome;

pub fn get_round(input: &str) -> Round {
   let mut iter = input.chars();

    let opponent_move = get_move(iter.next().unwrap()).unwrap();
    // pass over empty space
    iter.next();
    let desired_outcome = get_outcome(iter.next().unwrap()).unwrap();
    let my_move = suggest_move(&opponent_move, desired_outcome);
    Round::from_fields(my_move, opponent_move)
}

fn get_move(letter: char) -> Option<Move> {
    match letter {
        'A' => Some(Move::Rock),
        'B' => Some(Move::Paper),
        'C' => Some(Move::Scissors),
        _ => None,
    }
}

fn get_outcome(letter: char) -> Option<Outcome> {
    match letter {
        'X' => Some(Outcome::Loss),
        'Y' => Some(Outcome::Tie),
        'Z' => Some(Outcome::Win),
        _ => None
    }
}
fn beat(given: &Move) -> Move {
    match given {
        Move::Rock => Move::Paper,
        Move::Paper => Move::Scissors,
        Move::Scissors => Move::Rock,
    }
}

fn lose_to(given: &Move) -> Move {
    match given {
        Move::Rock => Move::Scissors,
        Move::Paper => Move::Rock,
        Move::Scissors => Move::Paper,
    }
}
fn suggest_move(opponent: &Move, outcome: Outcome) -> Move {
   match outcome {
        Outcome::Tie => opponent.clone(),
        Outcome::Win => beat(opponent),
        Outcome::Loss => lose_to(opponent),
    } 
}
