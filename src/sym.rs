use std::borrow::Borrow;
use std::cell::{Cell, RefCell};
use std::collections::HashMap;
use std::rc::Rc;

use inkwell::types::PointerType;
use inkwell::values::{AnyValue, AnyValueEnum, PointerValue};

#[derive(Clone, PartialEq, Debug)]
pub struct Sym {
    pub name: String,
    // TODO: Going to be a AnyValue later on.
    pub val: AnyValueEnum<'static>,
}

impl Sym {
    pub fn new(name: &str, val: AnyValueEnum<'static>) -> Sym {
        Sym {
            name: name.to_string(),
            val,
        }
    }
}

pub struct SymTable {
    // parent: Option<Box<SymTable<'ctx>>>,
    // parent: Option<Rc<&'ctx SymTable<'ctx>>>,
    parent: Option<Box<SymTable>>,
    symbols: HashMap<String, Sym>,
}

impl SymTable {
    pub fn root() -> SymTable {
        SymTable::new(None)
    }

    pub fn new(parent: Option<Box<SymTable>>) -> SymTable {
        SymTable {
            parent: parent,
            symbols: HashMap::new(),
        }
    }

    pub fn new_subtable(self) -> Box<SymTable> {
        Box::from(SymTable::new(Some(Box::from(self))))
    }

    pub fn insert(&mut self, sym: Sym) {
        self.symbols.insert(sym.name.to_string(), sym);
    }

    pub fn get(&self, name: &str) -> Option<&Sym> {
        // self.symbols.borrow().get(name)

        self.symbols
            .get(name)
            .or_else(|| self.parent.as_ref().and_then(|p| p.get(name)))

        // match self.symbols.borrow().get(name) {
        //     Some(sym) => Some(sym),
        //     None => match self.parent {
        //         Some(p) => p.get(name),
        //         None => None,
        //     },
        // }
    }
}

use inkwell::context::Context;

#[test]
fn test_insert() {
    let context = Context::create();
    let i64_type = context.i64_type();
    let zero = i64_type.const_zero().as_any_value_enum();

    let mut root = SymTable::root();
    let sym = Sym::new("x", zero);
    let sym_clone = &sym.clone();

    assert_eq!(root.get("x"), None);
    assert_eq!(root.get("y"), None);

    root.insert(sym);

    assert_eq!(root.get("x"), Some(sym_clone));
    assert_eq!(root.get("y"), None);
}

#[test]
fn test_insert2() {
    let context = Context::create();
    let mut root = SymTable::root();

    assert_eq!(root.get("x"), None);
    assert_eq!(root.get("y"), None);
    assert_eq!(root.get("z"), None);

    root.insert(const_sym(&context, "x", 0));
    root.insert(const_sym(&context, "y", 1));

    assert_eq!(root.get("x"), Some(&const_sym(&context, "x", 0)));
    assert_eq!(root.get("y"), Some(&const_sym(&context, "y", 1)));
    assert_eq!(root.get("z"), None);
}

#[test]
fn test_subtable_insert() {
    let context = Context::create();
    let mut root = SymTable::root();
    let mut sub1 = root.new_subtable();

    assert_eq!(root.get("x"), None);
    assert_eq!(root.get("y"), None);
    assert_eq!(sub1.get("x"), None);
    assert_eq!(sub1.get("y"), None);

    root.insert(const_sym(&context, "x", 0));

    assert_eq!(root.get("x"), Some(&const_sym(&context, "x", 0)));
    assert_eq!(root.get("y"), None);
    assert_eq!(sub1.get("x"), None);
    assert_eq!(sub1.get("y"), None);

    sub1.insert(const_sym(&context, "y", 1));
}

fn const_sym(context: &'static Context, name: &str, value: u64) -> Sym {
    Sym::new(name, const_for(&context, value))
}

fn const_for(context: &'static Context, value: u64) -> AnyValueEnum<'static> {
    context
        .i64_type()
        .const_int(value, true)
        .as_any_value_enum()
}
