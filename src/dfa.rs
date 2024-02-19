use std::collections::{HashSet,HashMap};

/// Deterministic finite automaton
#[derive(Debug)]
pub struct Dfa {
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
    pub fn is_in_final(&self) -> bool {
        self.final_states.contains(&self.state)
    }

    /// True if character is in alphabet
    pub fn is_valid_char(&self,c:char) -> bool {
        self.alphabet.contains_key(&c)
    }

    /// Performs transition consuming the given character
    /// if character is valid, return whether new state is final
    pub fn transition(&mut self,c:char) -> Option<bool> {
        let c_index: usize = *self.alphabet.get(&c)?;
        
        self.state = *self.transitions.get(self.state)?.get(c_index)?;

        Some(self.is_in_final())
    }

    /// Initialize new DFA
    pub fn new(
        alpha:HashMap<char,usize>,
        finals:HashSet<usize>,
        trans:Vec<Vec<usize>>) -> Dfa {
        
        Dfa {
            alphabet: alpha,
            final_states: finals,
            transitions: trans,
            state: 0
        }
    }

}
