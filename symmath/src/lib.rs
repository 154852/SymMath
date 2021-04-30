pub mod expr;
pub mod num;
pub mod simplify;

#[cfg(test)]
mod tests {
    use crate::expr::*;

    #[test]
    fn create_eqn() {
        let expr = Expr::frac(1, 2) + Expr::frac(3, 2);
        println!("{}", expr.to_ascii());
    }
}
