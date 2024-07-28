use std::fs::File;
use clap::Parser;

use text_editor::*;

fn main() {
    let args = Cli::parse();
    println!("{:?}", args.path);

    let file_result = File::open(args.path);

    let mut file = match file_result {
        Ok(file) => file,
        Err(e) => panic!("Failed to read your file!\n{:?}", e),
    };
    
    println!("{:?}", file);
    driver_loop(&mut file);
}
