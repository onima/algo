use std::process;
use std::io;
use std::num;

pub fn number() -> Result<u32, num::ParseIntError> {
    let user_input = input("Please, provide a number").trim().parse::<u32>()?;
    Ok(user_input)
}

pub fn list_of_numbers() -> Result<Vec<u32>, num::ParseIntError> {
    input("Please, provide a list of numbers")
        .split_whitespace()
        .map(|c| { c.parse::<u32>() })
        .collect()
}

fn input(request: &str) -> String {
    println!("{}", request);

    fn input() -> Result<String, io::Error> {
        let reader = io::stdin();
        let mut new_str = String::new();
        reader.read_line(&mut new_str)?;
        Ok(new_str)
    }

    return input().unwrap_or_else(|err| {
        eprintln!("Problem reading input: {}", err);
        process::exit(1);
    });
}