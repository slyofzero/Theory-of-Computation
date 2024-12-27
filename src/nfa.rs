#![allow(dead_code, unused_assignments)]

use std::{collections::{HashMap, HashSet}, ops::{Deref, DerefMut}, str};

use crate::dfa::DFATransitionTable;

pub type State = String;
pub type Alphabet = char;

// StatesSet
#[derive(Debug, Clone)]
pub struct StatesSet(HashSet<State>);

impl StatesSet {
    fn new() -> StatesSet {
        StatesSet(HashSet::new())
    }

    fn from(array: &[&str]) -> StatesSet {
        StatesSet(array.iter().map(|&s| s.to_string()).collect())
    }
}

impl Deref for StatesSet {
    type Target = HashSet<State>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for StatesSet {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

type AlphabetsSet = HashSet<Alphabet>;
type TransitionTable = HashMap<State, HashMap<Alphabet, StatesSet>>;

// NFA
pub struct NFA {
    states: StatesSet,
    alphabets: AlphabetsSet,
    transitions: TransitionTable,
    start_state: State,
    final_states: StatesSet
}

impl NFA {
    pub fn new(
        states: StatesSet,
        alphabets: AlphabetsSet,
        transitions: TransitionTable,
        start_state: State,
        final_states: StatesSet) -> Self {
        NFA { states, alphabets, transitions, start_state, final_states }
    }

    fn get_next_states(&self, current_states: StatesSet, char: char) -> StatesSet {
        let mut next_states = StatesSet::new();

        for state in current_states.iter() {
            if let Some(current_state_transitions) = self.transitions.get(state) {
                if let Some(next_state_set) = current_state_transitions.get(&char) {
                    for next_state in next_state_set.iter() {
                        next_states.insert(next_state.clone());
                    }
                }
            }
        }

        next_states
    }

    pub fn check(&self, input_string: &str) -> bool {
        let mut current_states = StatesSet::from(&[&self.start_state]);

        for char in input_string.chars() {
            current_states = self.get_next_states(current_states, char);
        }

        for current_state in current_states.iter() {
            if self.final_states.contains(current_state) {
                return true;
            }
        }

        false
    }

    pub fn to_dfa(&self) {
        let mut dfa_transitions = DFATransitionTable::new();
        let mut current_state = &self.start_state;
        let mut states_added = StatesSet::new();

        if states_added.get(current_state).is_none() {
            states_added.insert(current_state.to_string());
            if let Some(current_transitions) = self.transitions.get(current_state) {
                for (char, next_states) in current_transitions {
                    dfa_transitions.insert(current_state.clone(), *char, &next_states);
                }
                println!("{:?}", dfa_transitions);
            }
        }
    }
}

pub fn create_nfa() -> NFA {
    let states = StatesSet::from(&["q0", "q1", "q2"]);
    let final_states = StatesSet::from(&["q2"]);
    let alphabets = HashSet::from(['a', 'b']);
    let start_state = "q0".to_string();
    let mut transitions: TransitionTable = HashMap::new();

    transitions.insert("q0".to_string(), HashMap::from([
        ('a', StatesSet::from(&["q0", "q1"])),
        ('b', StatesSet::from(&["q0"])),
    ]));

    transitions.insert("q1".to_string(), HashMap::from([
        ('b', StatesSet::from(&["q2"])),
    ]));

    let nfa = NFA::new(states, alphabets, transitions, start_state, final_states);
    nfa
}

#[test]
fn nfa_accepts_ab() {
    let dfa = create_nfa();
    assert_eq!(dfa.check("ab"), true);
}

#[test]
fn nfa_rejects_ba() {
    let dfa = create_nfa();
    assert_eq!(dfa.check("ba"), false);
}

#[test]
fn nfa_accepts_aaab() {
    let dfa = create_nfa();
    assert_eq!(dfa.check("aaab"), true);
}

#[test]
fn nfa_accepts_bbab() {
    let dfa = create_nfa();
    assert_eq!(dfa.check("bbab"), true);
}

#[test]
fn nfa_accepts_long_string() {
    let dfa = create_nfa();
    assert_eq!(dfa.check("ababababababbabaabbbaabab"), true);
}

#[test]
fn nfa_rejects_empty_string() {
    let dfa = create_nfa();
    assert_eq!(dfa.check(""), false);
}

#[test]
fn nfa_rejects_single_a() {
    let dfa = create_nfa();
    assert_eq!(dfa.check("a"), false);
}

#[test]
fn nfa_rejects_single_b() {
    let dfa = create_nfa();
    assert_eq!(dfa.check("b"), false);
}

#[test]
fn nfa_accepts_baab() {
    let dfa = create_nfa();
    assert_eq!(dfa.check("baab"), true);
}

#[test]
fn nfa_accepts_caab() {
    let dfa = create_nfa();
    assert_eq!(dfa.check("caab"), false);
}