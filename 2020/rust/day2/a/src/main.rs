extern crate regex;
use crate::regex::Regex;

fn main() {
    let re = Regex::new(r"(\d+)-(\d+)\s([a-z]):\s+([a-zA-Z]+)").unwrap();

    let file_contents = std::fs::read_to_string("data").unwrap();
    let mut valid_count = 0;

    for line in file_contents.lines() {
        if let Some(caps) = re.captures(&line) {
            let low: i32 = caps.get(1).unwrap().as_str().parse().unwrap();
            let high: i32 = caps.get(2).unwrap().as_str().parse().unwrap();
            let letter = caps.get(3).unwrap().as_str();
            let password = caps.get(4).unwrap().as_str();

            // println!("{}", low);
            // println!("{}", high);
            // println!("{}", letter);
            // println!("{}", password);

            if low <= password.matches(letter).count() as i32
                && password.matches(letter).count() as i32 <= high
            {
                valid_count += 1;
            }
        } else {
            println!("no match");
        }
    }

    println!("{}", valid_count);
}
