use std::{
    collections::HashMap,
    error::Error,
    fs::File,
    io::{self, BufRead},
    path::Path,
};

use regex::Regex;

fn main() -> Result<(), Box<dyn Error>> {
    let path = Path::new("input/day3.txt");

    // Open the file in read-only mode
    let file = File::open(&path)?;

    // Create a BufReader for the file
    let reader = io::BufReader::new(file);
    for line in reader.lines() {
        match line {
            Ok(l) => {
                check_valid_number(&l);
                // println!("{}", l)
            }
            Err(e) => println!("Error: {}", e),
        }
    }
    Ok(())
}

fn check_valid_number(s: &str) -> HashMap<String, Vec<u32>> {
    let re_symbols = Regex::new(r"[^0-9\.]").unwrap();
    let re_numbers = Regex::new(r"\d+").unwrap();
    let mut counts = HashMap::new();

    for cap in re_symbols.captures_iter(s) {
        println!("{:?}", cap);
    }

    counts
}
