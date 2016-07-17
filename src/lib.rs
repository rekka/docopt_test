extern crate rustc_serialize;

#[derive(Debug, RustcDecodable)]
pub struct Args {
    pub flag_n: Option<usize>,
    pub arg_test: Option<String>,
}
