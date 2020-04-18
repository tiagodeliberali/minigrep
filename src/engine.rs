extern crate regex;

use regex::Regex;
use std::error::Error;
use std::fs;

pub mod config;

use config::Config;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&config.filename)?;

    let filtered_contents = search(&config.query, &contents);

    for line in filtered_contents {
        println!("{}", line);
    }

    Ok(())
}

pub fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let regex_expression = Regex::new(&query).unwrap();
    let mut results = Vec::new();
    for line in content.lines() {
        if let Some(_) = regex_expression.find(&line) {
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
        ";

        assert_eq!(
            vec!["O papel ganha da pedra, mas perde da tesoura"],
            search(&query, contents)
        );
    }

    #[test]
    fn find_case_insensitive() {
        let query = "(?i)pEDrA";
        let contents = "\
Pedra, papel e tesoura
O papel ganha da pedra, mas perde da tesoura
A tesoura ganha do papel
        ";

        assert_eq!(
            vec![
                "Pedra, papel e tesoura",
                "O papel ganha da pedra, mas perde da tesoura"
            ],
            search(&query, contents)
        );
    }
}
