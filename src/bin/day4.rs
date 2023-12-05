use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() -> io::Result<()> {
    // Specify the file path
    let path = Path::new("input/day4.txt");

    // Open the file in read-only mode
    let file = File::open(&path)?;

    // Create a BufReader for the file
    let reader = io::BufReader::new(file);
    let mut point_sum: u32 = 0;
    // Iterate over each line
    for line in reader.lines() {
        match line {
            Ok(l) => {
                let mut card_points = 0;
                let card_split: Vec<&str> = l.split("|").collect();
                let my_card = card_split
                    .first()
                    .unwrap()
                    .to_string()
                    .split(":")
                    .collect::<Vec<&str>>()
                    .last()
                    .unwrap()
                    .to_string();
                let card_id = card_split
                    .first()
                    .unwrap()
                    .to_string()
                    .split(":")
                    .collect::<Vec<&str>>()
                    .first()
                    .unwrap()
                    .to_string()
                    .split(" ")
                    .collect::<Vec<&str>>()
                    .last()
                    .unwrap()
                    .to_string();
                let winning_numbers = card_split.last().unwrap().to_string();
                let winning_numbers_vec: Vec<&str> = winning_numbers.split(" ").collect();
                let mut winning_numbers_vec_parsed: Vec<u32> = vec![];
                for num_str in winning_numbers_vec.clone() {
                    if num_str != "" {
                        winning_numbers_vec_parsed.push(num_str.parse::<u32>().unwrap());
                    }
                }
                let my_card_vec: Vec<&str> = my_card.split(" ").collect();
                let mut my_card_vec_parsed: Vec<u32> = Vec::new();
                for num_str in my_card_vec {
                    if num_str != "" {
                        my_card_vec_parsed.push(num_str.parse::<u32>().unwrap());
                    }
                }
                // println!("{:?}", my_card_vec_parsed);
                for num in my_card_vec_parsed {
                    if winning_numbers_vec_parsed.contains(&num) {
                        if card_points == 0 {
                            card_points = 1;
                        } else {
                            card_points = card_points * 2;
                        }
                    }
                }
                point_sum += card_points;
            }
            Err(e) => {
                println!("Error: {}", e)
            }
        }
    }
    println!("Total Points: {}", point_sum);
    Ok(())
}
