use crate::z3_example::run_all_examples;

mod cobol;
mod s_expression;
mod z3_example;

fn main() {
    println!("=== COBOL Parser Example ===\n");
    cobol::main();
    run_all_examples();
}
