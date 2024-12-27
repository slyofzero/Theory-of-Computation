#![allow(dead_code, unused_assignments)]
use std::{collections::HashMap, ops::{Deref, DerefMut}};

use crate::nfa::{Alphabet, State, StatesSet};

#[derive(Debug)]
pub struct DFATransitionTable (HashMap<State, HashMap<Alphabet, StatesSet>>);

impl Deref for DFATransitionTable {
    type Target =HashMap<State, HashMap<Alphabet, StatesSet>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for DFATransitionTable {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl DFATransitionTable {
    pub fn new() -> Self {
        DFATransitionTable(HashMap::new())
    }

    pub fn insert(&mut self, state: State, char: char, next_states: &StatesSet) {
        self.entry(state).or_insert_with(HashMap::new).insert(char, next_states.clone());
    }
}