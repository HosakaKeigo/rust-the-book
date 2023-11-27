use minigrep::Config;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        std::process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    // read file
    if let Err(e) = minigrep::run(config) {
        println!("Application error: {}", e);
        std::process::exit(1);
    };
}
