use crate::Args;
use crate::expr::*;
use std::io::{stdin, stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use colored::Colorize;

const NAME: &str = env!("CARGO_PKG_NAME");

enum Command {
    Eval,
    Simplify,
    Clear,
    Help,
    Exit,
    Unknown,
}

impl Command {
    fn from_str(s: &str) -> Self {
        match s {
            "eval" => Self::Eval,
            "e" => Self::Eval,
            "simplify" => Self::Simplify,
            "s" => Self::Simplify,
            "clear" => Self::Clear,
            "help" => Self::Help,
            "exit" => Self::Exit,
            _ => Self::Unknown,
        }
    }

    fn display(&self) -> String {
        match self {
            Self::Eval => "eval".to_string(),
            Self::Simplify => "simplify".to_string(),
            Self::Clear => "clear".to_string(),
            Self::Help => "help".to_string(),
            Self::Exit => "exit".to_string(),
            Self::Unknown => "unknown".to_string(),
        }
    }

    fn help() -> String {
        format!(
            "Commands
- eval (e): Evaluate an expression
- simplify (s): Simplify an expression
- clear: Clear the screen
- help: Display this help message
- exit: Exit the program"
        )
    }
}

fn repl_eval(buffer: &str, stdout: &mut std::io::Stdout, history: &mut Vec<String>) {
    let buffer = buffer.trim();

    if buffer.is_empty() {
        return;
    }

    history.push(buffer.to_string());

    let words: Vec<&str> = buffer.split_whitespace().collect();
    let command = Command::from_str(words[0]);

    println!("{}", command.display());
    
    match command {
        Command::Eval => {
            let _ = writeln!(stdout, "{}", "Eval");
        }

        Command::Simplify => {
            let _ = writeln!(stdout, "{}", "Simplify");
        }

        Command::Clear => {
            let _ = write!(stdout, "{}", termion::clear::All);
            let _ = write!(stdout, "{}", termion::cursor::Goto(1, 1));
            return;
        }

        Command::Help => {
            let _ = writeln!(stdout, "{}", Command::help());
        }

        Command::Exit => {
            let _ = writeln!(stdout, "{}", "Bye!");
            std::process::exit(0);
        }

        Command::Unknown => {
            let _ = writeln!(stdout, "{}", "Unknown command".red().bold());
        }
    }

    let _ = write!(stdout, "\n");
}

pub fn start_repl(_args: Args) {
    let stdin = stdin();
    let mut stdout = stdout();
    let mut buffer = String::new();

    let _ = write!(stdout, "{}", format!("{}> ", NAME).green().bold());
    stdout.flush().unwrap();

    let mut history: Vec<String> = Vec::new();
    for key in stdin.keys() {
        let key = key.unwrap();
        match key {
            Key::Backspace => {
                if buffer.is_empty() {
                    continue;
                }
                buffer.pop();
                write!(stdout, "\r{}", " ".repeat(buffer.len() + 2)).unwrap();
                write!(stdout, "\r{}", format!("{}> {}", NAME, buffer)).unwrap();
                stdout.flush().unwrap();
            }

            Key::Char('\n') => {
                repl_eval(&buffer, &mut stdout, &mut history);
                buffer.clear();

                let _ = write!(stdout, "{}", format!("{}> ", NAME).green().bold());
                stdout.flush().unwrap();
            }

            Key::Up | Key::Down => {
                if history.len() == 0 {
                    continue;
                }

                let last = history.len() - 1;
                let last = if key.clone() == Key::Up {
                    last
                } else {
                    0
                };

                buffer = history[last].clone();
                write!(stdout, "\r{}", " ".repeat(buffer.len() + 2)).unwrap();
                write!(stdout, "\r{}", format!("{}> {}", NAME, buffer)).unwrap();

            }

            Key::Char(c) => {
                buffer.push(c);
                stdout.flush().unwrap();
            }

            _ => continue,
        }
    }
}
