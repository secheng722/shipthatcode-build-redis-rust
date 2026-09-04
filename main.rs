use std::io::{self, BufRead, Write};

fn parse_args(line: &str) -> Vec<String> {
    let mut args = Vec::new();
    let mut current = String::new();
    let mut in_quotes = false;
    for ch in line.chars() {
        match ch {
            '"' if !in_quotes => in_quotes = true,
            '"' if in_quotes => in_quotes = false,
            ' ' if !in_quotes => {
                if !current.is_empty() {
                    args.push(current.clone());
                    current.clear();
                }
            }
            _ => current.push(ch),
        }
    }
    if !current.is_empty() {
        args.push(current);
    }
    args
}

fn eb(s: Option<&str>) -> String {
    if let Some(s) = s {
        format!("${}\r\n{}\r\n", s.len(), s)
    } else {
        "$-1\r\n".into()
    }
}

fn es(s: &str) -> String {
    format!("+{}\r\n", s)
}

fn ee(msg: &str) -> String {
    format!("-{}\r\n", msg)
}

fn ei(n: i64) -> String {
    format!(":{}\r\n", n)
}

#[derive(Debug)]
enum Command {
    Ping,
    Echo,
    Command,
    Unknown,
}

impl Command {
    pub fn from_str(s: &str) -> Result<Self, String> {
        match s.to_uppercase().as_str() {
            "PING" => Ok(Command::Ping),
            "ECHO" => Ok(Command::Echo),
            "COMMAND" => Ok(Command::Command),
            _ => Err(format!("ERR unknown command '{}'", s)),
        }
    }

    pub fn arity(&self) -> Option<(usize, Option<usize>)> {
        match self {
            Command::Ping => Some((0, Some(1))),
            Command::Echo => Some((1, Some(1))),
            Command::Command => Some((0, Some(0))),
            Command::Unknown => None,
        }
    }

    pub fn check_arity(&self, args_len: usize) -> Result<(), String> {
        let name = format!("{:?}", self).to_uppercase();
        if let Some((min, max)) = self.arity() {
            if args_len < min {
                return Err(format!(
                    "ERR wrong number of arguments for '{}' command",
                    name
                ));
            }
            if let Some(max) = max {
                if args_len > max {
                    return Err(format!(
                        "ERR wrong number of arguments for '{}' command",
                        name
                    ));
                }
            }
            Ok(())
        } else {
            Err(format!("ERR unknown command {}", name))
        }
    }
}

fn handle_command(args: &[String]) -> String {
    let cmd = match Command::from_str(&args[0]) {
        Ok(cmd) => cmd,
        Err(err) => return ee(&err),
    };

    if let Err(err) = cmd.check_arity(args.len() - 1) {
        return ee(&err);
    }

    match cmd {
        Command::Ping => {
            if let Some(msg) = args.get(1) {
                eb(Some(msg))
            } else {
                es("PONG")
            }
        }
        Command::Echo => eb(Some(args[1].as_str())),
        Command::Command => es("OK"),
        Command::Unknown => unreachable!(),
    }
}

fn main() {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut out = stdout.lock();

    for line in stdin.lock().lines() {
        let line = line.unwrap();
        let line = line.trim().to_string();
        if line.is_empty() {
            continue;
        }
        let args = parse_args(&line);
        let response = handle_command(&args);
        write!(out, "{}", response).unwrap();
        out.flush().unwrap();
    }
}
