use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = parse_config(&args);
    // sanity checks
    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    // read the file
    let contents = fs::read_to_string(config.file_path)
        .expect("Should have been able to read the file");

    println!("With text:\n{contents}")
}

struct Config {
    query: String,
    file_path: String,
}
fn parse_config(args: &[String]) -> Config {
    // Discarding &args[0] for the purpose of this simple program
    let query = args[1].clone();
    let file_path = args[2].clone();
    
    Config { query, file_path }
}
