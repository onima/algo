use std::process;
use std::io;

pub fn input(request: String) -> String {
    println!("{}", request);

    fn input() -> Result<String, io::Error> {
        let reader = io::stdin();
        let mut new_str = String::new();
        reader.read_line(&mut new_str)?;
        Ok(new_str)
    }

    return input().unwrap_or_else(|err| {
        println!("Problem parsing input: {}", err);
        process::exit(1);
    });
}