extern crate rustc_serialize;

#[derive(Debug, RustcDecodable)]
pub struct Args {
    pub arg_test: Option<String>,
    pub flag_n: Option<usize>,
}
