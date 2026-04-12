use std::io::{self, Write, BufRead};
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;

enum Builtin {
    Exit,
    Echo,
    Type
}

#[derive(PartialEq, Eq)]
enum CommandType {
    Builtin,
    External(PathBuf),
    NotFound
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
                let Some(parsed_args) = args.first() else {
                    println!("type: missing argument");
                    return false
                };
                let cmd_type = resolve_command(parsed_args);

                if cmd_type == CommandType::Builtin {
                    println!("{} is a shell builtin", parsed_args);
                } else if let CommandType::External(path) = cmd_type {
                    println!("{} is {}", parsed_args, path.display());
                } else {
                    println!("{}: not found", parsed_args);
                }
                false
            }
        }
    }
}

fn resolve_command(name: &str) -> CommandType {
    if Builtin::parse(name).is_some() {
        CommandType::Builtin
    } else if let Some(path) = resolve_external(name) {
        CommandType::External(path)
    } else {
        CommandType::NotFound
    }
}
fn resolve_external(cmd: &str) -> Option<PathBuf> {
    if cmd.is_empty() { return None }
    let path_env = std::env::var_os("PATH");
    let all_paths = std::env::split_paths(&path_env.unwrap_or_default()).collect::<Vec<_>>();
    for path in all_paths {
        let candidate = path.join(cmd);
        if !candidate.is_file(){
            continue
        }
        let meta = std::fs::metadata(&candidate).ok()?;
        let mode = meta.permissions().mode();
        if mode & 0o111 != 0 {
            return Some(candidate);
        }
    }
    None
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
            None => println!("{}: not found", command)
        }
    }
}

fn read_input() -> Result<String, io::Error> {
    let mut line = String::new();
    io::stdin().lock().read_line(&mut line)?;
    Ok(line)
}