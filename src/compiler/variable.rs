use pest::iterators::Pair;
use std::{
    cell::RefCell,
    rc::Rc,
};
use crate::{
    compiler::cmp,
    HashMapVar,
    types::VarType,
    Rule
};
pub fn variables(line:Pair<Rule>,variables:&mut HashMapVar) {
    let mut inner_rules = line.into_inner(); // { name ~ "=" ~ value }
    let name = inner_rules.next().unwrap().as_str();
    let var_type = inner_rules.next().unwrap().as_str();
    let vargs = inner_rules.next().unwrap().as_str();
    println!("varg: {vargs}");
    let value = inner_rules.next().unwrap();
    println!("{var_type}");
    let value = if !vargs.is_empty() {
        println!("called hello");
        let var = if vargs.contains("ref") {
            Rc::clone(variables.get(value.as_str()).unwrap())
        } else {
            let inner_value = (**variables.get(value.as_str()).unwrap()).clone();
            Rc::new(inner_value)
        };
        var
    } else {
        if format!("{:?}",value).contains("cmp") {
            let out = cmp::compare(value, variables);
            Rc::new(RefCell::new(VarType::Bool(out)))
        } else {
            println!("called var");
            let var = VarType::new(var_type,value.as_str()).unwrap();
            Rc::new(RefCell::new(var))
        }
    };
    variables.insert(name.to_string(),value);
}
