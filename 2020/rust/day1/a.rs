fn main() {
    // let file_contents = include_str!("data");
    let file_contents = std::fs::read_to_string("data").expect("Error reading input");
    let mut numbers: Vec<i32> = Vec::new();

    for line in file_contents.lines() {
        numbers.push(line.parse::<i32>().unwrap());
    }

    for num1 in &numbers {
        for num2 in &numbers {
            if num1 + num2 == 2020 {
                // println!("{num1}");
                // println!("{num2}");
                let product: i32 = num1 * num2;
                println!("{product}");

                return;
                // let product: i32 = num1 * num2;
                // print!("{product}");
                // println!("{}", num1 * num2);
            }
        }
    }
}
