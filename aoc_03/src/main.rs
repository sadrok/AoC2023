use std::collections::HashSet;

use regex::Regex;

type SurroundingNumber = (u32, u32, u32); // line_number, position, value

#[derive(Debug)]
enum MyEnum {
    Symbol(char),
    Number(u32),
}

#[derive(Debug)]
struct Element {
    line_number: u32,
    start: u32,
    end: u32,
    value: MyEnum,
}

fn main() {
    let mut elements: Vec<Element> = Vec::new();
    let lines = std::io::stdin().lines();

    extract_elements(lines, &mut elements);

    for item in &elements {
        println!("{item:?}");
    }

    let mut total = 0u32;
    let mut total_gear_ratios = 0;

    let mut surrounding_numbers: HashSet<SurroundingNumber> = HashSet::new();

    for item in &elements {
        // Check if the item is a symbol
        if let MyEnum::Symbol(symbol) = item.value {
            let line_range =
                item.line_number.checked_sub(1).unwrap_or(item.line_number)..item.line_number + 2;

            println!(
                "Symbol {} on line {} search in line range: {:?}",
                symbol, item.line_number, line_range
            );

            let mut potential_gears: HashSet<SurroundingNumber> = HashSet::new();

            elements
                .iter()
                .filter(|check_item| line_range.contains(&check_item.line_number))
                .for_each(|check_item| {
                    extract_numbers_and_gears(
                        check_item,
                        item,
                        &mut total,
                        &mut surrounding_numbers,
                        &mut potential_gears,
                    );
                });

            if symbol == '*' && potential_gears.len() == 2 {
                total_gear_ratios += potential_gears.iter().map(|gear| gear.2).product::<u32>();
            }
        }
    }

    println!("Applicable numbers were: {surrounding_numbers:?}");
    println!("Total: {total}");
    println!("Total gear ratios: {total_gear_ratios}");
}

fn extract_numbers_and_gears(
    check_item: &Element,
    item: &Element,
    total: &mut u32,
    surrounding_numbers: &mut HashSet<(u32, u32, u32)>,
    potential_gears: &mut HashSet<(u32, u32, u32)>,
) {
    if let MyEnum::Number(number) = check_item.value {
        println!(
            "We have number {} on line {}",
            number, check_item.line_number
        );
        if check_item.start <= item.start + 1
            && check_item.end >= item.end.checked_sub(1).unwrap_or(item.end)
        {
            println!("Number {number} is in range of this symbol");
            *total += number;
            surrounding_numbers.insert((check_item.line_number, check_item.start, number));
            potential_gears.insert((check_item.line_number, check_item.start, number));
        }
    }
}

fn extract_elements(lines: std::io::Lines<std::io::StdinLock<'_>>, elements: &mut Vec<Element>) {
    let number_regex = Regex::new(r"\d+").unwrap();
    let symbol_regex = Regex::new(r"[^a-zA-Z0-9.]").unwrap();

    for (line_number, line) in lines.enumerate() {
        let line = line.unwrap();
        number_regex
            .captures_iter(line.as_str())
            .for_each(|capture| {
                elements.push(Element {
                    line_number: line_number as u32,
                    start: capture.get(0).unwrap().start() as u32,
                    end: capture.get(0).unwrap().end() as u32,
                    value: MyEnum::Number(capture.get(0).unwrap().as_str().parse::<u32>().unwrap()),
                });
            });

        symbol_regex
            .captures_iter(line.as_str())
            .for_each(|capture| {
                elements.push(Element {
                    line_number: line_number as u32,
                    start: capture.get(0).unwrap().start() as u32,
                    end: capture.get(0).unwrap().end() as u32,
                    value: MyEnum::Symbol(capture.get(0).unwrap().as_str().chars().next().unwrap()),
                });
            });
    }
}
