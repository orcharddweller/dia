use dia_core::parser::parse;

fn main() {
    let input = include_str!("../resources/t1.dia");

    let module = parse(input);

    println!("{:?}", module);
}
