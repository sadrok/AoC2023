use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    filename: PathBuf,
}

fn main() {
    let args = Args::parse();
    let mut total = 0u32;

    // Get args.input as an line iterable file
    let file = std::fs::read_to_string(&args.filename).expect("Couldn't read file");
    let lines = file.lines();

    // Iterate over lines
    for line in lines {
        let mut first_digit = 0;
        let mut last_digit = 0;
        for char in line.chars() {
            if char.is_digit(10) {
                // Convert char to integer
                first_digit = char.to_digit(10).unwrap();
                last_digit = first_digit;
                break;
            }
        }
        for char in line.chars() {
            if char.is_digit(10) {
                last_digit = char.to_digit(10).unwrap();
            }
        }

        println!(
            "This line's first digit is {} and last digit is {}",
            first_digit, last_digit
        );
        total += first_digit * 10 + last_digit;
    }

    println!("The total is {}", total);
}
