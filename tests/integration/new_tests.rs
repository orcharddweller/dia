use std::env::set_current_dir;
use tempfile::tempdir;

#[test]
fn new_e2e() {
    let mut dir = tempdir().unwrap().into_path();

    set_current_dir(&dir).unwrap();

    let output = std::process::Command::new("dia")
        .arg("new")
        .arg("test")
        .output()
        .unwrap();

    assert!(output.status.success());

    dir.push("test");

    assert!(dir.exists());
}
