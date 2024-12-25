#![allow(dead_code, unused_assignments)]
use std::{collections::{HashMap, HashSet}, str};

type State = String;
type Alphabet = char;

type StatesSet = HashSet<String>;
type AlphabetsSet = HashSet<Alphabet>;
type TransitionTable = HashMap<State, HashMap<Alphabet, State>>;

pub struct DFA {
    states: StatesSet,
    alphabets: AlphabetsSet,
    transitions: TransitionTable,
    start_state: State,
    final_states: StatesSet
}

impl DFA {
    pub fn new(
        states: StatesSet,
        alphabets: AlphabetsSet,
        transitions: TransitionTable,
        start_state: State,
        final_states: StatesSet) -> Self {
        DFA { states, alphabets, transitions, start_state, final_states }
    }

    pub fn check(&self, input_string: &str) -> bool {
        let mut current_state = &self.start_state;

        for char in input_string.chars() {
            if let Some(current_state_transitions) = self.transitions.get(current_state) {
                if let Some(next_state) = current_state_transitions.get(&char) {
                    current_state = next_state;
                } else {
                    return false;
                }
            } else {
                return false;
            }
        }

        self.final_states.contains(current_state)
    }
}

fn set_from_array(array: &[&str]) -> StatesSet {
    array.iter().map(|&s| s.to_string()).collect()
}

fn create_dfa() -> DFA {
    let states = set_from_array(&["q0", "q1", "q2"]);
    let final_states = set_from_array(&["q2"]);
    let alphabets = HashSet::from(['a', 'b']);
    let start_state = "q0".to_string();
    let mut transitions: TransitionTable = HashMap::new();

    transitions.insert("q0".to_string(), HashMap::from([
        ('a', "q1".to_string()),
        ('b', "q0".to_string()),
    ]));

    transitions.insert("q1".to_string(), HashMap::from([
        ('a', "q1".to_string()),
        ('b', "q2".to_string()),
    ]));

    transitions.insert("q2".to_string(), HashMap::from([
        ('a', "q1".to_string()),
        ('b', "q0".to_string()),
    ]));

    let dfa = DFA::new(states, alphabets, transitions, start_state, final_states);
    dfa
}

#[test]
fn test_dfa_accepts_ab() {
    let dfa = create_dfa();
    assert_eq!(dfa.check("ab"), true);
}

#[test]
fn test_dfa_rejects_ba() {
    let dfa = create_dfa();
    assert_eq!(dfa.check("ba"), false);
}

#[test]
fn test_dfa_accepts_aaab() {
    let dfa = create_dfa();
    assert_eq!(dfa.check("aaab"), true);
}

#[test]
fn test_dfa_accepts_bbab() {
    let dfa = create_dfa();
    assert_eq!(dfa.check("bbab"), true);
}

#[test]
fn test_dfa_accepts_long_string() {
    let dfa = create_dfa();
    assert_eq!(dfa.check("ababababababbabaabbbaabab"), true);
}

#[test]
fn test_dfa_rejects_empty_string() {
    let dfa = create_dfa();
    assert_eq!(dfa.check(""), false);
}

#[test]
fn test_dfa_rejects_single_a() {
    let dfa = create_dfa();
    assert_eq!(dfa.check("a"), false);
}

#[test]
fn test_dfa_rejects_single_b() {
    let dfa = create_dfa();
    assert_eq!(dfa.check("b"), false);
}

#[test]
fn test_dfa_accepts_baab() {
    let dfa = create_dfa();
    assert_eq!(dfa.check("baab"), true);
}

#[test]
fn test_dfa_accepts_caab() {
    let dfa = create_dfa();
    assert_eq!(dfa.check("caab"), false);
}