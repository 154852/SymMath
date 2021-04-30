use symmath::expr::*;
use symmath::simplify::*;

fn main() {
    // let mut expr = Expr::pi() + Expr::frac(1, 2) + Expr::frac(4, 5) + Expr::frac(3, 2) * (Expr::frac(4, 2) * Expr::int(10)) + Expr::var("x")*Expr::int(0);
    let mut expr = (Expr::var("x") + Expr::frac(1, 2)) / Expr::frac(2, 1);
    println!("Before simplify: {}", expr.to_ascii());
    expr.simplify(&SimplifcationOpts::expand());
    expr.simplify(&SimplifcationOpts::integers());
    println!("After simplify: {}", expr.to_ascii());
}