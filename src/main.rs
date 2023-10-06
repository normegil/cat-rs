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
