use pest::iterators::Pair;
use crate::{
    VarType,
    HashMapVar,
    Rule
};
pub fn compare(line:Pair<Rule>,variables: &mut HashMapVar) -> bool {
// cmp   = { "cmp" ~ "(" ~ args ~ ("$" ~ type)?  ~ name ~ opp ~ args ~("$" ~ type)?  ~ name ~ ")" }
    let mut inner_rule = line.into_inner();
    let left_rule = inner_rule.next().unwrap().as_str().trim();
    println!("left rule {}",left_rule);
    let name = inner_rule.next().unwrap().as_str();
    println!("name{}",name);
    let left_value = get_val(left_rule, name, variables);

    let opp = inner_rule.next().unwrap().as_str();

    let right_rule = inner_rule.next().unwrap().as_str().trim();
    println!("name type {}",name);
    let name = inner_rule.next().unwrap().as_str();
    let right_value = get_val(right_rule, name, variables);
    match opp {
        "==" => {
            return right_value == left_value;
        },
        "!=" => {
            return left_value != right_value;
        },
        "<=" => {
            let right_num = right_value.get_int().unwrap();
            let left_num  = left_value.get_int().unwrap();
            return  left_num <= right_num;
        },
        ">=" => {
            let right_num = right_value.get_int().unwrap();
            let left_num  = left_value.get_int().unwrap();
            return  left_num >= right_num;
        },
        ">" => {
            return left_value.get_int().unwrap() > right_value.get_int().unwrap();
        },
        "<" => {
            return left_value.get_int().unwrap() < right_value.get_int().unwrap();
        },
        _ => unreachable!(),
    }
}
pub fn get_val<'a>(vtype: &str, name: &str, variables: &mut HashMapVar) -> VarType {
    if vtype == "var" {
        println!("var");
        let var = variables.get(name).unwrap().borrow();
        return var.get_type().to_owned();
    } else {
        return VarType::new(vtype, name).unwrap();
    };
}
