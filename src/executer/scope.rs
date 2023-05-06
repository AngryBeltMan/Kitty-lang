use pest::iterators::{Pairs, Pair};
use crate::Rule;
pub struct Scope <'a> {
    pub rules: Vec<Pair<'a, Rule>>
}
impl <'a> Scope <'a>{
    pub fn parse_scope(mut pairs: Pairs<'a,Rule>) -> Self {
        let mut rules = vec![];
        let mut pairs = pairs.next().unwrap().into_inner();
        while let Some(rule) = pairs.next() {
            println!("rule: {}",rule.as_str());
            rules.push(rule);
        }
        Self {rules}
    }
}
