#[macro_use]
extern crate lalrpop_util;

lalrpop_mod!(pub parser);

mod ast;
mod ext;
mod gen;

use std::fs;

use inkwell::context::Context;

use gen::Gen;
use parser::UnitParser;

fn main() {
    let test = read_test();
    let parser = UnitParser::new();
    let context = Context::create();
    let builder = context.create_builder();
    let module = context.create_module("main");
    let mut gen = Gen::new(builder, &context, module);

    match parser.parse(test.as_str()) {
        Ok(ref module) => {
            println!("{:?}\n", module);
            gen.compile(module);
        }
        Err(err) => println!("{}", err),
    }
}

fn read_test() -> String {
    fs::read_to_string("examples/test.txt").unwrap()
}
