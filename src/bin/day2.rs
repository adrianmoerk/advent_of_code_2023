use regex::Regex;
use std::{
    collections::HashMap,
    error::Error,
    fs::File,
    io::{self, BufRead},
    path::Path,
};

fn main() -> Result<(), Box<dyn Error>> {
    let mut game_id_sum = 0;
    // Specify the file path
    let path = Path::new("input/day2.txt");

    // Open the file in read-only mode
    let file = File::open(&path)?;

    // Create a BufReader for the file
    let reader = io::BufReader::new(file);

    for game in reader.lines() {
        match game {
            Ok(g) => {
                let mut current_game = Game::new();
                // println!("game: {}", g);
                let split_vec: Vec<&str> = g.split(" ").collect();
                let game_id: u32 = split_vec[1].replace(":", "").parse().unwrap();
                // println!("GAME ID IS: {}",game_id);
                let colors_count_map = extract_colors(g.as_str());
                for blue in colors_count_map.get("blue").unwrap() {
                    current_game.update_blue(*blue);
                }
                for red in colors_count_map.get("red").unwrap() {
                    current_game.update_red(*red);
                }
                for green in colors_count_map.get("green").unwrap() {
                    current_game.update_green(*green);
                }
                // println!("{:#?}", current_game);
                if current_game.check_valid(12, 13, 14) {
                    game_id_sum += game_id;
                }
            }
            Err(e) => {
                println!("Error reading Lines: {}", e);
            }
        }
    }
    println!("{}", game_id_sum);

    Ok(())
}

fn extract_colors(s: &str) -> HashMap<String, Vec<u32>> {
    let re = Regex::new(r"(\d+)\s*(green|blue|red)").unwrap();
    let mut counts = HashMap::new();

    for cap in re.captures_iter(s) {
        let color = cap[2].to_string();
        let count = cap[1].parse::<u32>().unwrap();
        counts.entry(color).or_insert_with(Vec::new).push(count);
    }

    counts
}
#[derive(Debug)]
struct Game {
    red: u32,
    blue: u32,
    green: u32,
}
impl Game {
    fn new() -> Self {
        Self {
            red: 0,
            blue: 0,
            green: 0,
        }
    }
    fn update_blue(&mut self, blue_val: u32) {
        if self.blue < blue_val {
            self.blue = blue_val;
        }
    }
    fn update_red(&mut self, red_val: u32) {
        if self.red < red_val {
            self.red = red_val;
        }
    }
    fn update_green(&mut self, green_val: u32) {
        if self.green < green_val {
            self.green = green_val;
        }
    }
    fn check_valid(&mut self, max_red: u32, max_green: u32, max_blue: u32) -> bool {
        if self.red > max_red {
            return false;
        }
        if self.green > max_green {
            return false;
        }
        if self.blue > max_blue {
            return false;
        }
        true
    }
}
