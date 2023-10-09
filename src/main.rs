use std::{fs, io};

use clap::Parser;
use thiserror::Error;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, default_value_t = false)]
    all: bool,

    #[arg(short = 'e', default_value_t = false)]
    show_end: bool,

    #[arg(short = 't', default_value_t = false)]
    show_tab: bool,

    #[arg(short = 'E', default_value_t = false)]
    show_end_partial: bool,

    #[arg(short = 'T', default_value_t = false)]
    show_tab_partial: bool,

    #[arg(short = 'v', default_value_t = false)]
    show_non_printing: bool,

    input: Option<Vec<String>>,
}

#[derive(Error, Debug)]
enum Errors {
    #[error("Cannot read from stdin")]
    CouldNotReadFromStdin(#[from] io::Error),
}

fn main() {
    let args = Args::parse();

    let output_opts = OutputOptions::from_args(&args);

    if args.input.is_none() {
        if let Err(e) = read_from_stdin(output_opts) {
            panic!("{}", e);
        }
    } else if let Some(inputs) = args.input {
        for input in inputs {
            match fs::read_to_string(&input) {
                Ok(content) => {
                    print_content(&output_opts, &content);
                }
                Err(e) => {
                    panic!("Error when reading {}: {}", &input, e);
                }
            }
        }
    }
}

fn read_from_stdin(opts: OutputOptions) -> Result<(), Errors> {
    let stdin = io::stdin();
    loop {
        let mut buffer = String::new();
        stdin.read_line(&mut buffer)?;
        print_content(&opts, &buffer)
    }
}

struct OutputOptions {
    show_tabs: bool,
    show_ends: bool,
}

impl OutputOptions {
    fn from_args(args: &Args) -> Self {
        let show_tabs =
            args.all || args.show_tab || (args.show_non_printing && args.show_tab_partial);
        let show_ends =
            args.all || args.show_end || (args.show_non_printing && args.show_end_partial);
        Self {
            show_tabs,
            show_ends,
        }
    }
}

fn print_content(opts: &OutputOptions, input: &str) {
    let mut to_print = input.to_string();
    if opts.show_tabs {
        to_print = to_print.replace('\t', "^I");
    }
    if opts.show_ends {
        to_print = to_print.replace('\n', "$\n");
    }
    println!("{}", to_print.trim())
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
        let args =
            Args::parse_from(&["cat-rs", "first_arg.txt", "second_arg.txt", "third_arg.txt"]);
        assert_eq!(
            args.input,
            Some(vec![
                "first_arg.txt".to_string(),
                "second_arg.txt".to_string(),
                "third_arg.txt".to_string(),
            ])
        );
    }

    #[test]
    fn arg_show_tab() {
        let args = Args::parse_from(&["cat-rs", "-t", "first_arg.txt"]);
        assert!(args.show_tab);
    }

    #[test]
    fn arg_show_tab_partial() {
        let args = Args::parse_from(&["cat-rs", "-T", "first_arg.txt"]);
        assert!(args.show_tab_partial);
    }

    #[test]
    fn arg_show_end() {
        let args = Args::parse_from(&["cat-rs", "-e", "first_arg.txt"]);
        assert!(args.show_end);
    }

    #[test]
    fn arg_show_end_partial() {
        let args = Args::parse_from(&["cat-rs", "-E", "first_arg.txt"]);
        assert!(args.show_end_partial);
    }

    #[test]
    fn arg_show_non_printing() {
        let args = Args::parse_from(&["cat-rs", "-v", "first_arg.txt"]);
        assert!(args.show_non_printing);
    }
}
