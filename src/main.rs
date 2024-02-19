use std::env;
use std::path::Path;
use std::fs;
use std::process;

mod dfa;
mod parser;


const IN_LANGUAGE: i32 = 0;
const NOT_IN_LANGUAGE: i32 = 1;
const INVALID_DFA: i32 = 2;

// Usage: automatoy <automaton file> <string to parse>
// returns 0 if recognized, 1 if not
// 2 if dfa is invalid
fn main() {
    let args: Vec<String> = env::args().collect();
    assert!(args.len() >= 3,"Usage: automatoy <automaton> <string>");

    let dfa_path = Path::new(&args[1]);
    let query_string = args[2].clone();
    assert!(dfa_path.exists(), "Error: path to automaton does not exist");

    let dfa_string = fs::read_to_string(dfa_path.as_os_str());
    if let Err(_) = dfa_string {
        process::exit(INVALID_DFA);
    }
    let dfa_string = dfa_string.unwrap();

    let dfa = parser::parse(&dfa_string);
    if let None = dfa {
        process::exit(INVALID_DFA);
    }
    let mut dfa = dfa.unwrap();

    for c in query_string.chars() {
        if !dfa.is_valid_char(c) {
            println!("Error: {c} is not a valid character for this dfa");
            process::exit(INVALID_DFA);
        }
        dfa.transition(c);
    }

    match dfa.is_in_final() {
        true => process::exit(IN_LANGUAGE),
        false => process::exit(NOT_IN_LANGUAGE)
    }
    
}
