pub fn day_one() -> u32 {
    let input = include_str!("../Inputs/input1.txt");
    let mut output: u32 = 0;
    for line in input.lines() {
        let mut number = String::new();
        let mut first_digit = '\0';
        let mut last_digit = '\0';
        for letter in line.chars() {
            if letter.is_ascii_digit() {
                if first_digit == '\0' { first_digit = letter; }
                last_digit = letter;
            }
        }
        number.push(first_digit);
        number.push(last_digit);
        output += number.parse::<u32>().unwrap();
    }
    output
}