use std::process;
mod ask;

struct Config {
    list_size:  u32,
    list_of_numbers: Vec<u32>
}

impl Config {
    fn new(list_size: u32, list_of_numbers: Vec<u32>) -> Result<Config, &'static str> {
        if list_size < 2 { return Err("list size must be at least a size of 2" ); }
        if list_of_numbers.len() < 2 { return Err("you must provide at least 2 numbers" ); }
        if list_of_numbers.len() != list_size as usize { return Err("list size and numbers provided must be of the same length" ); }

        Ok(Config { list_size, list_of_numbers})
    }
}

fn main() {

    let list_size = match ask::input(String::from("Please, provide n which is the size of the array")).trim().parse::<u32>() {
        Ok(list_size) => list_size,
        Err(e) => {
            println!("{}", e);
            process::exit(1);
        }
    };

    let list_of_numbers = ask::input(String::from("Please, provide a list of numbers")).split_whitespace()
        .map(|c| {
            match c.parse::<u32>() {
                Ok(number) => number,
                Err(e) => {
                    println!("{}", e);
                    process::exit(1);
                }
            }
        }).collect::<Vec<u32>>();


    let config = Config::new(list_size, list_of_numbers).unwrap_or_else(|err| {
        println!("{}", err);
        process::exit(1);
    });
}

