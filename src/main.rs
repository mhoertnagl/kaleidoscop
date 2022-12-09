extern crate llvm_sys;

use codegen::Codegen;

// mod llvm;
mod ast;
mod codegen;
mod rlvm;
mod visitor;

// https://github.com/watchexec/cargo-watch
// https://github.com/Wilfred/bfc/blob/master/src/llvm.rs
// https://gitlab.com/taricorp/merthc/-/blob/master/src/main.rs

#[macro_use]
extern crate lalrpop_util;

fn main() {
    let prog = r"
        def main()
            let a = 1;
            let b = 5;
            let c = 7;
            let d = 0;
            if a then
                d = b + a;
            else
                d = c - a;
            end
            return d;
        end
    ";

    let parser = parser::ModuleParser::new();

    match parser.parse(prog) {
        Ok(module) => unsafe {
            println!("{}\n", module);
            let mut gen = Codegen::new();
            gen.emit(&module);
        },
        Err(err) => println!("{}", err),
    }
}

lalrpop_mod!(pub parser);

#[test]
fn parser() {
    let parser = parser::ModuleParser::new();
    let act = parser
        .parse(
            r"
                # Comment
                (1 + 2) * 4;
            ",
        )
        .unwrap();

    assert_eq!(&format!("{}", act), "((1 + 2) * 4);");
}
