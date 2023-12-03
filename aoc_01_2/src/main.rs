use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    filename: PathBuf,
}

const POTENTIALS: [&str; 20] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "0", "1", "2",
    "3", "4", "5", "6", "7", "8", "9",
];

fn main() {
    let args = Args::parse();
    let mut total = 0u32;

    // Get args.input as an line iterable file
    let file = std::fs::read_to_string(args.filename).expect("Couldn't read file");
    let lines = file.lines();

    // Iterate over lines
    for line in lines {
        let haystack = line.to_ascii_lowercase();
        let mut values: Vec<(usize, u32)> = vec![];

        // Find matches from the beginning
        for (i, needle) in POTENTIALS.iter().enumerate() {
            if let Some(location) = haystack.find(needle) {
                values.push((location, i as u32 % 10));
            }
        }
        values.sort();
        if values.is_empty() {
            println!("No numerics found in line {}", haystack);
            continue;
        }
        let digit = values.first().unwrap().1;
        print!("({},", digit);
        // Add the tens value
        total += digit * 10;

        // Search from the end
        for (i, needle) in POTENTIALS.iter().enumerate() {
            if let Some(location) = haystack.rfind(needle) {
                values.push((location, i as u32 % 10));
            }
        }
        values.sort();

        let digit = values.last().unwrap().1;
        print!("{})", digit);

        total += values.last().unwrap().1;
        println!(" found in {}", haystack);
    }

    println!("The total is {}", total);
}
