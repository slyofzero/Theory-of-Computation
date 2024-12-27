mod grammar;

use grammar::Grammar;

fn main() {
    let variables  = vec!["S", "L", "L'"];
    let terminals  = vec!["(", ")", "a", ","];
    let start = "S";

    let mut grammar = Grammar::new(variables, terminals, start);
    grammar.add_rule("S", vec!["(", "L", ")"]);
    grammar.add_rule("S", vec!["a"]);
    grammar.add_rule("L", vec!["S", "L'"]);
    grammar.add_rule("L'", vec![""]);
    grammar.add_rule("L'", vec![",", "S", "L'"]);
}