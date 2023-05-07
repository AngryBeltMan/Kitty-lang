#![feature(iter_next_chunk)]
extern crate pest;
extern crate pest_derive;

use pest::{Parser, iterators::{Pairs, Pair}};
use std::{
    cell::{RefCell,Cell},
    rc::Rc,
    collections::HashMap
};
use pest_derive::Parser;
use crate:: types::VarType ;

mod types;
mod compiler;
mod init;


const FILE:&'static str = include_str!("test.csv");

#[derive(Parser)]
#[grammar = "grammar/grammar.pest"]
pub struct KilnParser;
pub type HashMapVar = HashMap<String, Rc<RefCell<VarType>>>;

fn main() {
    let file = KilnParser::parse(Rule::file, FILE)
        .expect("could not parse").next().expect("never fails");
    let executer = compiler::Compiler::new(file.into_inner());
    executer.execute();
}
