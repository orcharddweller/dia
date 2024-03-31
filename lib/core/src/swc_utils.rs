use swc_common::BytePos;
use swc_ecma_ast::Module;
use swc_ecma_parser::{lexer::Lexer, Parser, StringInput, Syntax, TsConfig};

pub fn parse_ts_module(ts_code: &str) -> Module {
    let lexer = Lexer::new(
        Syntax::Typescript(TsConfig::default()),
        Default::default(),
        StringInput::new(ts_code, BytePos(0), BytePos(0)),
        None,
    );

    let mut parser = Parser::new_from(lexer);

    parser.parse_module().expect("Failed to parse module")
}
