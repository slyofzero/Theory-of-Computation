#![allow(dead_code)]
use std::collections::{HashMap, HashSet};

type InputSymbolSet = Vec<&'static str>;
type SymbolSet = HashSet<String>;

#[derive(Debug, Clone)]
pub(crate) struct Grammar {
    pub(crate) variables: SymbolSet,
    pub(crate) terminals: SymbolSet,
    pub(crate) rules: HashMap<String, Vec<Vec<String>>>,
    pub(crate) start: String,
    first_set: HashMap<String, SymbolSet>,
    follow_set: HashMap<String, SymbolSet>,
    symbol_index: HashMap<String, Vec<(String, usize, usize)>>
}

impl Grammar {
    fn convert_input_set<T>(input: InputSymbolSet) -> T 
    where T: FromIterator<String> {
        input.into_iter().map(|s| s.to_string()).collect()
    }

    pub(crate) fn new(variables: InputSymbolSet, terminals: InputSymbolSet, start: &str) -> Self {
        let variables: SymbolSet = Self::convert_input_set(variables);
        let terminals: SymbolSet = Self::convert_input_set(terminals);
        Grammar { 
            variables, 
            terminals, 
            rules: HashMap::new(), 
            start: start.to_string(),
            first_set: HashMap::new(),
            follow_set: HashMap::new(),
            symbol_index: HashMap::new()
        }
    }

    pub(crate) fn add_rule(&mut self, variable: &str, rules: InputSymbolSet) {
        let variable = variable.to_string();
        let rules: Vec<String> = Self::convert_input_set(rules);
        self.rules.entry(variable).or_insert_with(Vec::new).push(rules);
    }

    pub(crate) fn first(&mut self, variable: &str) -> &SymbolSet {
        if !self.first_set.contains_key(variable) {
            self.calculate_first(variable);
        }
        self.first_set.get(&variable.to_string()).unwrap()
    }

    fn calculate_first(&mut self, variable: &str) -> SymbolSet {
        let mut first_set = HashSet::new();
        if let Some(variable_rules) = self.rules.clone().get(variable) {
            for rule in variable_rules {
                let first = rule[0].clone();
                
                if first.len() > 0 && first.chars().all(|c| c.is_uppercase()) {
                    if let Some(to_extend) = self.first_set.get(&first) {
                        first_set.extend(to_extend.clone());
                    } else {
                        first_set.extend(self.calculate_first(first.as_str()));
                    }
                } else {
                    first_set.insert(first);
                }
            }
        }

        self.first_set.entry(variable.to_string()).or_insert_with(|| first_set.clone());
        first_set
    }

    fn create_symbol_index(&mut self) {
        for (lhs, rules) in &self.rules {
            for (rule_pos, rule) in rules.iter().enumerate() {
                for (pos, symbol) in rule.iter().enumerate() {
                    self.symbol_index.entry(symbol.clone()).or_insert_with(Vec::new).push((lhs.clone(), rule_pos, pos));
                }
            }
        }
    }

    pub(crate) fn follow(&mut self, variable: &str) -> &SymbolSet {
        if !self.follow_set.contains_key(variable) {
            self.calculate_follow(variable);
        }
        self.follow_set.get(&variable.to_string()).unwrap()
    }

    fn calculate_follow(&mut self, variable: &str) -> SymbolSet {
        if self.symbol_index.is_empty() {
            self.create_symbol_index();
        }

        let mut follow_set = HashSet::new();

        if let Some(symbol_occurrences) = self.symbol_index.get(variable).cloned() {
            for (rule_var, rule_pos, symbol_pos) in symbol_occurrences {
                if let Some(next_symbol) = self
                    .rules.clone()
                    .get(&rule_var)
                    .and_then(|rules| rules.get(rule_pos))
                    .and_then(|rule| rule.get(symbol_pos + 1))
                {
                    if self.variables.contains(next_symbol.as_str()) {
                        let first_set: Vec<_> = self.first(next_symbol).iter().cloned().collect();
                        for first in first_set {
                            if first != "" {
                                follow_set.insert(first);
                            } else if rule_var != variable {
                                follow_set.extend(self.calculate_follow(&rule_var));
                            }
                        }
                    } else {
                        follow_set.insert(next_symbol.clone());
                    }
                } else if rule_var != variable {
                    follow_set.extend(self.calculate_follow(&rule_var));
                }
            }
        }

        self.follow_set.entry(variable.to_string()).or_insert_with(|| follow_set.clone());

        follow_set
    }

}