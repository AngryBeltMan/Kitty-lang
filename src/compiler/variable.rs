use pest::iterators::Pair;
use std::{
    cell::RefCell,
    rc::Rc,
};
use crate::{
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
    let value = inner_rules.next().unwrap().as_str();
    println!("{var_type}");
    let value = if !vargs.is_empty() {
        let var = if vargs.contains("ref") {
            Rc::clone(variables.get(value).unwrap())
        } else {
            let inner_value = (**variables.get(value).unwrap()).clone();
            Rc::new(inner_value)
        };
        var
    } else {
        let var = VarType::new(var_type,value).unwrap();
        Rc::new(RefCell::new(var))
    };
    variables.insert(name.to_string(),value);
}
