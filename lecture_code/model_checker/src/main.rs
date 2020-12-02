use model_checker::{State,LTS,Expr};
use std::collections::{HashMap,HashSet};

fn main() {
    
    // Test LTS
    let s1 = State{name:String::from("S1"),terminal:None};
    let s2 = State{name:String::from("S2"),terminal:None};

    let mut states = HashSet::new();
    states.insert(s1.clone());
    states.insert(s2.clone());

    let mut transitions = HashMap::new();
    let mut labels = HashSet::new();
    labels.insert(String::from("a"));
    transitions.insert((s1,s2),labels);

    let lts = LTS{states,transitions};

    println!("{:?}",lts);


    // Test expr
    
    let e = Expr::Bool(true);
    println!("{:?}",e.negate().unwrap());

}
