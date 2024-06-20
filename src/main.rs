use clap::{ArgAction, Parser};
use std::fs::read_to_string;

#[derive(Debug, Parser)]
#[command(author, about, long_about = None)]
struct Args {
    #[arg(short, long, action=ArgAction::SetTrue, help="number all output lines")]
    number: bool,
    #[arg(short, long, value_parser, num_args = 1.., value_delimiter = ' ',help = "files to read")]
    file: Vec<String>,
}

fn main() {
    let args = Args::parse();
    println!("{:?}", args);
    let mut result: String = String::new();
    for file in args.file {
        for (i, line) in read_to_string(file).unwrap().lines().enumerate() {
            // result.push(line.to_string())
            if args.number {
                result.push_str(i.to_string().as_str());
                result.push_str(": ");
            }
            result.push_str(line);
            result.push_str("\n");
        }
    }
    println!("{}", result);
}
