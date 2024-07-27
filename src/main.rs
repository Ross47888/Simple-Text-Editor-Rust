use clap::Parser;
use text_editor::*;

fn main() {
    let args = Cli::parse();
    println!("{:?}", args.path);
}
