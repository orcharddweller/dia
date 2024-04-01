pub trait Codegen {
    fn generate(&self) -> String;
}

pub trait Compile<T> {
    fn to_ts_ast(&self) -> T;
}
