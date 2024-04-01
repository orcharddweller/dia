use std::{
    fs::{create_dir_all, read_to_string, remove_dir_all, File},
    io::Write,
    path::Path,
};

use dia_core::{compiler::emit::emit_ts_module, parser::parse, traits::Compile};

use crate::common::FileType;

pub fn build(output_dir: &Path) {
    let _ = remove_dir_all(output_dir);

    for path in FileType::Dia.walk_files() {
        let module = parse(read_to_string(&path).unwrap().as_str());

        let compiled = emit_ts_module(&module.to_ts_ast());

        let new_path = output_dir
            .join(path)
            .with_extension(FileType::Ts.extension());

        create_dir_all(new_path.parent().unwrap()).unwrap();

        let mut file = File::create(new_path).unwrap();

        file.write_all(compiled.as_bytes()).unwrap();
    }

    for path in FileType::Ts.walk_files() {
        let new_path = output_dir.join(&path);

        create_dir_all(new_path.parent().unwrap()).unwrap();

        std::fs::copy(&path, &new_path).unwrap();
    }
}
