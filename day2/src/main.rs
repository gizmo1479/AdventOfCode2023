use regex::{Match, Regex};
use std::cmp::max;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

const FILEPATH: &str = "data.txt";

fn parse_line_1(line: &str) -> (bool, u32) {
    // get the id number
    let game_and_rest: Vec<&str> = line.split(":").collect();
    let re = Regex::new(r"\d+").unwrap();
    let d_match: Option<Match> = re.find(game_and_rest[0]);
    let id = d_match.map(|s| s.as_str().parse::<u32>().unwrap()).unwrap();

    let batch = game_and_rest[1].split(";");
    let mut possible = true;
    for s in batch {
        let colors = s.split(",");
        if !possible {
            break;
        }
        for color in colors {
            let num_and_col: Vec<&str> = color.split_whitespace().collect();
            let num = num_and_col[0].parse::<u32>().unwrap();
            if num_and_col[1] == "blue" && num > 14 {
                possible = false;
            } else if num_and_col[1] == "red" && num > 12 {
                possible = false;
            } else if num_and_col[1] == "green" && num > 13 {
                possible = false;
            }
        }
    }

    (possible, id)
}

fn parse_line_2(line: &str) -> u32 {
    // get the id number
    let game_and_rest: Vec<&str> = line.split(":").collect();
    // set up min cube values
    let mut min_red = 0;
    let mut min_blue = 0;
    let mut min_green = 0;

    let batch = game_and_rest[1].split(";");
    for s in batch {
        let colors = s.split(",");
        for color in colors {
            let num_and_col: Vec<&str> = color.split_whitespace().collect();
            let num = num_and_col[0].parse::<u32>().unwrap();
            if num_and_col[1] == "blue" {
                min_blue = max(num, min_blue);
            } else if num_and_col[1] == "red" {
                min_red = max(num, min_red);
            } else if num_and_col[1] == "green" {
                min_green = max(num, min_green);
            }
        }
    }

    min_blue * min_green * min_red
}

fn main() {
    let f = File::open(FILEPATH).unwrap();
    let r = BufReader::new(f);

    let mut id_sum = 0;
    let mut power_sum = 0;
    for line in r.lines() {
        let line_ref = line.unwrap();
        let (possible, id) = parse_line_1(&line_ref);
        if possible {
            id_sum += id;
        }

        power_sum += parse_line_2(&line_ref);
    }

    println!("id_sum: {}", id_sum);
    println!("power of sets: {}", power_sum);
}
