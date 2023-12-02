use std::io;
pub fn read(file: &str) -> io::Result<String> {
    let s = std::fs::read_to_string(file)?;
    Ok(s)
}

/**
 *  Takes a function and applies it to each line of the input.
 *  returning the sum of all of these mappings as an i32.
 */
pub fn sum_map(s: &str, f: fn((usize, &str)) -> i32) -> i32 {
    s.lines().enumerate().map(f).sum()
}
