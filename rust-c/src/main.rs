use std::env; // Bring the env module
use std::fs;
mod lexer;

fn main() {
    let args: Vec<String> = env::args().collect(); // collect turns iterator into a vector containing all the values produced by the iterator.
    // dbg!(args); // debugging 

    /*
    Option <T> = either
    - Some(T)   // there is a value
    - None      // there is no value 
    args.get(1) = Some(&String) or None
    */
    if let Some(file_path) = args.get(1) { // We have to check whether the file input exists, args.get() returns a value
        println!("Input file is: {file_path}");
        let file_contents = fs::read_to_string(file_path).
                            expect("Should have been able to read the file");
        lexer::lex(&file_contents);
    }
    else {
        eprintln!("Usage: rust-c <file> <optional flag(s)>");
    }
}
