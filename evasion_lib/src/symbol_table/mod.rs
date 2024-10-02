use std::{collections::HashMap, fmt::Display, usize};
mod symbol_table_tests;

#[derive(Debug, PartialEq, Clone)]
enum Scope {
    GlobalScope,
}

#[derive(PartialEq, Debug, Clone)]
struct Symbol {
    name: String,
    scope: Scope,
    index: usize,
}

impl Symbol {
    fn new<T>(name: T, scope: Scope, index: usize) -> Self
    where
        T: Into<String>,
    {
        Self {
            name: name.into(),
            scope,
            index,
        }
    }
}
impl Display for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

struct SymbolTable {
    store: HashMap<String, Symbol>,
    num_definition: usize,
}

impl SymbolTable {
    fn new() -> Self {
        Self {
            store: HashMap::new(),
            num_definition: 0,
        }
    }

    fn define<T>(&mut self, name: T) -> Symbol
    where
        T: Into<String>,
    {
        let name: String = name.into();

        let symbol = Symbol::new(name.clone(), Scope::GlobalScope, self.num_definition);
        self.store.insert(name.clone(), symbol.clone());
        self.num_definition += 1;
        symbol
    }
    fn resolve<T>(&self, symbol: T) -> Option<&Symbol>
    where
        T: Into<String>,
    {
        self.store.get(&symbol.into())
    }
}
