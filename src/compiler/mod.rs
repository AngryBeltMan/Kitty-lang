use crate::{
    compiler::{if_statement::IfStatement, loops::WhileLoop},
    HashMapVar,
};
use crate::{formatting::format_string, types::VarType, Rule};
use pest::iterators::{Pair, Pairs};
use std::{
    cell::{Ref, RefCell},
    collections::HashMap,
    rc::Rc,
};

use self::loops::ForLoop;

pub mod arithmetic;
pub mod if_statement;
pub mod loops;
pub mod printing;
pub mod scope;
pub mod variable;
pub mod cmp;

pub struct Compiler<'a> {
    variables: HashMap<String, Rc<RefCell<VarType>>>,
    pub rules: Pairs<'a, Rule>,
    name: String,
}
impl<'a> Compiler<'a> {
    pub fn new<'b>(rules: Pairs<'b, Rule>) -> Self
    where
        'b: 'a,
    {
        Self {
            rules,
            variables: HashMap::new(),
            name: String::new(),
        }
    }
    pub fn execute(mut self) {
        for rule in self.rules.clone() {
            self.parse_rule(rule)
        }
        println!("{:#?}", self.variables);
    }
    pub fn parse_rule(&mut self, line: Pair<Rule>) {
        match line.as_rule() {
            Rule::variable => variable::variables(line, &mut self.variables),
            Rule::arithmetic => {
                arithmetic::arithmetic(line, &mut self.variables);
            }
            Rule::if_statement => {
                self.if_statement(line);
            }
            Rule::while_loop => {
                self.while_loop(line);
            }
            Rule::for_loop => self.for_loop(line),
            Rule::EOI => (),
            Rule::print => {
                printing::print(line, &mut self.variables);
            }
            _ => {
                panic!("{:?}", line.as_rule())
            }
        }
    }
}
