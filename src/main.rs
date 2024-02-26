use std::path::Path;
use std::fs;
use std::process;
use clap::Parser;

mod dfa;
mod parser;


const IN_LANGUAGE: i32 = 0;
const NOT_IN_LANGUAGE: i32 = 1;
const INVALID_DFA: i32 = 2;


/// Simple program to compute deterministic finite automata
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// File of the DFA descriptor
    dfa_file: String,

    /// String to be tested
    string: String,

    /// Verbose mode (output the current state at each step)
    #[arg(short, long, default_value_t = true)]
    verbose: bool
}

// Usage: automatoy <automaton file> <string to parse>
// returns 0 if recognized, 1 if not
// 2 if dfa is invalid
fn main() {

    let args = Args::parse();

    //let dfa_path = Path::new(&args2[1]);
    let dfa_path = Path::new(&args.dfa_file);
    let query_string = args.string;
    assert!(dfa_path.exists(), "Error: path to automaton does not exist");

    let Ok(dfa_string) = fs::read_to_string(dfa_path.as_os_str()) else {
        process::exit(INVALID_DFA);
    };

    let Some(mut dfa) = parser::parse(&dfa_string) else {
        process::exit(INVALID_DFA);
    };

    if args.verbose {
        println!("{}",dfa.current_state_name());
    }

    for c in query_string.chars() {
        if !dfa.is_valid_char(c) {
            println!("Error: {c} is not a valid character for this dfa");
            process::exit(INVALID_DFA);
        }
        dfa.transition(c);
        if args.verbose {
            println!("{}",dfa.current_state_name());
        }
    }

    match dfa.is_in_final() {
        true => process::exit(IN_LANGUAGE),
        false => process::exit(NOT_IN_LANGUAGE)
    }
    
}
