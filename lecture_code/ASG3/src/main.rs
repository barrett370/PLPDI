use std::collections::HashMap;
use std::ops::{Add, Mul};

#[derive(Clone)]
enum OP {
    Plus,
    Mul,
    Let,
}

#[derive(Clone)]
enum Expr<T>
where
    T: Copy,
{
    Value(T),
    Var(char),
    Unary(OP, Box<Expr<T>>),
    Binary(OP, Box<Expr<T>>, Box<Expr<T>>),
    Ternary(OP, Box<Expr<T>>, Box<Expr<T>>, Box<Expr<T>>),
}

impl<T> Expr<T>
where
    T: Copy,
{
    fn plus(e1: Expr<T>, e2: Expr<T>) -> Expr<T> {
        Expr::Binary(OP::Plus, Box::new(e1), Box::new(e2))
    }

    fn mul(e1: Expr<T>, e2: Expr<T>) -> Expr<T> {
        Expr::Binary(OP::Mul, Box::new(e1), Box::new(e2))
    }

    fn letexpr(v1: Expr<T>, e1: Expr<T>, e2: Expr<T>) -> Expr<T> {
        Expr::Ternary(OP::Let, Box::new(v1), Box::new(e1), Box::new(e2))
    }

    fn eval(self, ctx: &Option<HashMap<char, T>>) -> Result<T, &'static str>
    where
        T: Add<Output = T> + Mul<Output = T> + Copy,
    {
        match self {
            Expr::Var(c) => Ok(*(ctx.as_ref().unwrap().get(&c).unwrap())),
            Expr::Value(v) => Ok(v),
            Expr::Unary(op, v) => match op {
                _ => Err("Not a valid Unary operator"),
            },
            Expr::Binary(op, e1, e2) => match op {
                OP::Plus => Ok(e1.eval(ctx)? + e2.eval(ctx)?),
                OP::Mul => Ok(e1.eval(ctx)? * e2.eval(ctx)?),
                _ => Err("Not a valid binary operator"),
            },
            Expr::Ternary(op, v1, e1, e2) => match op {
                OP::Let => {
                    let mut new_ctx: HashMap<char, T> = HashMap::new();
                    match *v1 {
                        Expr::Var(c) => new_ctx.insert(c, e1.eval(ctx)?),
                        _ => {
                            return Err("First expression in a let must be of type Expr::Var(char)")
                        }
                    };
                    match ctx {
                        None => e2.eval(&Some(new_ctx)),
                        Some(ctx) => {
                            new_ctx.extend(ctx);
                            e2.eval(&Some(new_ctx))
                        }
                    }
                }
                _ => Err("Not a valid ternary operator"),
            },
        }
    }
}

fn main() {
//    let res = Expr::plus(Expr::Value(2), Expr::Value(3)).eval(&None);
//    println!("{:?}", res);
//
//    //let x = 1 + 2 in x + x
//    let res = Expr::letexpr(
//        Expr::Var('x'),
//        Expr::plus(Expr::Value(1), Expr::Value(2)),
//        Expr::plus(Expr::Var('x'), Expr::Var('x')),
//    )
//    .eval(&None);
//    println!("{:?}", res);
        for _ in 0..100000000 {
            Expr::letexpr(
            Expr::Var('x'),
            Expr::plus(Expr::Value(1), Expr::Value(2)),
            Expr::plus(Expr::Var('x'), Expr::Var('x')),
        )
        .eval(&None);
            }

//    // ASG
//
//    let a1 = Expr::plus(Expr::Value(1.0), Expr::Value(2.0));
//    let asg = Expr::plus(a1.clone(), a1).eval(&None);
//    println!("{:?}", asg);
//    //    for _ in 0..100000000{
//    //       Expr::plus(a1.clone(),a1.clone()).eval(&None);
//    //    }
//
//    // mul test
//
//    let res = Expr::mul(Expr::Value(2), Expr::Value(3)).eval(&None);
//    println!("{:?}", res);
//
//    // panic test
//    let res = Expr::letexpr(
//        Expr::Value(0),
//        Expr::plus(Expr::Value(1), Expr::Value(2)),
//        Expr::plus(Expr::Var('x'), Expr::Var('x')),
//    )
//    .eval(&None);
//    println!("{:?}", res);
}
