use std::error::Error;
use std::fs;

pub mod config;

use config::Config;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&config.filename)?;

    let filtered_contests = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in filtered_contests {
        println!("{}", line);
    }

    Ok(())
}

pub fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in content.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}

pub fn search_case_insensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    let query_lowercase = query.to_lowercase();

    for line in content.lines() {
        if line.to_lowercase().contains(&query_lowercase) {
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
            search(query, contents)
        );
    }

    #[test]
    fn find_case_insensitive() {
        let query = "pEDrA";
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
            search_case_insensitive(query, contents)
        );
    }
}
