use std::io;
pub fn read(file: &str) -> io::Result<String> {
    let s = std::fs::read_to_string(file)?;
    Ok(s)
}
