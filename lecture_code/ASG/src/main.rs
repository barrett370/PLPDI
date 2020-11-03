use std::collections::HashMap;
use std::fmt;
use std::fmt::Debug;
use std::hash::Hash;
use std::ops::{Add, Mul};

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
enum OP {
    INT,
    ADD,
    MUL,
    LET,
    VAR,
}

#[derive(Clone, Debug)]
struct Expr<T> {
    tag: OP,
    data: Option<T>,
    operands: Option<Vec<Expr<T>>>,
    vars: Option<HashMap<T, Expr<T>>>,
}

impl<T> fmt::Display for Expr<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.tag {
            OP::INT => write!(f, "Val:{}", self.data.as_ref().unwrap()),
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
            OP::VAR => write!(f, "{}", self.data.as_ref().unwrap()),
        }
    }
}

impl<T> Expr<T> {
    fn pass_context(mut self, ctx: &Option<HashMap<T, Expr<T>>>) -> Expr<T>
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
            OP::INT => return self,
            OP::ADD => {
                let ops = self.operands.as_ref().unwrap();
                let e0: Expr<T> = ops.get(0).unwrap().clone().pass_context(&self.vars).eval();
                let d0 = &e0.data;
                let e1: Expr<T> = ops.get(1).unwrap().clone().pass_context(&self.vars).eval();
                let d1 = &e1.data;
                let d2 = Some(d0.unwrap() + d1.unwrap());
                Expr {
                    tag: OP::INT,
                    data: d2,
                    operands: None,
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
                    tag: OP::INT,
                    data: d2,
                    operands: None,
                    vars: None,
                }
            }
            OP::LET => {
                let ops = self.operands.as_ref().unwrap();
                let var_name = ops.get(0).unwrap().clone().data.unwrap();
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
                        let mut v: HashMap<T, Expr<T>> = vars.clone();
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
                    .get(&self.data.unwrap())
                    .unwrap()
                    .clone();
                vars
            }
        }
    }
}

fn main() {
    let c1: Expr<i64> = Expr {
        tag: OP::INT,
        data: Some(1),
        operands: None,
        vars: None,
    };
    let c2: Expr<i64> = Expr {
        tag: OP::INT,
        data: Some(2),
        operands: None,
        vars: None,
    };

    let e1 = Expr {
        tag: OP::ADD,
        data: None,
        operands: Some(vec![c1, c2]),
        vars: None,
    };
    let res = e1.eval();
    println!("{}", res);

    let c2: Expr<i64> = Expr {
        tag: OP::INT,
        data: Some(2),
        operands: None,
        vars: None,
    };
    let c1: Expr<i64> = Expr {
        tag: OP::INT,
        data: Some(2),
        operands: None,
        vars: None,
    };

    let e2 = Expr {
        tag: OP::MUL,
        data: None,
        operands: Some(vec![c2, c1]),
        vars: None,
    };
    let res = e2.eval();
    println!("{}", res);

    // let x = 1 + 2 in x + x

    let c1: Expr<i64> = Expr {
        tag: OP::INT,
        data: Some(1),
        operands: None,
        vars: None,
    };
    let c2: Expr<i64> = Expr {
        tag: OP::INT,
        data: Some(2),
        operands: None,
        vars: None,
    };
    let v1 = Expr {
        tag: OP::VAR,
        data: Some(1),
        operands: None,
        vars: None,
    };

    let a1 = Expr {
        tag: OP::ADD,
        data: None,
        operands: Some(vec![c1, c2]),
        vars: None,
    };

    let a2 = Expr {
        tag: OP::ADD,
        data: None,
        operands: Some(vec![v1.clone(), v1.clone()]),
        vars: None,
    };

    let l1 = Expr {
        tag: OP::LET,
        data: None,
        operands: Some(vec![v1, a1, a2]),
        vars: None,
    };
    println!("{}",l1);
    let res = l1.eval();
    println!("{}", res);

    // ASG for let x = 1 + 2 in x + x

    let c1: Expr<i64> = Expr {
        tag: OP::INT,
        data: Some(1),
        operands: None,
        vars: None,
    };
    let c2: Expr<i64> = Expr {
        tag: OP::INT,
        data: Some(2),
        operands: None,
        vars: None,
    };

    let a1 = Expr {
        tag: OP::ADD,
        data: None,
        operands: Some(vec![c1, c2]),
        vars: None,
    };
    let a2 = Expr {
        tag: OP::ADD,
        data: None,
        operands: Some(vec![a1.clone(), a1]),
        vars: None,
    };
    println!("{}",a2);
    let res = a2.eval();

//    for _ in 1..100000000 {
//        a2.clone().eval();
//    }

    println!("{}", res);
}
