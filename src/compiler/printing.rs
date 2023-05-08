use pest::iterators::Pair;
use crate::HashMapVar;
use crate::formatting::format_string;
use crate::Rule;

pub fn print(mut line:Pair<Rule>,variables:&mut HashMapVar) {
    let mut rules = line.into_inner();
    let print_type = rules.next().unwrap().as_str().trim();
    let string = rules.next().unwrap().as_str();
    match print_type {
        "print" => {
            print!("{}",string)
        },
        "println" => {
            println!("{}",string)
        },
        "printf" => {
            print!("{}",format_string(&string[0..string.len()-1], variables));
        },
        "printfln" => {
            println!("{}",format_string(&string[0..string.len()-1], variables));
        }
        _ => unreachable!()

    }

}
