use regex::Regex;

pub fn question_one() -> u32 {
    let input = include_str!("../Inputs/input1.txt");
    let mut output: u32 = 0;
    for line in input.lines() {
        let mut number = String::new();
        let mut first_digit = '\0';
        let mut last_digit = '\0';
        for letter in line.chars() {
            if letter.is_ascii_digit() {
                if first_digit == '\0' {
                    first_digit = letter;
                }
                last_digit = letter;
            }
        }
        number.push(first_digit);
        number.push(last_digit);
        output += number.parse::<u32>().unwrap();
    }
    output
}

fn parse_number(input: &str) -> &str {
    match input {
        "0" => "0",
        "1" => "1",
        "2" => "2",
        "3" => "3",
        "4" => "4",
        "5" => "5",
        "6" => "6",
        "7" => "7",
        "8" => "8",
        "9" => "9",
        "one" => "1",
        "two" => "2",
        "three" => "3",
        "four" => "4",
        "five" => "5",
        "six" => "6",
        "seven" => "7",
        "eight" => "8",
        "nine" => "9",
        default => panic!("Expected a number, got {}", default),
    }
}
pub fn question_two() -> u32 {
    let input = include_str!("../Inputs/input1.txt");
    let re = Regex::new(r"([0-9]|one|two|three|four|five|six|seven|eight|nine)").unwrap();
    let mut output: u32 = 0;
    for line in input.lines() {
        let mut number = String::new();
        let mut first_digit = "\0";
        let mut last_digit = "\0";
        let mut matches = re.find_iter(line);
        first_digit = matches.next().unwrap().as_str();
        last_digit = first_digit;
        for item in matches {
            if item.is_empty() {
                continue;
            }
            last_digit = item.as_str();
        }
        number.push_str(parse_number(first_digit));
        number.push_str(parse_number(last_digit));
        output += number.parse::<u32>().unwrap();
    }
    output
}
