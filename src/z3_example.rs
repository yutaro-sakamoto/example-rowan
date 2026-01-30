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

#[derive(Debug, Clone)]
enum Atom {
    Var(String),
    Int(i64),
}

#[derive(Debug, Clone)]
enum Arith {
    Add(Box<Arith>, Box<Arith>),
    Sub(Box<Arith>, Box<Arith>),
    Mul(Box<Arith>, Box<Arith>),
    Div(Box<Arith>, Box<Arith>),
    Atom(Box<Atom>),
}

#[derive(Debug, Clone)]
enum Comparison {
    Eq(Box<Arith>, Box<Arith>),
    Neq(Box<Arith>, Box<Arith>),
    Lt(Box<Arith>, Box<Arith>),
    Le(Box<Arith>, Box<Arith>),
    Gt(Box<Arith>, Box<Arith>),
    Ge(Box<Arith>, Box<Arith>),
}

#[derive(Debug, Clone)]
enum BoolExpr {
    True,
    False,
    Not(Box<BoolExpr>),
    And(Box<BoolExpr>, Box<BoolExpr>),
    Or(Box<BoolExpr>, Box<BoolExpr>),
    Implies(Box<BoolExpr>, Box<BoolExpr>),
    Comparison(Box<Comparison>),
}

#[derive(Debug, Clone)]
enum Statement {
    Let(String, Box<Arith>),
    If(Box<BoolExpr>, Block, Block),
}

type StatementVisited<'a> = (&'a Statement, bool);

type Block = Vec<Statement>;
type ExecutionPoint<'a> = std::slice::Iter<'a, Statement>;

fn example_program() -> Block {
    vec![
        Statement::Let("x".to_string(), Box::new(Arith::Atom(Box::new(Atom::Int(5))))),
    ]
}

enum PathBranch {
    Then,
    Else,
    Finish,
}

fn forward_until_branch(execution_point: &mut ExecutionPoint) -> bool {
    while let Some(statement) = execution_point.next() {
        match statement {
            Statement::If(_, _, _) => return true,
            _ => continue,
        }
    }
    false
}

fn init_visited_statements(statements: &Block) -> Vec<StatementVisited> {
    statements.iter().map(|s| (s, false)).collect()
}

//fn find_paths_covers_all_edges(statements: Block) {
//    let mut execution_point = statements.iter();
//    let mut visited_statements = init_visited_statements(&statements);
//    let mut path_queue: Vec<Vec<PathBranch>> = Vec::new();
//
//    while let Some(path) = path_queue.pop() {
//
//    }
//}

pub fn run_all_examples() {
    solve_linear_equations();
    let program = example_program();
    println!("Example program: {:?}", program);
    //solve_inequalities();
    //solve_boolean_formula();
    //optimization_example();
}
