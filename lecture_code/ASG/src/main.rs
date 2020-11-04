use std::collections::HashMap;
use std::fmt;
use std::fmt::Debug;
use std::hash::Hash;
use std::ops::{Add, Mul};

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
enum OP {
    VAL,
    ADD,
    MUL,
    LET,
    VAR,
}

#[derive(Clone, Debug)]
struct Expr<T> {
    tag: OP,
    data: Option<T>,
    name: Option<char>,
    operands: Option<Vec<Expr<T>>>,
    vars: Option<HashMap<char, Expr<T>>>,
}

impl<T> fmt::Display for Expr<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.tag {
            OP::VAL => write!(f, "{}", self.data.as_ref().unwrap()),
            OP::ADD => write!(
                f,
                "{} + {}",
                self.operands.as_ref().unwrap().get(0).unwrap(),
                self.operands.as_ref().unwrap().get(1).unwrap()
            ),
            OP::MUL => write!(
                f,
                "{} * {}",
                self.operands.as_ref().unwrap().get(0).unwrap(),
                self.operands.as_ref().unwrap().get(1).unwrap()
            ),
            OP::LET => write!(
                f,
                "let {} = {} in {}",
                self.operands.as_ref().unwrap().get(0).unwrap(),
                self.operands.as_ref().unwrap().get(1).unwrap(),
                self.operands.as_ref().unwrap().get(2).unwrap()
            ),
            OP::VAR => write!(f, "{}", self.name.as_ref().unwrap()),
        }
    }
}

impl<T> Expr<T> {
    fn pass_context(mut self, ctx: &Option<HashMap<char, Expr<T>>>) -> Expr<T>
    where
        T: Copy + Clone,
    {
        match ctx {
            Some(vars) => {
                self.vars = Some(vars.clone());
                self
            }
            None => self,
        }
    }

    fn eval(self) -> Expr<T>
    where
        T: Add<Output = T> + Copy + Clone + Mul<Output = T> + PartialEq + Eq + Hash + Debug,
    {
        match self.tag {
            OP::VAL => return self,
            OP::ADD => {
                let ops = self.operands.as_ref().unwrap();
                let e0: Expr<T> = ops.get(0).unwrap().clone().pass_context(&self.vars).eval();
                let d0 = &e0.data;
                let e1: Expr<T> = ops.get(1).unwrap().clone().pass_context(&self.vars).eval();
                let d1 = &e1.data;
                let d2 = Some(d0.unwrap() + d1.unwrap());
                Expr {
                    tag: OP::VAL,
                    data: d2,
                    operands: None,
                    name: None,
                    vars: None,
                }
            }
            OP::MUL => {
                let ops = self.operands.as_ref().unwrap();
                let e0: Expr<T> = ops.get(0).unwrap().clone().pass_context(&self.vars).eval();
                let d0 = &e0.data;
                let e1: Expr<T> = ops.get(1).unwrap().clone().pass_context(&self.vars).eval();
                let d1 = &e1.data;
                let d2 = Some(d0.unwrap() * d1.unwrap());
                Expr {
                    tag: OP::VAL,
                    data: d2,
                    operands: None,
                    name: None,
                    vars: None,
                }
            }
            OP::LET => {
                let ops = self.operands.as_ref().unwrap();
                let var_name = ops.get(0).unwrap().name.unwrap();
                let var_val = ops.get(1).unwrap().clone().pass_context(&self.vars).eval();
                let mut body = ops.get(2).unwrap().clone();
                match body.vars {
                    None => {
                        let mut hm = HashMap::new();
                        hm.insert(var_name, var_val);
                        body.vars = Some(hm);
                        return body.eval();
                    }
                    Some(vars) => {
                        let mut v: HashMap<char, Expr<T>> = vars.clone();
                        v.insert(var_name, var_val);
                        body.vars = Some(v);
                        return body.eval();
                    }
                }
            }
            OP::VAR => {
                let vars = self
                    .vars
                    .clone()
                    .unwrap()
                    .get(&self.name.unwrap())
                    .unwrap()
                    .clone();
                vars
            }
        }
    }
}

fn main() {
    
    // let x = 1 + 2 in x + x

    let c1: Expr<i64> = Expr {
        tag: OP::VAL,
        data: Some(1),
        operands: None,
        name: None,
        vars: None,
    };
    let c2: Expr<i64> = Expr {
        tag: OP::VAL,
        data: Some(2),
        operands: None,
        name: None,
        vars: None,
    };
    let v1 = Expr {
        tag: OP::VAR,
        data: None,
        operands: None,
        name: Some('x'),
        vars: None,
    };

    let a1 = Expr {
        tag: OP::ADD,
        data: None,
        operands: Some(vec![c1, c2]),
        name: None,
        vars: None,
    };

    let a2 = Expr {
        tag: OP::ADD,
        data: None,
        operands: Some(vec![v1.clone(), v1.clone()]),
        name: None,
        vars: None,
    };

    let l1 = Expr {
        tag: OP::LET,
        data: None,
        operands: Some(vec![v1, a1, a2]),
        name: None,
        vars: None,
    };
    
    let res = l1.clone().eval();
    println!("eval({}) = {}",l1,res);
    //ASG 
    let c1: Expr<i64> = Expr {
        tag: OP::VAL,
        data: Some(1),
        operands: None,
        name: None,
        vars: None,
    };
    let c2: Expr<i64> = Expr {
        tag: OP::VAL,
        data: Some(2),
        operands: None,
        name: None,
        vars: None,
    };

    let a1 = Expr {
        tag: OP::ADD,
        data: None,
        operands: Some(vec![c1, c2]),
        name: None,
        vars: None,
    };
    let a2 = Expr {
        tag: OP::ADD,
        data: None,
        operands: Some(vec![a1.clone(), a1]),
        name: None,
        vars: None,
    };
    let res = a2.clone().eval();
    println!("eval({}) = {}",a2,res);

    //basic mul
    let c1: Expr<i64> = Expr {
        tag: OP::VAL,
        data: Some(2),
        operands: None,
        name: None,
        vars: None,
    };
    let c2: Expr<i64> = Expr {
        tag: OP::VAL,
        data: Some(2),
        operands: None,
        name: None,
        vars: None,
    };

    let e2 = Expr {
        tag: OP::MUL,
        data: None,
        operands: Some(vec![c2, c1]),
        name: None,
        vars: None,
    };
    let res = e2.eval();
    println!("{}", res);
}


#[cfg(test)]
mod tests;

