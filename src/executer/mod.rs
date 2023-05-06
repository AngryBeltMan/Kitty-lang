use pest::iterators::{Pairs, Pair};
use crate::executer::scope::Scope;
use std::{
    cell::RefCell,
    rc::Rc,
    collections::HashMap
};
use crate::{
    types::VarType,
    Rule
};

use self::if_statement::IfStatement;

pub mod scope;
pub mod arithmetic;
pub mod variable;
pub mod if_statement;


pub struct Executer<'a> {
    variables:HashMap<String,Rc<RefCell<VarType>>>,
    pub rules:Pairs<'a,Rule>,
    name:String,
}
impl <'a>Executer <'a>{
    pub fn new<'b>(rules:Pairs<'b,Rule>) -> Self
        where 'b:'a
    {
        Self {
            rules,
            variables:HashMap::new(),
            name:String::new()
        }
    }
    pub fn execute(mut self) {
        for rule in self.rules.clone() {
            self.parse_rule(rule)
        }
        println!("{:#?}",self.variables);
    }
    pub fn parse_rule(&mut self,line:Pair<Rule>) {
        match line.as_rule() {
            Rule::variable => { variable::variables(line, &mut self.variables) },
            Rule::arithmetic => { arithmetic::arithmetic(line, &mut self.variables); },
            Rule::if_statement => { self.if_statement(line); },
            Rule::while_loop => {
                let mut rules = line.into_inner();
                while if_statement::parse_to_bool(&mut rules, &mut self.variables) {

                }
            },
            Rule::EOI => (),

            Rule::print => {
                let mut rules = line.into_inner();
                let string = rules.next().unwrap().as_str();
                println!("{string}");
            },
            Rule::name => {
                let mut inner_rules = line.into_inner();
                self.name = inner_rules.next().unwrap().as_str().to_string();
            }
            _ => {
                panic!("{:?}",line.as_rule())
            },
        }
    }
}
