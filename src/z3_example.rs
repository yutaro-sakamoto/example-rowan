use std::vec;

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
        let sol: Vec<u64> = solution
            .iter()
            .map(Int::as_u64)
            .map(Option::unwrap)
            .collect();
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
    If(Box<BoolExpr>, Block),
    IfElse(Box<BoolExpr>, Block, Block),
}

type StatementVisited<'a> = (&'a Statement, bool);

type Block = Vec<Statement>;
type ExecutionPoint<'a> = std::slice::Iter<'a, Statement>;

fn example_program() -> Block {
    vec![
        Statement::Let(
            "x".to_string(),
            Box::new(Arith::Atom(Box::new(Atom::Int(5)))),
        ),
        Statement::IfElse(
            Box::new(BoolExpr::Comparison(Box::new(Comparison::Gt(
                Box::new(Arith::Atom(Box::new(Atom::Var("x".to_string())))),
                Box::new(Arith::Atom(Box::new(Atom::Int(0)))),
            )))),
            vec![Statement::Let(
                "y".to_string(),
                Box::new(Arith::Atom(Box::new(Atom::Int(1)))),
            )],
            vec![Statement::Let(
                "y".to_string(),
                Box::new(Arith::Atom(Box::new(Atom::Int(-1)))),
            )],
        ),
        Statement::If(
            Box::new(BoolExpr::Comparison(Box::new(Comparison::Eq(
                Box::new(Arith::Atom(Box::new(Atom::Var("y".to_string())))),
                Box::new(Arith::Atom(Box::new(Atom::Int(1)))),
            )))),
            vec![Statement::Let(
                "z".to_string(),
                Box::new(Arith::Atom(Box::new(Atom::Int(100)))),
            )],
        ),
    ]
}

#[derive(Debug, Clone)]
enum PathBranch {
    Then,
    Else,
    Finish,
}

fn forward_until_branch<'a>(execution_point: &'a mut ExecutionPoint) -> Option<&'a Statement> {
    while let Some(statement) = execution_point.next() {
        match statement {
            Statement::If(_, _) => return Some(statement),
            Statement::IfElse(_, _, _) => return Some(statement),
            _ => continue,
        }
    }
    None
}

fn init_visited_statements(statements: &Block) -> Vec<StatementVisited> {
    statements.iter().map(|s| (s, false)).collect()
}

fn find_paths_c0_coverage(execution_point: &mut ExecutionPoint) -> Vec<Vec<PathBranch>> {
    if let Some(control_statement) = forward_until_branch(execution_point) {
        println!("Found control statement: {:?}", control_statement);
        match control_statement {
            Statement::If(_, block) => {
                println!("Control Statement1: {:?}", control_statement);
                let then_branches = find_paths_c0_coverage(&mut block.iter().clone())
                    .into_iter()
                    .map(|mut path| {
                        let mut new_path = vec![PathBranch::Then];
                        new_path.append(&mut path);
                        new_path
                    });
                let paths = then_branches.collect::<Vec<_>>();
                paths
            }
            Statement::IfElse(_, then_block, else_block) => {
                println!("Control Statement2: {:?}", control_statement);
                let then_branches = find_paths_c0_coverage(&mut then_block.iter().clone())
                    .into_iter()
                    .map(|mut path| {
                        let mut new_path = vec![PathBranch::Then];
                        new_path.append(&mut path);
                        new_path
                    });
                let else_branches = find_paths_c0_coverage(&mut else_block.iter().clone())
                    .into_iter()
                    .map(|mut path| {
                        let mut new_path = vec![PathBranch::Else];
                        new_path.append(&mut path);
                        new_path
                    });
                let mut paths = then_branches.collect::<Vec<_>>();
                paths.append(&mut else_branches.collect::<Vec<_>>());
                paths
            }
            _ => vec![],
        }
    } else {
        println!("Not Found");
        vec![vec![PathBranch::Finish]]
    }
}

pub fn run_all_examples() {
    solve_linear_equations();
    let program = example_program();
    println!("Example program: {:?}", program);
    let mut exec_point = program.iter();
    let paths = find_paths_c0_coverage(&mut exec_point);
    println!("C0 Coverage Paths: {:?}", paths);
    //solve_inequalities();
    //solve_boolean_formula();
    //optimization_example();
}
