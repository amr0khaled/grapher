use grapher_engine::eval::SymbolTable;
use grapher_engine::parser::parse_expr;

#[test]
fn test_dependent_evaluation() {
    let mut st = SymbolTable::new();
    st.set_expr('a', parse_expr("5").unwrap());
    st.set_expr('b', parse_expr("a + 2").unwrap());
    assert_eq!(st.eval_symbol('b', 0.0, 0.0), 7.0);
}

#[test]
fn test_cycle_detection() {
    let mut st = SymbolTable::new();
    st.set_expr('a', parse_expr("b").unwrap());
    st.set_expr('b', parse_expr("a").unwrap());
    assert!(st.eval_symbol('a', 0.0, 0.0).is_nan());
}
