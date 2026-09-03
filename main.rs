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

fn encode_bulk_string(s: &str) -> String {
    format!("${}\r\n{}\r\n", s.len(), s)
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

fn handle_command(args: &[String]) -> String {
    let cmd = args[0].to_uppercase();

    match cmd.as_str() {
        "PING" => {
            if let Some(msg) = args.get(1) {
                eb(Some(msg))
            } else {
                es("PONG")
            }
        }
        "ECHO" => {
            if let Some(msg) = args.get(1) {
                eb(Some(msg))
            } else {
                "-ERR wrong number of arguments for 'ECHO'\r\n".to_string()
            }
        }
        _ => format!("-ERR unknown command\r\n"),
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
