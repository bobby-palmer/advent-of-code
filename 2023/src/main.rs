use std::io;

mod util;
mod one;
mod two;

fn main() -> std::io::Result<()> {
    let mut input = String::new();
    let stdin = io::stdin();
    println!("Which day do you want to run? (ie. 'one' will solve day one, part a and b)");
    stdin.read_line(&mut input)?;

    match input.trim() {
        "one" => one::solve("day1.txt")?,
        "two" => two::solve()?,
        x => println!("{x} is not implemented yet!"),
    }
    Ok(())
}
