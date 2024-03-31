use crate::{ast::TsModule, swc_utils::parse_ts_module, traits::Compile};

impl Compile<swc_ecma_ast::Module> for TsModule {
    fn to_ts_ast(&self) -> swc_ecma_ast::Module {
        parse_ts_module(self.raw())
    }
}
