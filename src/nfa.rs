#![allow(dead_code, unused_assignments)]
use std::{collections::{HashMap, HashSet}, str};

type State = String;
type Alphabet = char;

#[derive(Debug)]
pub struct StatesSet(HashSet<State>);

impl StatesSet {
    fn new() -> StatesSet {
        StatesSet(HashSet::new())
    }

    fn from(array: &[&str]) -> StatesSet {
        StatesSet(array.iter().map(|&s| s.to_string()).collect())
    }
}

type AlphabetsSet = HashSet<Alphabet>;
type TransitionTable = HashMap<State, HashMap<Alphabet, StatesSet>>;

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

        for state in current_states.0.iter() {
            if let Some(current_state_transitions) = self.transitions.get(state) {
                if let Some(next_state_set) = current_state_transitions.get(&char) {
                    for next_state in next_state_set.0.iter() {
                        next_states.0.insert(next_state.to_string());
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

        for current_state in current_states.0.iter() {
            if self.final_states.0.contains(current_state) {
                return true;
            }
        }

        false
    }

    pub fn to_dfa(&self) {}
}

fn create_nfa() -> NFA {
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