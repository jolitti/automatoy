# automatoy
Toy project for a deterministic finite automaton interpreter written in rust

Usage: `cargo run <dfa file> <string to test> -v/--verbose`
Where `<dfa file>` is the location of a file that describes a DFA (see folder `samples` for the syntax) 
and `<string to test>` is the desired string you wish to test

`--verbose`: enable verbose mode (print each state reached during execution)

Returns `0` for a string that matches, `1` for a non-matching string, and `2` for a badly-defined DFA 
or for a string that contains characters not defined for the automaton
