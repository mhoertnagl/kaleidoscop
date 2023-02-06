use std::collections::HashMap;

use inkwell::values::AnyValueEnum;

pub struct SymTable {
    scopes: Vec<Scope>,
}

impl SymTable {
    pub fn new() -> Self {
        SymTable { scopes: vec![] }
        // let mut table = SymTable {
        //     sp: 0,
        //     scopes: vec![],
        // };
        // table.push_scope();
        // table
    }

    pub fn push_scope(&mut self) {
        self.scopes.push(Scope::new());
        // self.scopes.push(cur.new_sub_scope());
    }

    pub fn pop_scope(&mut self) {
        self.scopes.pop();
    }

    pub fn insert(&mut self, sym: Sym) {
        self.scopes.last_mut().unwrap().insert(sym);
    }

    pub fn lookup(&mut self, name: &str) -> Option<&Sym> {
        self.scopes
            .iter()
            .rev()
            .fold(None, |acc, s| acc.or(s.lookup(name)))

        // for idx in (0..self.scopes.len()).rev() {
        //     let sym = self.scopes.get(idx).and_then(|s| s.lookup(name));
        //     if sym.is_some() {
        //         return sym;
        //     }
        // }
        // None

        // let mut idx = self.scopes.len();
        // let mut res = Option::<&Sym>::None;
        // while idx > 0 {
        //     if let Some(x) = self.scopes.get(idx) {
        //         res = x.lookup(name);
        //         idx = x.parent;
        //     }
        // }
        // res
    }

    // fn get_current_scope_mut(&mut self) -> &mut Scope {
    //     self.scopes.last_mut().unwrap()
    // }

    // fn get_current_scope_ref(&mut self) -> &Scope {
    //     self.scopes.last().unwrap()
    // }

    // fn get_current_scope(&mut self) -> Scope {
    //     self.scopes.pop().unwrap()
    // }
}

#[derive(Clone, PartialEq, Debug)]
pub struct Scope {
    symbols: HashMap<String, Sym>,
}

impl Scope {
    pub fn new() -> Self {
        Scope {
            symbols: HashMap::new(),
        }
    }

    pub fn insert(&mut self, sym: Sym) -> &mut Self {
        self.symbols.insert(sym.name.to_string(), sym);
        self
    }

    pub fn lookup(&self, name: &str) -> Option<&Sym> {
        self.symbols.get(name)
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct Sym {
    pub name: String,
    // TODO: Going to be a AnyValue later on.
    pub val: AnyValueEnum<'static>,
}

impl Sym {
    pub fn new(name: &str, val: AnyValueEnum<'static>) -> Self {
        Sym {
            name: name.to_string(),
            val,
        }
    }
}

#[cfg(test)]
mod tests {
    use inkwell::{context::Context, values::AnyValue};

    use super::*;

    #[test]
    fn create_new_symtable() {
        let context = Context::create();
        let i64_type = context.i64_type();
        let zero = i64_type.const_zero().as_any_value_enum();

        let mut st = SymTable::new();
        st.push_scope();
        st.insert(Sym::new("x", zero));

        assert_eq!(st.lookup("x").unwrap().val, zero);
    }

    #[test]
    fn insert() {
        let context = Context::create();
        let i64_type = context.i64_type();
        let zero = i64_type.const_zero().as_any_value_enum();
        let one = i64_type.const_int(1, false).as_any_value_enum();

        let mut st = SymTable::new();
        st.push_scope();
        st.insert(Sym::new("x", zero));
        st.push_scope();
        st.insert(Sym::new("y", one));

        assert_eq!(st.lookup("x").unwrap().val, zero);
        assert_eq!(st.lookup("y").unwrap().val, one);
    }
}
