use std::env;
use std::fs;
use std::error::Error;

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

pub fn search<'a>(query: &str, content: &'a str) ->Vec<&'a str> {
    let mut results = Vec::new();
    for line in content.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}

pub fn search_case_insensitive<'a>(query: &str, content: &'a str) ->Vec<&'a str> {
    let mut results = Vec::new();
    let query_lowercase = query.to_lowercase();

    for line in content.lines() {
        if line.to_lowercase().contains(&query_lowercase) {
            results.push(line);
        }
    }
    results
}

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        let query = args[1].clone();
        let filename = args[2].clone();
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config { query, filename, case_sensitive })
    }

    pub fn build_from_args() -> Result<Config, &'static str> {
        let args: Vec<String> = env::args().collect();

        if args.len() < 3 {
            return Err("Você precisa fornecer 2 argumentos  ")
        }

        Config::new(&args)
    }
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
            vec!["Pedra, papel e tesoura", 
                "O papel ganha da pedra, mas perde da tesoura"],
            search_case_insensitive(query, contents)
        );
    }
}