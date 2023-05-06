use pest::iterators::Pair;
use crate::{Rule,HashMapVar};

pub fn arithmetic(line:Pair<Rule>,variables:&mut HashMapVar) {
    let mut inner_rules = line.into_inner();
    println!("{inner_rules:?}");
    let name = inner_rules.next().unwrap().as_str();
    let arithmetic_type = inner_rules.next().unwrap().as_str();
    let amount = if let Some(rule) = inner_rules.next() {
        println!("num {}",rule.as_str());
        rule.as_str().trim().parse::<f64>().unwrap_or(rule.as_str().trim().parse::<i64>().unwrap() as f64)
    } else {1.0};
    let var = variables.get_mut(name).unwrap();
    match arithmetic_type {
        "++" => { var.borrow_mut().add(amount).unwrap() }
        "--" => { var.borrow_mut().subtract(amount).unwrap() }
        _ => panic!("unknown arithmetic type {arithmetic_type}")
    }
}
