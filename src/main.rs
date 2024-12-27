mod grammar;
mod ll1;

use grammar::Grammar;
use ll1::LL1;

fn main() {
    // let variables  = vec!["S", "L", "L'"];
    // let terminals  = vec!["(", ")", "a", ","];
    // let start = "S";

    // let mut grammar = Grammar::new(variables, terminals, start);
    // grammar.add_rule("S", vec!["(", "L", ")"]);
    // grammar.add_rule("S", vec!["a"]);
    // grammar.add_rule("L", vec!["S", "L'"]);
    // grammar.add_rule("L'", vec![""]);
    // grammar.add_rule("L'", vec![",", "S", "L'"]);

    let variables  = vec!["S", "A"];
    let terminals  = vec!["a", "b"];
    let start = "S";

    let mut grammar = Grammar::new(variables, terminals, start);
    grammar.add_rule("S", vec!["A", "A"]);
    grammar.add_rule("A", vec!["a", "A"]);
    grammar.add_rule("A", vec!["b"]);

    let mut ll1 = LL1::new(grammar);
    println!("{}", ll1.check("abba"));
}