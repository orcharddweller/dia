use std::path::PathBuf;

pub fn read_test_file(path: &str) -> String {
    let mut path_buf = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

    path_buf.push("resources/test");
    path_buf.push(path);

    std::fs::read_to_string(&path_buf)
        .unwrap_or_else(|_| panic!("Failed to read test file: {path_buf:?}"))
}
