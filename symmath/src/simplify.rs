use crate::expr::*;
use crate::num;

pub struct SimplifcationOpts {
    pub expand: bool,
    pub target_integers: bool
}

impl SimplifcationOpts {
    pub fn default() -> SimplifcationOpts {
        SimplifcationOpts {
            target_integers: false,
            expand: false
        }
    }

    pub fn expand() -> SimplifcationOpts {
        SimplifcationOpts {
            target_integers: false,
            expand: true
        }
    }

    pub fn integers() -> SimplifcationOpts {
        SimplifcationOpts {
            expand: false,
            target_integers: true
        }
    }
}

impl Expr {
    pub fn simplify(&mut self, opts: &SimplifcationOpts) {
        let mut a = true;
        let mut b = false;
        while a || b {
            a = self.simplify_impl(opts);
            b = self.flatten_impl(opts);
            // println!("{}", self.to_ascii());
        }
    }

    fn mul_frac(mut self, frac: num::Frac, _opts: &SimplifcationOpts) -> (Self, bool) {
        match self.operator {
            Operator::Add => {
                let mut new_elements = Vec::new();

                for child in self.elements.expect("Add has no children").into_iter() {
                    new_elements.push(child * Expr::frac_cpy(&frac));
                }

                self.elements = Some(new_elements);
                (self, true)
            },
            _ => (self * Expr::frac_cpy(&frac), false)
        }
    }

    fn expand_mul(self, other: Expr, opts: &SimplifcationOpts) -> (Self, bool) {
        match other.operator {
            Operator::Frac(f) => self.mul_frac(f, opts),
            _ => (self * other, false)
        }
    }

    fn simplify_impl(&mut self, opts: &SimplifcationOpts) -> bool {
        match &mut self.operator {
            Operator::Frac(ref mut frac) => {
                frac.simplify()
            },
            Operator::Add => {
                let mut changed = false;
                let mut fraccount = 0;
                let mut haszero = false;
                for ref mut child in self.elements.as_mut().expect("Add has no children").iter_mut() {
                    if child.simplify_impl(opts) { changed = true; }
                    match &child.operator {
                        Operator::Frac(f) => {
                            fraccount += 1;
                            haszero = haszero || f.is_zero();
                        },
                        _ => {}
                    }
                }
                if fraccount <= 1 && !haszero {
                    return changed;
                }

                let mut frac = num::Frac::new(0, 1);

                let mut new_elements = Vec::new();

                for child in self.elements.take().unwrap().into_iter() {
                    match child.operator {
                        Operator::Frac(f) => {
                            frac = frac + f;
                            changed = true;
                        },
                        _ => new_elements.push(child)
                    }
                }

                if !frac.is_zero() {
                    new_elements.push(Expr::new_empty(Operator::Frac(frac)));
                } else if new_elements.len() == 0 {
                    new_elements.push(Expr::int(0));
                }

                self.elements = Some(new_elements);
                
                return changed;
            },
            Operator::Mul => {
                let mut changed = false;
                let mut fraccount = 0;
                let mut iszero = false;
                for ref mut child in self.elements.as_mut().expect("Mul has no children").iter_mut() {
                    if child.simplify_impl(opts) { changed = true; }

                    match &child.operator {
                        Operator::Frac(f) => {
                            fraccount += 1;
                            iszero = f.is_zero();
                        },
                        _ => {}
                    }
                }
                
                if iszero {
                    self.operator = Operator::Frac(num::Frac::new(0, 1));
                    self.elements = None;
                    return true;
                }

                if fraccount >= 1 {
                    let mut frac = num::Frac::new(1, 1);

                    let mut new_elements = Vec::new();

                    for child in self.elements.take().unwrap().into_iter() {
                        match child.operator {
                            Operator::Frac(f) => {
                                frac = frac * f;
                                if fraccount != 1 { changed = true; }
                            },
                            _ => new_elements.push(child)
                        }
                    }

                    if opts.target_integers && frac.inv().is_int() && !frac.is_one() {
                        self.operator = Operator::Div;
                        self.elements = Some(vec![
                            Expr::new(Operator::Mul, Some(new_elements)),
                            Expr::frac_cpy(&frac.inv())
                        ]);
                        return true;
                    } else {
                        if !frac.is_one() {
                            new_elements.push(Expr::new_empty(Operator::Frac(frac)));
                        }

                        self.elements = Some(new_elements);
                    }
                }

                if opts.expand && self.elements.as_ref().unwrap().len() > 1 {
                    let mut elements = self.elements.take().unwrap().into_iter();
                    let mut new = elements.next().unwrap();

                    while let Some(next) = elements.next() {
                        let (new2, didchange) = new.expand_mul(next, opts);
                        if didchange {
                            changed = true;
                        }

                        new = new2;
                    }

                    self.operator = new.operator;
                    self.elements = new.elements;
                }

                return changed;
            },
            Operator::Div => {
                let mut changed = false;
                for ref mut child in self.elements.as_mut().expect("Div has no children").iter_mut() {
                    if child.simplify_impl(opts) { changed = true; }
                }
                
                if !opts.target_integers {
                    let mut replacement: Option<Expr> = None;
                    match &self.elements.as_ref().unwrap().get(1).expect("Div does not have a second child").operator {
                        Operator::Frac(f) => {
                            replacement = Some(Expr::new_empty(Operator::Frac(f.inv())));
                        },
                        _ => {}
                    }
                    
                    if let Some(e) = replacement {
                        self.operator = Operator::Mul;
                        self.elements.as_mut().unwrap()[1] = e;
                        changed = true;
                    }
                }
                
                return changed;
            },
            Operator::Sub => {
                let mut changed = false;
                for ref mut child in self.elements.as_mut().expect("Sub has no children").iter_mut() {
                    if child.simplify_impl(opts) { changed = true; }
                }
                return changed;
            },
            Operator::Pow => {
                let mut changed = false;
                for ref mut child in self.elements.as_mut().expect("Pow has no children").iter_mut() {
                    if child.simplify_impl(opts) { changed = true; }
                }
                return changed;
            },
            Operator::Name(_) => { false },
            Operator::Func(ref mut f) => {
                let mut changed = false;
                for ref mut child in self.elements.as_mut().expect("Func has no children").iter_mut() {
                    if child.simplify_impl(opts) { changed = true; }
                }
                
                let val = f.simplify_value(self.elements.take().unwrap());
                self.operator = val.operator;
                self.elements = val.elements;

                return changed;
            },
            Operator::Const(_) => { false }
        }
    }

    fn flatten_impl(&mut self, opts: &SimplifcationOpts) -> bool {
        match &mut self.operator {
            Operator::Frac(_) => { false },
            Operator::Add => {
                let mut changed = false;
                for ref mut child in self.elements.as_mut().expect("Add has no children").iter_mut() {
                    if child.flatten_impl(opts) { changed = true; }
                }

                let mut new_elements = Vec::new();

                for mut child in self.elements.take().unwrap().into_iter() {
                    match &child.operator {
                        Operator::Add => {
                            new_elements.append(child.elements.as_mut().expect("Add has no children"));
                            changed = true;
                        },
                        _ => new_elements.push(child)
                    }
                }

                if new_elements.len() == 1 {
                    let el = new_elements.into_iter().nth(0).unwrap();
                    self.operator = el.operator;
                    self.elements = el.elements;
                    changed = true;
                } else {
                    self.elements = Some(new_elements);
                }

                return changed;
            },
            Operator::Mul => {
                let mut changed = false;
                for ref mut child in self.elements.as_mut().expect("Mul has no children").iter_mut() {
                    if child.flatten_impl(opts) { changed = true; }
                }

                let mut new_elements = Vec::new();

                for mut child in self.elements.take().unwrap().into_iter() {
                    match &child.operator {
                        Operator::Mul => {
                            new_elements.append(child.elements.as_mut().expect("Mul has no children"));
                            changed = true;
                        },
                        _ => new_elements.push(child)
                    }
                }

                if new_elements.len() == 1 {
                    let el = new_elements.into_iter().nth(0).unwrap();
                    self.operator = el.operator;
                    self.elements = el.elements;
                    changed = true;
                } else {
                    self.elements = Some(new_elements);
                }

                return changed;
            },
            Operator::Div => {
                let mut changed = false;
                for ref mut child in self.elements.as_mut().expect("Div has no children").iter_mut() {
                    if child.flatten_impl(opts) { changed = true; }
                }
                return changed;
            },
            Operator::Sub => {
                let mut changed = false;
                for ref mut child in self.elements.as_mut().expect("Sub has no children").iter_mut() {
                    if child.flatten_impl(opts) { changed = true; }
                }
                self.operator = Operator::Add;

                let mut new_elements = Vec::new();

                for (idx, child) in self.elements.take().unwrap().into_iter().enumerate() {
                    if idx == 0 {
                        new_elements.push(child);
                    } else {
                        new_elements.push(child * Expr::int(-1));
                    }
                }

                self.elements = Some(new_elements);
                return changed;
            },
            Operator::Pow => {
                let mut changed = false;
                for ref mut child in self.elements.as_mut().expect("Pow has no children").iter_mut() {
                    if child.flatten_impl(opts) { changed = true; }
                }
                return changed;
            },
            Operator::Name(_) => { false },
            Operator::Func(_) => {
                let mut changed = false;
                for ref mut child in self.elements.as_mut().expect("Func has no children").iter_mut() {
                    if child.flatten_impl(opts) { changed = true; }
                }
                return changed;
            },
            Operator::Const(_) => { false }
        }
    }
}