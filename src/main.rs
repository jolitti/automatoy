use std::env;
use std::path::Path;

mod dfa;
mod parser;

// Usage: automatoy <automaton file> <string to parse>
// returns 0 if recognized, 1 if not
fn main() {
    let args: Vec<String> = env::args().collect();
    assert!(args.len() >= 3,"Usage: automatoy <automaton> <string>");

    let dfa_path = Path::new(&args[1]);
    let query_string = args[2].clone();
    assert!(dfa_path.exists(), "Error: path to automaton does not exist");

    

}
