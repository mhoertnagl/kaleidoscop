use std::fs;

#[macro_use]
extern crate lalrpop_util;

lalrpop_mod!(pub parser);

mod ast;

fn main() {
    let test = read_test();
    let parser = parser::ModuleParser::new();
    match parser.parse(test.as_str()) {
        Ok(prog) => println!("{:?}\n", prog),
        Err(err) => println!("{}", err),
    }
}

fn read_test() -> String {
    fs::read_to_string("examples/test.txt").unwrap()
}
