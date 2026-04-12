#[allow(unused_imports)]
use std::io::{self, Write, BufRead};

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let stdin = io::stdin();
        let line = read_input().expect("Failed to read input");
        let trimmed = line.trim();
        print!("{}: command not found", trimmed);
    }
}

fn read_input() -> Result<String, io::Error> {
    let mut line = String::new();
    io::stdin().lock().read_line(&mut line)?;
    Ok(line)
}