mod s_expression;
mod z3_example;

use s_expression::*;

fn main() {
    println!("=== Z3使用例 ===\n");
    z3_example::run_all_examples();
}
