use swc_common::{sync::Lrc, SourceMap};
use swc_ecma_ast::Module;
use swc_ecma_codegen::{text_writer::JsWriter, Config, Emitter};

pub fn emit_ts_module(module: &Module) -> String {
    let cm: Lrc<SourceMap> = Default::default();
    let mut buf = vec![];
    let mut emitter = Emitter {
        cfg: Config::default(),
        cm: cm.clone(),
        comments: None,
        wr: JsWriter::new(cm.clone(), "\n", &mut buf, None),
    };

    emitter.emit_module(module).unwrap();

    String::from_utf8(buf).unwrap()
}
