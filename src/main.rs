extern crate docopt;
extern crate rustc_serialize;
extern crate docopt_test;

const USAGE: &'static str = "
Test

Usage:
  test [options] [<test>]

Options:
  -n=INT                Integer
";

#[derive(Debug, RustcDecodable)]
pub struct Args {
    arg_test: Option<String>,
    flag_n: Option<usize>,
}

fn main() {
    let args: docopt_test::Args = docopt::Docopt::new(USAGE)
        .and_then(|d| d.decode())
        .unwrap_or_else(|e| e.exit());

    println!("{:?}", args.flag_n);
}
