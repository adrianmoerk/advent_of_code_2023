use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() -> io::Result<()> {
    // Specify the file path
    let path = Path::new("input/day1.txt");

    // Open the file in read-only mode
    let file = File::open(&path)?;

    // Create a BufReader for the file
    let reader = io::BufReader::new(file);
    let mut sum: u32 = 0;
    // Iterate over each line
    for line in reader.lines() {
        match line {
            Ok(l) => {
                let mut nums_in_line_vec: Vec<u32> = vec![];
                for char in l.chars() {
                    if char.is_numeric() {
                        nums_in_line_vec.push(char.to_digit(10).unwrap())
                    }
                }
                let mut num = nums_in_line_vec.first().unwrap().to_string();
                num.push_str(nums_in_line_vec.last().unwrap().to_string().as_str());
                let num_val: u32 = num.parse().unwrap();
                sum += num_val;
            }
            Err(e) => println!("Error reading line: {}", e),
        }
    }
    println!("SUM: {}", sum);

    Ok(())
}
