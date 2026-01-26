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

}

///// 不等式制約の例
///// x > 0
///// y > 0
///// x + y < 10
///// 2*x + y > 12
//pub fn solve_inequalities() {
//    println!("\n=== 不等式制約を解く ===");
//    let cfg = Config::new();
//    let ctx = Context::new(&cfg);
//    let solver = Solver::new(&ctx);
//
//    let x = Int::new_const(&ctx, "x");
//    let y = Int::new_const(&ctx, "y");
//
//    // 制約を追加
//    solver.assert(&x.gt(&Int::from_i64(&ctx, 0)));
//    solver.assert(&y.gt(&Int::from_i64(&ctx, 0)));
//    solver.assert(&x.add(&[&y]).lt(&Int::from_i64(&ctx, 10)));
//    solver.assert(
//        &Int::from_i64(&ctx, 2)
//            .mul(&[&x])
//            .add(&[&y])
//            .gt(&Int::from_i64(&ctx, 12)),
//    );
//
//    match solver.check() {
//        SatResult::Sat => {
//            let model = solver.get_model().unwrap();
//            println!("解が見つかりました:");
//            println!("  x = {}", model.eval(&x, true).unwrap());
//            println!("  y = {}", model.eval(&y, true).unwrap());
//        }
//        SatResult::Unsat => println!("解なし（制約が矛盾しています）"),
//        SatResult::Unknown => println!("不明"),
//    }
//}
//
///// ブール論理の例（SAT問題）
///// (a OR b) AND (NOT a OR c) AND (NOT b OR NOT c)
//pub fn solve_boolean_formula() {
//    println!("\n=== ブール論理式を解く ===");
//    let cfg = Config::new();
//    let ctx = Context::new(&cfg);
//    let solver = Solver::new(&ctx);
//
//    let a = z3::ast::Bool::new_const(&ctx, "a");
//    let b = z3::ast::Bool::new_const(&ctx, "b");
//    let c = z3::ast::Bool::new_const(&ctx, "c");
//
//    // (a OR b)
//    let clause1 = z3::ast::Bool::or(&ctx, &[&a, &b]);
//    // (NOT a OR c)
//    let clause2 = z3::ast::Bool::or(&ctx, &[&a.not(), &c]);
//    // (NOT b OR NOT c)
//    let clause3 = z3::ast::Bool::or(&ctx, &[&b.not(), &c.not()]);
//
//    // すべての節を追加
//    solver.assert(&clause1);
//    solver.assert(&clause2);
//    solver.assert(&clause3);
//
//    match solver.check() {
//        SatResult::Sat => {
//            let model = solver.get_model().unwrap();
//            println!("充足可能です:");
//            println!("  a = {}", model.eval(&a, true).unwrap());
//            println!("  b = {}", model.eval(&b, true).unwrap());
//            println!("  c = {}", model.eval(&c, true).unwrap());
//        }
//        SatResult::Unsat => println!("充足不可能"),
//        SatResult::Unknown => println!("不明"),
//    }
//}
//
///// 最適化の例：目的関数を最大化
///// x + y を最大化（制約付き）
//pub fn optimization_example() {
//    println!("\n=== 最適化の例 ===");
//    let cfg = Config::new();
//    let ctx = Context::new(&cfg);
//    let opt = z3::Optimize::new(&ctx);
//
//    let x = Int::new_const(&ctx, "x");
//    let y = Int::new_const(&ctx, "y");
//
//    // 制約
//    opt.assert(&x.ge(&Int::from_i64(&ctx, 0)));
//    opt.assert(&y.ge(&Int::from_i64(&ctx, 0)));
//    opt.assert(&x.add(&[&y]).le(&Int::from_i64(&ctx, 10)));
//    opt.assert(&Int::from_i64(&ctx, 2).mul(&[&x]).add(&[&y]).le(&Int::from_i64(&ctx, 15)));
//
//    // 目的関数: x + y を最大化
//    opt.maximize(&x.add(&[&y]));
//
//    match opt.check(&[]) {
//        SatResult::Sat => {
//            let model = opt.get_model().unwrap();
//            println!("最適解が見つかりました:");
//            println!("  x = {}", model.eval(&x, true).unwrap());
//            println!("  y = {}", model.eval(&y, true).unwrap());
//            println!("  x + y = {}", model.eval(&x.add(&[&y]), true).unwrap());
//        }
//        SatResult::Unsat => println!("解なし"),
//        SatResult::Unknown => println!("不明"),
//    }
//}

pub fn run_all_examples() {
    solve_linear_equations();
    //solve_inequalities();
    //solve_boolean_formula();
    //optimization_example();
}
