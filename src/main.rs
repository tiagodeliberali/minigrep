use std::process;
use minigrep::Config;

fn main() {
    let config = Config::build_from_args().unwrap_or_else(|err| {
        eprintln!("{}", err);
        process::exit(1);
    });

    if let Err(err) = minigrep::run(config) {
        eprintln!("Não deu para rodar: {}", err);
        process::exit(1);
    }
}
