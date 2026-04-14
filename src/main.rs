use std::io::{self, Write, BufRead};
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;
use std::os::unix::process::CommandExt;

#[derive(PartialEq, Eq)]
enum Builtin {
    Exit,
    Echo,
    Type,
    Pwd
}

#[derive(PartialEq, Eq)]
enum CommandType {
    Builtin(Builtin),
    External(PathBuf),
    NotFound
}


impl Builtin {
    fn parse(name: &str) -> Option<Self> {
        match name {
            "exit" => Some(Builtin::Exit),
            "echo" => Some(Builtin::Echo),
            "type" => Some(Builtin::Type),
            "pwd" => Some(Builtin::Pwd),

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

                match resolve_command(parsed_args) {
                    CommandType::Builtin(_) => println!("{} is a shell builtin", parsed_args),
                    CommandType::External(path) => println!("{} is {}", parsed_args, path.display()),
                    CommandType::NotFound => println!("{}: not found", parsed_args),
                }
                false
            },
            Builtin::Pwd => {
                match std::env::current_dir() {
                    Ok(path) => println!("{}", path.display()),
                    Err(err) => eprintln!("pwd: {}", err),
                }
                false
            }
        }
    }
}

fn resolve_command(name: &str) -> CommandType {
    if let Some(builtin) = Builtin::parse(name) {
        CommandType::Builtin(builtin)
    } else if let Some(path) = resolve_external(name) {
        CommandType::External(path)
    } else {
        CommandType::NotFound
    }
}
fn resolve_external(cmd: &str) -> Option<PathBuf> {
    if cmd.is_empty() { return None }
    let path_env = std::env::var_os("PATH");

    for path in std::env::split_paths(&path_env.unwrap_or_default()) {
        let candidate = path.join(cmd);
        if !candidate.is_file(){
            continue
        }
        let Ok(meta) = std::fs::metadata(&candidate) else { continue };
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

        if command.is_empty() { continue; }

        match resolve_command(command) {
            CommandType::Builtin(builtin) => {
                if builtin.run(&args) {
                    break;
                }
            }
            CommandType::External(path) => {
                let _ = std::process::Command::new(path)
                    .arg0(command)
                    .args(&args)
                    .status();
            }
            CommandType::NotFound => println!("{}: not found", command),
        }
    }
}

fn read_input() -> Result<String, io::Error> {
    let mut line = String::new();
    io::stdin().lock().read_line(&mut line)?;
    Ok(line)
}