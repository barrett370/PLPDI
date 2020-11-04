use super::*;   


#[test]
fn basic_add(){
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

    let e1 = Expr {
        tag: OP::ADD,
        data: None,
        operands: Some(vec![c1, c2]),
        name: None,
        vars: None,
    };
    let res = e1.eval();
    assert_eq!(3,res.data.unwrap());
}

#[test]
fn basic_mul(){

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
    assert_eq!(4,res.data.unwrap());

}
#[test]
fn let_expression(){
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
    
    let res = l1.eval();
    assert_eq!(6,res.data.unwrap());
}

#[test]
fn asg_implementation(){
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
    let res = a2.eval();
    assert_eq!(6,res.data.unwrap())
}
