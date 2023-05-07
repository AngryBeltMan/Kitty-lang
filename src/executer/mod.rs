use pest::iterators::{Pairs, Pair};
use crate::executer::scope::Scope;
use std::{
    cell::{RefCell, Ref},
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
                println!("rules {}",rules);
                let res = if_statement::parse_to_bool_with_name(&mut rules.clone(), &mut self.variables);
                if res.0 {
                    println!("rules {}",rules);
                    println!("true");
                    let scope = rules.clone().skip(2)
                        // okay to unwrap because the parser won't parse if it did return None
                        .next().unwrap().into_inner();
                    let name = res.1;
                    if name.is_none() {
                        loop {
                            for rule in scope.clone() {
                                println!("rule {}",rule.as_str());
                                self.parse_rule(rule);
                            }
                        }
                    } else {
                        let name = Rc::clone(self.variables.get(&name.unwrap()).unwrap());
                        loop {
                            match *name.borrow() {
                                 VarType::Bool(b) => {
                                    if b {
                                        for rule in scope.clone() {
                                            println!("rule {}",rule.as_str());
                                            self.parse_rule(rule);
                                        }
                                    } else { break; }
                                },
                                _ => panic!("unsupported type")
                            }
                        }
                    }


                }
            },
            Rule::EOI => (),

            Rule::print => {
                let mut rules = line.into_inner();
                let print_type = rules.next().unwrap().as_str();
                let string = rules.next().unwrap().as_str();

                println!("printed: {string}");
            }
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
