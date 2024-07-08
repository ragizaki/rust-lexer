mod scanner;
mod token;

use scanner::Scanner;
use std::env;
use std::fs;

const LEXICAL_ERROR_CODE: i32 = 65;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} tokenize <filename>", args[0]);
        return;
    }

    let command = &args[1];
    let filename = &args[2];

    match command.as_str() {
        "tokenize" => {
            let file_contents = fs::read_to_string(filename).expect("Failed to read file");

            let mut scanner = Scanner::new(&file_contents);
            let tokens = scanner.scan_tokens();

            for token in tokens {
                println!("{token}");
            }

            if scanner.error_found {
                std::process::exit(LEXICAL_ERROR_CODE);
            }
        }
        _ => {
            eprint!("Unknown command: {}", command);
        }
    }
}
