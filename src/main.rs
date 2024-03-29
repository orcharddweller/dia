use dia_core::parser::{DiaParser, Rule};
use pest::Parser;

fn main() {
    let res = DiaParser::parse(Rule::expression, "{\"}\"}")
        .unwrap()
        .next()
        .unwrap();

    match res.as_rule() {
        Rule::expression => {
            println!("{:?}", res.as_span())
        }
        _ => {
            println!("Something else");
        }
    }
}
