extern crate regex;

use crate::regex::Regex;

fn main() {
    let re = Regex::new(r"(\d+)-(\d+)\s([a-z]):\s+([a-zA-Z]+)").unwrap();

    let file_contents = std::fs::read_to_string("data").unwrap();
    let mut valid_count = 0;

    for line in file_contents.lines() {
        if let Some(caps) = re.captures(&line) {
            let char1: i32 = caps.get(1).unwrap().as_str().parse().unwrap();
            let char2: i32 = caps.get(2).unwrap().as_str().parse().unwrap();
            let letter = caps.get(3).unwrap().as_str().chars().next().unwrap();
            let password = caps.get(4).unwrap().as_str();

            // check if the letter is at both positions [char1] and [char2]

            if (password.chars().nth(char1 as usize - 1).unwrap() == letter
                && password.chars().nth(char2 as usize - 1).unwrap() != letter)
                || (password.chars().nth(char1 as usize - 1).unwrap() != letter
                    && password.chars().nth(char2 as usize - 1).unwrap() == letter)
            {
                valid_count += 1;
            }
        } else {
            println!("no match");
        }
    }

    println!("{}", valid_count);
}
