use std::fs::File;
use std::io::{BufRead, BufReader};
use std::env;

fn main() {
    // let args: Vec<String> = env::args().collect();
    
    // Prints each argument on a separate line
    for argument in env::args() {
        println!("{argument}");
    }

    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let file = File::open(filename);
    let file = match file {
        Ok(file) => file,
        Err(error) => {
            match error.kind() {
                std::io::ErrorKind::NotFound => {
                    panic!("File not found: {}", error)
                }
                _ => {
                    panic!("Error opening file: {}", error)
                }
            }
        }
    };
    
    let reader = BufReader::new(file);
    for line in reader.lines() {
        match line {
            Ok(line) => println!("{}", line),
            Err(error) => {
                panic!("Error reading line: {}", error)
            }
        }
    }
}