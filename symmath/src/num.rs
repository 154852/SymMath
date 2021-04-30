use std::ops::{Add, Sub, Mul, Div};

pub fn gcdi64(mut a: i64, mut b: i64) -> i64 {
    while b != 0 {
        let tmp = a % b;
        a = b;
        b = tmp;
    }
    return a;
}

pub fn lcmi64(a: i64, b: i64) -> i64 {
    return (a * b) / gcdi64(a, b);
}

// PartialEq: This assumes that both fractions are fully simplified
#[derive(PartialEq)]
pub struct Frac {
    pub num: i64,
    pub denom: i64
}

impl Frac {
    pub fn new(num: i64, denom: i64) -> Frac {
        Frac {
            num, denom
        }
    }

    pub fn gcd(&self) -> i64 {
        gcdi64(self.num, self.denom)
    }

    pub fn denom_lcm(&self, other: &Frac) -> i64 {
        lcmi64(self.denom, other.denom)
    }

    pub fn simplify(&mut self) -> bool {
        let g = self.gcd();
        if g == 1 { return false; }
        self.num /= g;
        self.denom /= g;
        return true;
    }

    pub fn is_zero(&self) -> bool {
        self.num == 0
    }

    pub fn is_one(&self) -> bool {
        self.num == self.denom
    }

    pub fn is_int(&self) -> bool {
        self.denom == 1
    }

    pub fn is_negative(&self) -> bool {
        self.denom * self.num < 0
    }
    
    pub fn inv(&self) -> Self {
        Frac {
            num: self.denom,
            denom: self.num
        }
    }
}

impl Add for Frac {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let denom = self.denom_lcm(&other);
        let mut f = Frac::new((self.num * (denom / self.denom)) + (other.num * (denom / other.denom)), denom);
        f.simplify();
        return f;
    }
}

impl Sub for Frac {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        let denom = self.denom_lcm(&other);
        let mut f = Frac::new((self.num * (denom / self.denom)) - (other.num * (denom / other.denom)), denom);
        f.simplify();
        return f;
    }
}

impl Mul for Frac {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        let mut f = Frac::new(self.num * other.num, self.denom * other.denom);
        f.simplify();
        return f;
    }
}

impl Div for Frac {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        let mut f = Frac::new(self.num * other.denom, self.denom * other.num);
        f.simplify();
        return f;
    }
}