extern crate regex;

use regex::Regex;
use std::error::Error;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;

pub mod config;

use config::Config;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let filtered_contents = if &config.filename == "-" {
        let stdin = io::stdin();
        let reader = stdin.lock();
        search(reader, &config.query)
    } else {
        let f = File::open(config.filename).unwrap();
        let reader = BufReader::new(f);
        search(reader, &config.query)
    };

    for line in filtered_contents {
        println!("{}", line);
    }

    Ok(())
}

pub fn search<T: BufRead + Sized>(reader: T, query: &str) -> Vec<String> {
    let regex_expression = Regex::new(&query).unwrap();
    let mut results = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();
        if regex_expression.find(&line).is_some() {
            results.push(line);
        }
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_case_sensitive() {
        let query = "pedra";
        let contents = "\
Pedra, papel e tesoura
O papel ganha da pedra, mas perde da tesoura
A tesoura ganha do papel
        "
        .as_bytes();

        assert_eq!(
            vec!["O papel ganha da pedra, mas perde da tesoura"],
            search(contents, &query)
        );
    }

    #[test]
    fn find_case_insensitive() {
        let query = "(?i)pEDrA";
        let contents = "\
Pedra, papel e tesoura
O papel ganha da pedra, mas perde da tesoura
A tesoura ganha do papel
        "
        .as_bytes();

        assert_eq!(
            vec![
                "Pedra, papel e tesoura",
                "O papel ganha da pedra, mas perde da tesoura"
            ],
            search(contents, &query)
        );
    }
}
