use std::collections::{HashMap,HashSet};

#[derive(Debug)]
pub enum OP {
    Implication,
    And,
    Or,
    Until,
    Always,
    Eventually,
    Not,
}

#[derive(Debug)]
pub enum Expr {
    Unary(OP,Box<Expr>),
    Binary(OP,Box<Expr>,Box<Expr>),
    Var(char),
    Bool(bool),
}

#[derive(Hash,PartialEq,Eq,Clone,Debug)]
pub struct State{
    pub name: String,
    pub terminal: Option<bool>
}

#[derive(Debug)]
pub struct LTS {

    pub states: HashSet<State>,
    pub transitions: HashMap<(State,State),HashSet<String>>

}

type Letter = char;

struct NFA {

    states: HashSet<State>,
    transitions: HashMap<(State,State),Letter>

}

impl Expr{

pub fn eval(self) -> Result<Expr,String>{
    let unary_err = Err(String::from("Error, Operation cannot be unary"));
    let unimp_err = Err(String::from("Not Implemented"));
    match self {

        Expr::Unary(op, e) => {
            match op {
                OP::Implication => return unary_err,
                OP::And => return unary_err,
                OP::Or => return unary_err,
                OP::Until => return unary_err,
                OP::Always => return unimp_err,
                OP::Eventually => return unimp_err,
                OP::Not => {
                    match e {
                        Unary(op,
            }
        },
        Expr::Binary(op,e1,e2) => return unimp_err,
        Expr::Var(c) => return Ok(Expr::Unary(OP::Not,Box::new(Expr::Var(c)))),
        Expr::Bool(b) => return Ok(Expr::Bool(!b)),



}

Ok(Expr::Bool(true))
}
}
