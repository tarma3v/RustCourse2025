#![forbid(unsafe_code)]

use std::collections::HashSet;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn read_lines(filename: &str) -> HashSet<String> {
    BufReader::new(File::open(filename).expect("Error opening file"))
        .lines()
        .map(|line| line.expect("Error reading a line"))
        .collect()
}

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() != 3 {
        eprintln!("Incorrect args, should use: {} <file1> <file2>", args[0]);
        std::process::exit(1);
    }
    let set1 = read_lines(&args[1]);
    let set2 = read_lines(&args[2]);
    for line in set1.intersection(&set2) {
        println!("{}", line);
    }
}
