use std::collections::{HashSet,HashMap};

/// Deterministic finite automaton
#[derive(Debug)]
struct Dfa {
    /// Set of acceptable characters in the consumed string
    /// mapped to corresponding index
    alphabet: HashMap<char,usize>,

    /// Set of final states
    final_states: HashSet<usize>,

    /// Set of transitions
    /// memorized as vectors of new states, per state
    transitions: Vec<Vec<usize>>,

    /// Current state
    state: usize
}

impl Dfa {

    /// True if current state is in final
    fn is_in_final(&self) -> bool {
        self.final_states.contains(&self.state)
    }

    /// True if character is in alphabet
    fn is_valid_char(&self,c:char) -> bool {
        self.alphabet.contains_key(&c)
    }

    /// Performs transition consuming the given character
    /// if character is valid, return whether new state is final
    fn transition(&mut self,c:char) -> Option<bool> {
        let c_index: usize = *self.alphabet.get(&c)?;
        
        self.state = *self.transitions.get(self.state)?.get(c_index)?;

        Some(self.is_in_final())
    }

}
