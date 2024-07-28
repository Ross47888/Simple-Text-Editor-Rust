use std::path::PathBuf;
use std::fs::File;
use std::io::{Read, BufReader};

use colored::Colorize;
use clap::Parser;

#[derive(Parser)]
pub struct Cli {
    pub path: PathBuf,
}

pub fn driver_loop(file: &mut File) {
    let mut buf = String::new();
    let file_string_result = file.read_to_string(&mut buf);
    let _file_string = match file_string_result {
        Ok(file_string) => file_string,
        Err(e) => panic!("Error reading file to string\n{:?}", e),
    };

    let title = "\nFile Contents".bold().blue();
    println!("{}", title);
    // println!("{}", {&buf});

    let vec: Vec<&str> = buf.lines().collect();

    for (c, v) in vec.iter().enumerate() {
        println!("{} {}", c.to_string().bold(), v);
    }
}

#[cfg(test)]
mod tests {
    // use crate::lib::*;

}
