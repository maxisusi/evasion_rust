mod tests {
    use core::panic;
    use std::collections::HashMap;

    use crate::symbol_table::{Scope, Symbol, SymbolTable};

    #[test]
    fn test_define() {
        let expected: HashMap<String, Symbol> = HashMap::from([
            ("a".to_string(), Symbol::new("a", Scope::GlobalScope, 0)),
            ("b".to_string(), Symbol::new("b", Scope::GlobalScope, 1)),
        ]);

        let mut symbol_table = SymbolTable::new();

        let a_symbol = symbol_table.define("a");
        let b_symbol = symbol_table.define("b");

        assert_eq!(&a_symbol, expected.get("a").unwrap());
        assert_eq!(&b_symbol, expected.get("b").unwrap());
    }

    #[test]
    fn test_resolve() {
        let mut symbol_table = SymbolTable::new();

        symbol_table.define("a");
        symbol_table.define("b");

        let expected: HashMap<String, Symbol> = HashMap::from([
            ("a".to_string(), Symbol::new("a", Scope::GlobalScope, 0)),
            ("b".to_string(), Symbol::new("b", Scope::GlobalScope, 1)),
        ]);

        for (key, symbol) in expected {
            let resolved = symbol_table.resolve(key);

            if let Some(res_symbol) = resolved {
                assert_eq!(
                    res_symbol, &symbol,
                    "expected={}\ngot={}",
                    symbol, res_symbol
                );
            } else {
                panic!("Couldn't find symbol inside symbol table\nngot={}", symbol)
            }
        }
    }
}
