use crate::build::build;
use std::path::Path;

const DEV_PATH: &str = ".dia/dev";

pub fn dev() {
    build(Path::new(DEV_PATH))
}
