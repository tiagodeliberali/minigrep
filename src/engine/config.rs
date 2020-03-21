use std::env;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Você precisa passar o termo como primeiro parâmetro")
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Você precisa passar o nome do arquivo como segundo parâmetro")
        };

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config { query, filename, case_sensitive })
    }

    pub fn build_from_args() -> Result<Config, &'static str> {
        Config::new(env::args())
    }
}
