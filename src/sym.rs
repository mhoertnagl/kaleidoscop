pub struct SymTable {
    val: i64,
    pub kids: Vec<SymTable>,
}

impl SymTable {
    pub fn new(val: i64) -> Self {
        SymTable {
            val: val,
            kids: vec![],
        }
    }

    pub fn insert(&mut self, kid: SymTable) -> &mut Self {
        self.kids.push(kid);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_new_symtable() {
        let st = SymTable::new(1);

        assert_eq!(st.val, 1);
    }

    #[test]
    fn insert() {
        let mut st = SymTable::new(1);
        st.insert(SymTable::new(2));
        st.insert(SymTable::new(3));

        assert_eq!(st.kids.len(), 2);
        assert_eq!(st.kids[0].val, 2);
        assert_eq!(st.kids[1].val, 3);
    }
}
