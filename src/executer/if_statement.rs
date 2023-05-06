use pest::iterators::{Pair,Pairs};
use crate::HashMapVar;
use crate::executer::scope::Scope;
use crate::{
    types::VarType,
    Rule
};
pub trait IfStatement {
    fn if_statement(&mut self,line:Pair<Rule>);
}
impl <'a>IfStatement for crate::executer::Executer<'a>{
    fn if_statement(&mut self,line:Pair<Rule>) {
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
pub fn parse_to_bool(line:&mut Pairs<Rule>,variables:&mut HashMapVar) -> bool {
    let statement = line.next().unwrap().as_str().trim();
    println!("statement: {}",statement);
    if statement == "var" {
        println!("running var");
        let name = line.next().unwrap().as_str().trim();
        let opp = variables.get(name).unwrap().borrow();
        if let VarType::Bool(b) = *opp {
            println!("valid bool");
            drop(opp);
            return b;
        } else {panic!("Non boolean value provided")}
    } else {
        let statement = line.next().unwrap().as_str().trim();

        // okay to unwrap because if it panics the user put in a non-bool value
        return statement.parse::<bool>().unwrap();
    }

}
