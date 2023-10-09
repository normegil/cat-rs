use std::{fs, io};

use clap::Parser;
use thiserror::Error;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    input: Option<Vec<String>>,
}

#[derive(Error, Debug)]
enum Errors {
    #[error("Cannot read from stdin")]
    CouldNotReadFromStdin(#[from] io::Error)
}

fn main() {
    let args = Args::parse();

    if args.input.is_none() {
        if let Err(e) = read_from_stdin() {
            panic!("{}", e);
        }
    } else if let Some(inputs) = args.input {
        for input in inputs {
            match fs::read_to_string(&input) {
                Ok(content) => {
                    println!("{}", content.trim());
                }
                Err(e) => {
                    panic!("Error when reading {}: {}", &input, e);
                }
            }
        }
    }
}

fn read_from_stdin() -> Result<(), Errors> {
    let stdin = io::stdin();
    loop {
        let mut buffer = String::new();
        stdin.read_line(&mut buffer)?;
        println!("{}", buffer.trim());
    }
}

#[cfg(test)]
mod tests {
    use clap::Parser;

    use super::Args;

    #[test]
    fn arg_none() {
        let args = Args::parse_from(&["cat-rs"]);
        assert_eq!(args.input, None);
    }

    #[test]
    fn arg_one() {
        let args = Args::parse_from(&["cat-rs", "first_arg.txt"]);
        assert_eq!(args.input, Some(vec!["first_arg.txt".to_string()]));
    }

    #[test]
    fn arg_multiple() {
        let args = Args::parse_from(&["cat-rs", "first_arg.txt", "second_arg.txt", "third_arg.txt"]);
        assert_eq!(args.input, Some(vec![
            "first_arg.txt".to_string(),
            "second_arg.txt".to_string(),
            "third_arg.txt".to_string()
        ]));
    }
}