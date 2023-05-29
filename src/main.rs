#[macro_use]
extern crate lalrpop_util;

lalrpop_mod!(pub parser);

mod ast;

use std::fs;

use parser::UnitParser;

fn main() {
    let test = read_test();
    let parser = UnitParser::new();

    match parser.parse(test.as_str()) {
        Ok(ref module) => {
            println!("{:?}\n", module);
        }
        Err(err) => println!("{}", err),
    }
}

fn read_test() -> String {
    fs::read_to_string("examples/test.txt").unwrap()
}
