use std::env;
use std::path::Path;
use std::{fs,io};

mod dfa;
mod parser;

// Usage: automatoy <automaton file> <string to parse>
// returns 0 if recognized, 1 if not
fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    assert!(args.len() >= 3,"Usage: automatoy <automaton> <string>");

    let dfa_path = Path::new(&args[1]);
    let query_string = args[2].clone();
    assert!(dfa_path.exists(), "Error: path to automaton does not exist");

    let dfa_string = fs::read_to_string(dfa_path.as_os_str())?;

    parser::parse(&dfa_string);
    
    Ok(())
}
