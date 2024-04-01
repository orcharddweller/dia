use crate::build::build;
use dotenv;
use std::path::Path;

const DEV_PATH: &str = ".dia/dev";

pub fn dev() {
    dotenv::dotenv().ok();
    build(Path::new(DEV_PATH))
}
