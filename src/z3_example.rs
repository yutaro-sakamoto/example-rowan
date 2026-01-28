use z3::ast::{Ast, Int};
use z3::{Config, Context, SatResult, Solver};

/// 基本的なZ3の使用例：線形方程式を解く
/// x + y = 10
/// x - y = 2
pub fn solve_linear_equations() {
    let solver = Solver::new();

    // 変数を作成
    let x = Int::fresh_const("x");
    let y = Int::fresh_const("y");

    solver.assert((&x + &y).eq(30));

    for solution in solver.solutions([&x, &y], false).take(1) {
        let sol: Vec<u64> = solution.iter().map(Int::as_u64).map(Option::unwrap).collect();
        let x_val = sol[0];
        let y_val = sol[1];
        println!("Solution found: x = {}, y = {}", x_val, y_val);
    }

}

pub fn run_all_examples() {
    solve_linear_equations();
    //solve_inequalities();
    //solve_boolean_formula();
    //optimization_example();
}
