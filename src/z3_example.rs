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

#[derive(Debug)]
enum Atom {
    Var(String),
    Int(i64),
}

#[derive(Debug)]
enum Arith {
    Add(Box<Arith>, Box<Arith>),
    Sub(Box<Arith>, Box<Arith>),
    Mul(Box<Arith>, Box<Arith>),
    Div(Box<Arith>, Box<Arith>),
    Atom(Box<Atom>),
}

#[derive(Debug)]
enum Comparison {
    Eq(Box<Arith>, Box<Arith>),
    Neq(Box<Arith>, Box<Arith>),
    Lt(Box<Arith>, Box<Arith>),
    Le(Box<Arith>, Box<Arith>),
    Gt(Box<Arith>, Box<Arith>),
    Ge(Box<Arith>, Box<Arith>),
}

#[derive(Debug)]
enum BoolExpr {
    True,
    False,
    Not(Box<BoolExpr>),
    And(Box<BoolExpr>, Box<BoolExpr>),
    Or(Box<BoolExpr>, Box<BoolExpr>),
    Implies(Box<BoolExpr>, Box<BoolExpr>),
    Comparison(Box<Comparison>),
}

#[derive(Debug)]
enum Statement {
    Let(String, Box<Arith>),
    If(Box<BoolExpr>, Block, Block),
}

type Block = Vec<Statement>;

fn example_program() -> Block {
    vec![
        Statement::Let("x".to_string(), Box::new(Arith::Atom(Box::new(Atom::Int(5))))),
    ]
}

pub fn run_all_examples() {
    solve_linear_equations();
    let program = example_program();
    println!("Example program: {:?}", program);
    //solve_inequalities();
    //solve_boolean_formula();
    //optimization_example();
}
