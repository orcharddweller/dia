use dia_core::{
    compiler::emit::emit_ts_module,
    parser::parse,
    traits::{Codegen, Compile},
};

fn main() {
    let input = include_str!("../resources/t1.dia");

    let module = parse(input);

    println!("{:?}", module);

    println!("{:?}", module.code().to_ts_ast());

    println!("{}", module.generate());

    println!("{}", emit_ts_module(&module.code().to_ts_ast()))
}
