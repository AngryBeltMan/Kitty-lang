use std::os::linux::raw::stat;

use crate::compiler::{
    cmp,
    scope::Scope
};
use crate::HashMapVar;
use crate::{types::VarType, Rule,};
use pest::iterators::{Pair, Pairs};
pub trait IfStatement {
    fn if_statement(&mut self, line: Pair<Rule>);
}
impl<'a> IfStatement for crate::compiler::Compiler<'a> {
    fn if_statement(&mut self, line: Pair<Rule>) {
        println!("encountered if statement");
        let mut inner_rules = line.into_inner();
        let res = parse_to_bool(&mut inner_rules, &mut self.variables);
        if res {
            let rules = Scope::parse_scope(inner_rules).rules;
            for rule in rules {
                self.parse_rule(rule);
            }
        }
    }
}
pub fn parse_to_bool(line: &mut Pairs<Rule>, variables: &mut HashMapVar) -> bool {
    parse_to_bool_with_name(line, variables).0
}
pub fn parse_to_bool_with_name(
    line: &mut Pairs<Rule>,
    variables: &mut HashMapVar,
) -> (bool, Option<String>) {
    let statement = line.next().unwrap().as_str().trim();
    println!("statement: {}", statement);
    if statement == "var" {
        // println!("running var");
        let name = line.next().unwrap().as_str().trim();
        let opp = variables.get(name).unwrap().borrow();
        if let VarType::Bool(b) = *opp {
            drop(opp);
            return (b, Some(name.to_string()));
        } else {
            panic!("Non boolean value provided")
        }
    } else {
        let statement = line.next().unwrap();
        if statement.as_str().contains("cmp") {
            let b = cmp::compare(statement, variables);
            return (b,None);
        } else {
            // okay to unwrap because if it panics the user put in a non-bool value
            return (statement.as_str().parse::<bool>().unwrap(), None);
        }
    }
}
