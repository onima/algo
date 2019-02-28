use std::error::Error;
use crate::ask;

struct Config {
    list_size: u32,
    list_of_numbers: Vec<u32>
}

struct Number {
    value: u32,
    index: usize
}

impl Config {
    pub fn new(list_size: u32, list_of_numbers: Vec<u32>) -> Result<Config, &'static str> {
        if list_size < 2 { return Err("list size must be at least a size of 2" ); }
        if list_of_numbers.len() < 2 { return Err("you must provide at least 2 numbers" ); }
        if (list_of_numbers.len() as u32) != list_size { return Err("list size and numbers provided must be of the same length" ); }

        Ok(Config { list_size, list_of_numbers })
    }
}

fn calculate_max_pairwise_product(config: Config) -> u32 {
    let mut first_number = Number { value: 0, index: 0 };

    for (i, n) in config.list_of_numbers.iter().enumerate() {
        if n > &first_number.value {
            first_number.value = *n;
            first_number.index = i;
        }
    }

    let mut second_number = Number { value: 0, index: 0 };

    for (i, n) in config.list_of_numbers.iter().enumerate() {
        if n > &second_number.value && i != first_number.index {
            second_number.value = *n;
            second_number.index = i;
        }
    }

    first_number.value * second_number.value
}

pub fn run() -> Result<(u32), Box<dyn Error>> {
    let list_size = ask::number()?;
    let list_of_numbers = ask::list_of_numbers()?;
    let config = Config::new(list_size, list_of_numbers)?;

    Ok(calculate_max_pairwise_product(config))
}
