use std::ops::{Add, Sub, Mul, Div};
use crate::num;

#[derive(Clone)]
#[derive(PartialEq)]
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

    fn sine_simplify_value(&self, params: Vec<Expr>) -> Expr {
        let param = params.get(0).expect("Sin expects one parameter");

        // TODO: Don't do this at runtime
        // TODO: Find a way to get semi-simplification independent comparisons, for example for pi/6
        if *param == Expr::int(0) { Expr::int(0) }
        else {
            Expr::func(FuncIdentifier::Sine, params)
        }
    }

    pub fn simplify_value(&self, params: Vec<Expr>) -> Expr {
        match self {
            FuncIdentifier::Name(name) => Expr::func(FuncIdentifier::Name(name.clone()), params),
            FuncIdentifier::Sine => self.sine_simplify_value(params),
            FuncIdentifier::Cosine => Expr::func(FuncIdentifier::Cosine, params),
            FuncIdentifier::Tangent => Expr::func(FuncIdentifier::Tangent, params),
            FuncIdentifier::Abs => {
                let mut item = params.into_iter().next().unwrap();
                item.abs();
                item
            },
        }
    }
}

#[derive(PartialEq)]
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

#[derive(PartialEq)]
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

    pub fn func(func: FuncIdentifier, params: Vec<Expr>) -> Expr {
        Expr {
            operator: Operator::Func(func),
            elements: Some(params)
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

    pub fn abs(&mut self) {
        match &mut self.operator {
            Operator::Frac(ref mut frac) => {
                if frac.is_negative() {
                    frac.num *= -1;
                }
            },
            Operator::Add => {
                for ref mut child in self.elements.as_mut().expect("Add has no children").iter_mut() {
                    child.abs();
                }
            },
            Operator::Mul => {
                for ref mut child in self.elements.as_mut().expect("Mul has no children").iter_mut() {
                    child.abs();
                }
            },
            Operator::Div => {
                for ref mut child in self.elements.as_mut().expect("Div has no children").iter_mut() {
                    child.abs();
                }
            },
            Operator::Sub => {
                self.operator = Operator::Func(FuncIdentifier::Abs);
                self.elements = Some(vec![Expr::new(Operator::Sub, self.elements.take())]);
            },
            Operator::Pow => {
                self.operator = Operator::Func(FuncIdentifier::Abs);
                self.elements = Some(vec![Expr::new(Operator::Pow, self.elements.take())]);
            },
            Operator::Name(name) => {
                let newname = &name.clone();
                self.operator = Operator::Func(FuncIdentifier::Abs);
                self.elements = Some(vec![Expr::var(newname)]);
            },
            Operator::Func(ref mut f) => {
                let newf = f.clone();
                self.operator = Operator::Func(FuncIdentifier::Abs);
                self.elements = Some(vec![Expr::new(Operator::Func(newf), self.elements.take())]);
            },
            Operator::Const(c) => {
                match c {
                    _ => {}
                }
            }
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

// This is an elementwise comparison, not a mathematical one
impl PartialEq for Expr {
    fn eq(&self, other: &Self) -> bool {
        if self.operator != other.operator { return false; }
        if other.elements.is_none() != self.elements.is_none() { return false; }

        if let Some(els) = &self.elements {
            let otherels = other.elements.as_ref().unwrap();

            if otherels.len() != els.len() { return false }

            for i in 0..els.len() {
                if otherels[i] != els[i] {
                    return false;
                }
            }
        }

        return true;
    }
}