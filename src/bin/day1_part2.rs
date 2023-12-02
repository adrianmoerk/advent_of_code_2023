use std::collections::HashMap;
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
                // println!("\n\n---NEWLINE---");
                // let mut nums_in_line_map: HashMap<usize, u32> = HashMap::new();
                let mut num_indexes: HashMap<usize, u32> = HashMap::new();
                match l.find("one") {
                    Some(index) => {
                        num_indexes.insert(index, 1);
                        // println!("FOUND ONE, inserting 1 at {}",index);
                    }
                    None => {}
                }
                match l.find("two") {
                    Some(index) => {
                        num_indexes.insert(index, 2);
                    }
                    None => {}
                }
                match l.find("three") {
                    Some(index) => {
                        num_indexes.insert(index, 3);
                    }
                    None => {}
                }
                match l.find("four") {
                    Some(index) => {
                        num_indexes.insert(index, 4);
                    }
                    None => {}
                }
                match l.find("five") {
                    Some(index) => {
                        num_indexes.insert(index, 5);
                    }
                    None => {}
                }
                match l.find("six") {
                    Some(index) => {
                        num_indexes.insert(index, 6);
                    }
                    None => {}
                }
                match l.find("seven") {
                    Some(index) => {
                        num_indexes.insert(index, 7);
                    }
                    None => {}
                }
                match l.find("eight") {
                    Some(index) => {
                        num_indexes.insert(index, 8);
                    }
                    None => {}
                }
                match l.find("nine") {
                    Some(index) => {
                        num_indexes.insert(index, 9);
                    }
                    None => {}
                }
                let mut index = 0;

                for char in l.chars() {
                    if char.is_numeric() {
                        num_indexes.insert(index, char.to_digit(10).unwrap());
                    }
                    index += 1;
                }
                let mut first_num_index = usize::MAX;
                let mut last_num_index = usize::MIN;
                for (index, _) in &mut num_indexes {
                    if index < &first_num_index {
                        first_num_index = *index;
                        // println!("NEW FIRST INDEX: {}",first_num_index);
                    }
                    if index > &last_num_index {
                        last_num_index = *index;
                        // println!("NEW LAST INDEX: {}",last_num_index);
                    }
                }
                // println!("NUM_INDEXES: {:?}", num_indexes);
                let mut num = &mut num_indexes.get(&first_num_index).unwrap().to_string();
                num.push_str(
                    num_indexes
                        .get(&last_num_index)
                        .unwrap()
                        .to_string()
                        .as_str(),
                );
                let num_val: u32 = num.parse().unwrap();
                sum += num_val;
            }
            Err(e) => println!("Error reading line: {}", e),
        }
    }
    println!("SUM: {}", sum);

    Ok(())
}
