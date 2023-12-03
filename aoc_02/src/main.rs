use std::str::FromStr;

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
enum CubeColor {
    Red,
    Green,
    Blue,
}

impl FromStr for CubeColor {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_lowercase().as_str() {
            "red" => Ok(Self::Red),
            "green" => Ok(Self::Green),
            "blue" => Ok(Self::Blue),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
struct Cubes {
    color: CubeColor,
    number: u32,
}

#[derive(Debug, Default)]
struct MinCubes {
    red: u32,
    green: u32,
    blue: u32,
}

fn main() {
    let mut total = 0;
    let mut total_power = 0;

    for line in std::io::stdin().lines() {
        let mut min_cubes = MinCubes::default();
        let mut game_is_valid = true;
        let line = line.unwrap();
        let mut split = line.splitn(2, ':');
        let game_number = split
            .next()
            .expect("Game string found")
            .split_whitespace()
            .last()
            .expect("Game number found")
            .parse::<u32>()
            .expect("Game number is an integer");

        println!("Game number is {game_number}");
        let rest_string = split.next().expect("Subsets found");
        let groups_split = rest_string.split(';');
        for group in groups_split {
            println!("group: ");
            for cubes_string in group.split(',') {
                let mut cubes_split = cubes_string.split_whitespace();
                let number = cubes_split
                    .next()
                    .expect("No. cubes found")
                    .parse::<u32>()
                    .expect("No. cubes is an integer");
                let cube_color = cubes_split.next().expect("Cube color found");

                let cubes = Cubes {
                    color: cube_color
                        .parse::<CubeColor>()
                        .expect("Cube color is valid"),
                    number,
                };
                println!(" cubes: {cubes:?}");

                match cubes.color {
                    CubeColor::Red => {
                        if cubes.number > 12 {
                            game_is_valid = false;
                        }
                        min_cubes.red = min_cubes.red.max(cubes.number);
                    }
                    CubeColor::Green => {
                        if cubes.number > 13 {
                            game_is_valid = false;
                        }
                        min_cubes.green = min_cubes.green.max(cubes.number);
                    }
                    CubeColor::Blue => {
                        if cubes.number > 14 {
                            game_is_valid = false;
                        }
                        min_cubes.blue = min_cubes.blue.max(cubes.number);
                    }
                }
            }
        }
        let cubes_power = min_cubes.red * min_cubes.green * min_cubes.blue;
        println!("Min cubes for game is {min_cubes:?} with a power of {cubes_power}");
        total_power += cubes_power;
        if game_is_valid {
            println!("Game {game_number} is valid");
            total += game_number;
        } else {
            println!("Game {game_number} is invalid");
        }
    }

    println!("Total is {total}");
    println!("Total power is {total_power}");
}
