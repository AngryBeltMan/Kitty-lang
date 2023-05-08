use crate::HashMapVar;
#[test]
fn formatting_test() {
    use crate::VarType;
    use std::{
        rc::Rc,
        cell::RefCell,
        collections::HashMap
    };
    let mut variables:HashMapVar = HashMap::new();
    variables.insert(String::from("hello"),Rc::new(RefCell::new(VarType::String("world".to_string()))));
    let string = format_string("hello %hello",&mut variables);
    let string2 = format!("hello {:?}",variables.get("hello").unwrap().borrow());
    assert_eq!(string,string2);
}
pub fn format_string(string:&str,variables:&mut HashMapVar) -> String {
    let c = string.split(" ");
    let mut out = String::new();
    for a in c {
        if a.contains("%") {
            let name = a.replace("%", "");
            println!("{name}");
            let val = variables.get(&name).unwrap();
            out.push_str(format!("{:?} ",val.borrow()).as_str());
        } else {
            out.push_str(a);
            out.push_str(" ");
        }
    }
    out[0..out.len()-1].to_string()
}
