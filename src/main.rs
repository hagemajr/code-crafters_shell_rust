#[allow(unused_imports)]
use std::io::{self, Write, BufRead};

enum Builtin {
    Exit,
    Echo,
    Type
}

impl Builtin {
    fn parse(name: &str) -> Option<Self> {
        match name {
            "exit" => Some(Builtin::Exit),
            "echo" => Some(Builtin::Echo),
            "type" => Some(Builtin::Type),
            _ => None
        }
    }

    fn run(&self, args: &[&str]) -> bool {
        match self {
            Builtin::Exit => true,
            Builtin::Echo => {
                println!("{}", args.join(" "));
                false
            },
            Builtin::Type => {
                if Builtin::parse(args[0]).is_some() {
                    println!("{} is a shell builtin", args[0]);
                } else {
                    println!("{}: not found", args[0]);
                }
                false
            }
        }
    }
}

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let line = read_input().expect("Failed to read input");
        let mut parts = line.trim().split_whitespace();
        let command = parts.next().unwrap_or("");
        let args: Vec<&str> = parts.collect();

        match Builtin::parse(command) {
            Some(builtin) => {
                if builtin.run(&args) {
                    break;
                }
            },
            None => println!("{}: command not found", command)
        }
    }
}

fn read_input() -> Result<String, io::Error> {
    let mut line = String::new();
    io::stdin().lock().read_line(&mut line)?;
    Ok(line)
}