use std::ops::{Add, Sub, Mul, Div};
use crate::num;

pub enum FuncIdentifier {
    Name(String),
    Sine,
    Cosine,
    Tangent,
    Abs
}

impl FuncIdentifier {
    pub fn get_name(&self) -> String {
        match self {
            FuncIdentifier::Name(name) => name.clone(),
            FuncIdentifier::Sine => String::from("sin"),
            FuncIdentifier::Cosine => String::from("cos"),
            FuncIdentifier::Tangent => String::from("tan"),
            FuncIdentifier::Abs => String::from("abs")
        }
    }
}

pub enum Constant {
    Pi,
    E
}

impl Constant {
    pub fn get_name(&self) -> String {
        match self {
            Constant::Pi => String::from("pi"),
            Constant::E => String::from("e"),
        }
    }
}

pub enum Operator {
    Frac(num::Frac),
    Add,
    Mul,
    Div,
    Sub,
    Pow,
    Name(String),
    Func(FuncIdentifier),
    Const(Constant)
}

pub struct Expr {
    pub(crate) operator: Operator,
    pub(crate) elements: Option<Vec<Expr>>
}

impl Expr {
    pub fn new(operator: Operator, elements: Option<Vec<Expr>>) -> Expr {
        Expr {
            operator, elements
        }
    }

    pub fn new_empty(operator: Operator) -> Expr {
        Expr {
            operator,
            elements: None
        }
    }

    pub fn frac(num: i64, denom: i64) -> Expr {
        Expr {
            operator: Operator::Frac(num::Frac::new(num, denom)),
            elements: None
        }
    }

    pub fn frac_cpy(f: &num::Frac) -> Expr {
        Expr {
            operator: Operator::Frac(num::Frac::new(f.num, f.denom)),
            elements: None
        }
    }

    pub fn int(val: i64) -> Expr {
        Expr {
            operator: Operator::Frac(num::Frac::new(val, 1)),
            elements: None
        }
    }

    pub fn var(name: &str) -> Expr {
        Expr {
            operator: Operator::Name(String::from(name)),
            elements: None
        }
    }
    
    pub fn pi() -> Expr {
        Expr {
            operator: Operator::Const(Constant::Pi),
            elements: None
        }
    }
    
    pub fn e() -> Expr {
        Expr {
            operator: Operator::Const(Constant::E),
            elements: None
        }
    }

    fn join_operands_ascii(&self, delim: &str) -> String {
        return self.elements.as_ref().expect("Node has no children").iter().map(|el| el.to_ascii()).collect::<Vec<String>>().join(delim);
    }

    pub fn to_ascii(&self) -> String {
        match &self.operator {
            Operator::Frac(frac) => if frac.is_int() { format!("{}", frac.num) } else { format!("({}/{})", frac.num, frac.denom) },
            Operator::Add => format!("({})", self.join_operands_ascii(" + ")),
            Operator::Mul => format!("({})", self.join_operands_ascii(" * ")),
            Operator::Div => format!("({})", self.join_operands_ascii(" / ")),
            Operator::Sub => format!("({})", self.join_operands_ascii(" - ")),
            Operator::Pow => format!("({})", self.join_operands_ascii(" ^ ")),
            Operator::Name(name) => name.clone(),
            Operator::Func(ident) => format!("{}({})", ident.get_name(), self.join_operands_ascii(", ")),
            Operator::Const(cst) => cst.get_name()
        }
    }
}

impl Add for Expr {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Expr::new(Operator::Add, Some(vec![self, other]))
    }
}

impl Sub for Expr {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Expr::new(Operator::Sub, Some(vec![self, other]))
    }
}

impl Mul for Expr {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Expr::new(Operator::Mul, Some(vec![self, other]))
    }
}

impl Div for Expr {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        Expr::new(Operator::Div, Some(vec![self, other]))
    }
}