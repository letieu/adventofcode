use std::{
    fs::{self},
    path::Path,
};

fn main() {
    let path = Path::new("./input.txt");
    let content = fs::read_to_string(path).expect("Something went wrong reading the file");

    let mut top_three = [0, 0, 0];
    let mut block_sum = 0;

    for line in content.lines() {
        if line.is_empty() {
            let min = top_three.iter().min().unwrap();

            if &block_sum > min {
                top_three[0] = block_sum;
                top_three.sort();
            }

            // reset block_sum
            block_sum = 0;
            continue;
        }

        block_sum += line.trim().parse::<i32>().unwrap();
    }

    println!("Top three: {:?}", top_three);
    println!("Sum: {}", top_three.iter().sum::<i32>());
}
