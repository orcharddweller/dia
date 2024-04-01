use glob::glob;
use std::path::PathBuf;

pub enum FileType {
    Dia,
    Ts,
}

impl FileType {
    pub fn extension(&self) -> &'static str {
        match self {
            FileType::Dia => "dia",
            FileType::Ts => "ts",
        }
    }

    pub fn walk_files(&self) -> impl Iterator<Item = PathBuf> {
        let mut path = PathBuf::from("src/**");
        path.push(format!("*.{}", self.extension()));
        glob(path.to_str().unwrap()).unwrap().filter_map(Result::ok)
    }
}
