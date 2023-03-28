use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    // Discarding &args[0] for the purpose of this simple program
    let query = &args[1];
    let file_path = &args[2];
   
    // sanity checks
    println!("Searching for {}", query);
    println!("In file {}", file_path);

    // read the file
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    println!("With text:\n{contents}")
}
