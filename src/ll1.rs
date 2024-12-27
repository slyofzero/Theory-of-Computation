#![allow(dead_code, unused_variables)]
use std::collections::HashMap;

use crate::grammar::Grammar;
type ParseTable = HashMap<String, HashMap<String, Vec<String>>>;

#[derive(Debug)]
pub(crate) struct LL1 {
    stack: Vec<String>,
    grammar: Grammar,
    parse_table: ParseTable
}

impl LL1 {
    pub(crate) fn new(grammar: Grammar) -> Self {
        let parse_table = Self::make_parse_table(&mut grammar.clone());
        LL1 { stack: vec![grammar.start.clone()], grammar, parse_table }
    }

    fn make_parse_table(grammar: &mut Grammar) -> ParseTable {
        let mut parse_table: ParseTable = HashMap::new();

        for (variable, rules) in &grammar.clone().rules {
            for rule in rules {
                let first = rule.get(0).unwrap();
                if grammar.variables.contains(first) {
                    for first in grammar.first(variable) {
                        parse_table.entry(variable.clone()).or_default().insert(first.clone(), rule.clone());
                    }
                } else if first == "" {
                    for follow in grammar.follow(variable) {
                        parse_table.entry(variable.clone()).or_default().insert(follow.clone(), rule.clone());
                    }
                } else {
                    parse_table.entry(variable.clone()).or_default().insert(first.clone(), rule.clone());
                }
            }
        }
        
        parse_table
    }

    pub(crate) fn check(&mut self, string: &str) -> bool {
        let mut index = 0;
        
        while index < string.len() && !self.stack.is_empty() {
            let char = string.chars().nth(index).unwrap();
            if let Some(tos) = self.stack.last() {
                if self.grammar.variables.contains(tos) {
                    if let Some(rule) = self.parse_table.get(tos).and_then(|s| s.get(&char.to_string())) {
                        self.stack.pop();
                        for symbol in rule.iter().rev() {
                            self.stack.push(symbol.to_string());
                        }
                    }
                } else if tos.clone() == char.to_string() {
                    self.stack.pop();
                    index+=1;
                }
            }
        }

        self.stack.is_empty() && index == string.len()
    }
}