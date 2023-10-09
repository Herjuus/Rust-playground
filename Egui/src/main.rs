use std::io;

fn main() {
    println!("{}", "Enter number: ");

    let mut input = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut input).unwrap();
    let trimmed = input.trim();
    
    let mut int: i32 = 0;

    match trimmed.parse::<i32>() {
        Ok(i) => int = i,
        Err(..) => println!("this was not an integer: {}", trimmed),
    }

    let roman_num = calculate(int);
    println!("{}", roman_num);
}

fn calculate(num: i32) -> String {
    let mut number = num;
    let mut roman: String = String::from("");
    while number > 0 {
        if number >= 1000 {
            roman.push_str("M");
            number = number - 1000;
        } else if number >= 900 {
            roman.push_str("CM");
            number = number - 900;
        } else if number >= 500 {
            roman.push_str("D");
            number = number - 500;
        } else if number >= 400 {
            roman.push_str("CD");
            number = number - 400;
        } else if number >= 100 {
            roman.push_str("C");
            number = number - 100;
        } else if number >= 90 {
            roman.push_str("XC");
            number = number - 90;
        } else if number >= 50 {
            roman.push_str("L");
            number = number - 50;
        } else if number >= 40 {
            roman.push_str("XL");
            number = number - 40;
        } else if number >= 10 {
            roman.push_str("X");
            number = number - 10;
        } else if number >= 9 {
            roman.push_str("IX");
            number = number - 9;
        } else if number >= 5 {
            roman.push_str("V");
            number = number - 5;
        } else if number >= 4 {
            roman.push_str("IV");
            number = number - 4;
        } else if number >= 1 {
            roman.push_str("I");
            number = number - 1;
        }
    }

    return roman;
}