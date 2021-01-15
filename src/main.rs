use std::env;
use std::fs;
fn main() {
    let vars: Vec<String> = env::args().skip(1).collect();
    let config = Config::new(&vars);

    let contents = fs::read_to_string(config.filename)
        .expect("Somethign went wrong while reading the string.");
}

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Config {
        if (args.len() < 2) {
            panic!("Not enough arguments")
        }
        let query = args[0].clone();
        let filename = args[1].clone();

        Config { query, filename }
    }
}
