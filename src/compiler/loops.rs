use crate::compiler::Compiler;
use crate::compiler::if_statement;
use std::rc::Rc;
use std::cell::RefCell;
use crate::Rule;
use crate::VarType;
use pest::iterators::Pair;
pub trait WhileLoop {
    fn while_loop(&mut self,line:Pair<Rule>);
}
pub trait ForLoop {
    fn for_loop(&mut self,line:Pair<Rule>);
}
impl <'a> WhileLoop for Compiler<'a> {
    fn while_loop(&mut self,line:Pair<Rule>) {
        let rules = line.into_inner();
        let res = if_statement::parse_to_bool_with_name(&mut rules.clone(), &mut self.variables);
        if res.0 {
            let scope = rules.skip(2)
                // okay to unwrap because the parser won't parse if it did return None
                .next().unwrap().into_inner();
            let name = res.1;
            if name.is_none() {
                loop {
                    for rule in scope.clone() {
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
    }
}
impl <'a> ForLoop for Compiler<'a> {
    fn for_loop(&mut self,line:Pair<Rule>) {
        let mut inner_rules = line.into_inner();
        let var = inner_rules.next().unwrap().as_str();
        println!("{var}");
        let mut range = inner_rules.next().unwrap().into_inner();
        let start = range.next().unwrap().as_str().trim().parse::<i32>().expect("expected a number");
        let arrow = range.next().unwrap().as_str();
        let end = range.next().unwrap().as_str().trim().parse::<i32>().expect("expected a number");
        let scope = inner_rules.next().unwrap().into_inner();
        println!("scope {scope}");
        if arrow.trim() == "->" {
            if self.variables.insert(var.to_string(), Rc::new(RefCell::new(VarType::Int(start)))).is_some() {}
            for _ in start..end {
                for r in scope.clone() {
                    self.parse_rule(r);
                }
                self.variables.get(var).unwrap().borrow_mut().add(1.0).unwrap();
            }
        } else {
            if self.variables.insert(var.to_string(), Rc::new(RefCell::new(VarType::Int(start)))).is_some() {}
            for _ in end..start {
                for r in scope.clone() {
                    self.parse_rule(r);
                }
                self.variables.get(var).unwrap().borrow_mut().subtract(1.0).unwrap();
            }
        }
    }
}
