#![allow(dead_code, unused_variables)]
use std::collections::HashMap;

use crate::grammar::{Grammar, Rules, Symbol};
type ParseTable = HashMap<Symbol, HashMap<Symbol, Rules>>;

#[derive(Debug)]
pub(crate) struct LL1 {
    stack: Vec<String>,
    grammar: Grammar,
    parse_table: ParseTable
}

impl LL1 {
    pub(crate) fn new(grammar: Grammar) -> Self {
        let parse_table = Self::make_parse_table(&grammar);
        LL1 { stack: Vec::new(), grammar, parse_table }
    }

    fn make_parse_table(grammar: &Grammar) -> ParseTable {
        let mut parse_table: ParseTable = HashMap::new();

        for (variable, rules) in &grammar.rules {
            
        }
        
        parse_table
    }
}