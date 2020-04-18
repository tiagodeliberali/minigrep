extern crate clap;

use clap::{App, Arg};

pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn build_from_args() -> Result<Config, &'static str> {
        let args = App::new("minigrep")
            .version("0.1")
            .about("Busca por padrões em arquivos no ou STDIN")
            .arg(
                Arg::with_name("pattern")
                    .help("O padrão a ser buscado. Pode user regex. Exemplo de busca case insensitive: '(?i)pEDrA'.")
                    .takes_value(true)
                    .required(true),
            )
            .arg(
                Arg::with_name("input")
                    .help("O arquivo usado como referência")
                    .takes_value(true)
                    .required(true),
            )
            .get_matches();

        let query = args.value_of("pattern").unwrap().to_string();
        let filename = args.value_of("input").unwrap().to_string();

        Ok(Config { query, filename })
    }
}
