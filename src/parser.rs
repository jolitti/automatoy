use std::collections::{HashMap,HashSet};
use itertools::Itertools;

use crate::dfa::Dfa;

const ALPHA_PREFIX: &str = "ALPHABET: ";
const TRANS_DELIMITER: &str = " -> ";

/// Take a string that describes a dfa, return a struct representing it
pub fn parse(source:&str) -> Option<Dfa> {

    let lines = source.lines()
        .filter(|line| !line.is_empty())
        .filter(|line| !line.starts_with("/"))
        .collect::<Vec<_>>();

    let alphabet: HashMap<char,usize> = lines.get(0)?
        .replace(ALPHA_PREFIX,"")
        .chars()
        .unique()
        .enumerate()
        .map(|(i,c)| (c,i))
        .collect();
    //dbg!(&alphabet);

    let states: HashMap<String,usize> = lines.iter().skip(2)
        .map(|s| {
            match s.split_once(TRANS_DELIMITER) {
                Some((head,_tail)) => head,
                None => s
            }
        }
        )
        .map(|s| s.replace("*",""))
        .enumerate()
        .map(|(i,c)| (c,i))
        .collect();
    //dbg!(&states);

    let final_states: HashSet<usize> = lines.iter().skip(2)
        .enumerate()
        .filter(|(_i,l)| l.starts_with("*"))
        .map(|(i,_l)| i)
        .collect();
    //dbg!(&final_states);

    let mut transitions: Vec<Vec<usize>> = Vec::new();
    for line in lines.iter().skip(2) {
        if let Some((_head,tail)) = line.split_once(TRANS_DELIMITER) {
            let new_trans: Vec<usize> = tail.split(" ")
                .map(|st| states[st])
                .collect();
            transitions.push(new_trans);
        }
    }
    // dbg!(&transitions);
    if transitions.len() != states.len() { return None; }

    Some(
        Dfa::new(alphabet,final_states,transitions)
        )
}
