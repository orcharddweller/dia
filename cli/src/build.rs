use std::{
    fs::{create_dir_all, read_to_string, File},
    io::Write,
    path::PathBuf,
};

use dia_core::{compiler::emit::emit_ts_module, parser::parse, traits::Compile};
use glob::glob;

const DIST_PATH: &str = "dist";

fn find_dia() -> impl Iterator<Item = PathBuf> {
    glob("src/**/*.dia").unwrap().filter_map(Result::ok)
}

fn find_ts() -> impl Iterator<Item = PathBuf> {
    glob("src/**/*.ts").unwrap().filter_map(Result::ok)
}

fn transform_dia_path(path: &PathBuf) -> PathBuf {
    let mut new_path = PathBuf::from(DIST_PATH);
    new_path.push(path.strip_prefix("src").unwrap());
    new_path.set_extension("ts");
    new_path
}

fn transform_ts_path(path: &PathBuf) -> PathBuf {
    let mut new_path = PathBuf::from(DIST_PATH);
    new_path.push(path.strip_prefix("src").unwrap());
    new_path
}

pub fn build() {
    for path in find_dia() {
        let module = parse(read_to_string(&path).unwrap().as_str());

        let compiled = emit_ts_module(&module.to_ts_ast());

        let new_path = transform_dia_path(&path);

        create_dir_all(new_path.parent().unwrap()).unwrap();

        let mut file = File::create(new_path).unwrap();

        file.write_all(compiled.as_bytes()).unwrap();
    }

    for path in find_ts() {
        let new_path = transform_ts_path(&path);

        create_dir_all(new_path.parent().unwrap()).unwrap();

        std::fs::copy(&path, &new_path).unwrap();
    }
}
