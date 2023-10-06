use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    input: Option<Vec<String>>,
}
fn main() {
    let args = Args::parse();

    println!("Hello, {:?}", args);
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