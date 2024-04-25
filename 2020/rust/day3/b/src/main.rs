struct Slope {
    x: i32,
    y: i32,
}

fn main() {
    let slopes: Vec<Slope> = vec![
        Slope { x: 1, y: 1 },
        Slope { x: 3, y: 1 },
        Slope { x: 5, y: 1 },
        Slope { x: 7, y: 1 },
        Slope { x: 1, y: 2 },
    ];

    let file_contents = std::fs::read_to_string("./data").unwrap();

    let lines: Vec<&str> = file_contents.lines().collect();

    let mut values: Vec<i32> = vec![];

    for slope in slopes {
        let mut trees_found = 0;
        let mut i = 0;

        for line in lines.iter().step_by(slope.y as usize) {
            let position = slope.x * i % line.len() as i32;

            if line.chars().nth(position as usize).unwrap() == '#' {
                trees_found += 1;
            }

            i += 1;
        }

        values.push(trees_found);
    }

    let product: i64 = values.iter().map(|&x| x as i64).product();

    println!("Product: {}", product);
}
