fn main() {
    let file_contents = std::fs::read_to_string("./data").unwrap();

    let mut pos: i32 = 0;

    print!("{}", pos);

    let mut trees_found = 0;

    for line in file_contents.lines() {
        let position = pos % line.len() as i32;

        if line.chars().nth(position as usize).unwrap() == '#' {
            trees_found += 1;
        }

        pos = pos + 3;
    }

    println!("{}", trees_found);
}
