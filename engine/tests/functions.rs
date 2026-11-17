use grapher_engine::eval::SymbolTable;
use grapher_engine::parser::parse_expr;

#[test]
fn test_user_defined_function() {
    let mut st = SymbolTable::new();
    st.set_func('f', vec!['x'], parse_expr("x * 2").unwrap());
    let call_node = parse_expr("f(3 + 2)").unwrap();
    assert_eq!(call_node.eval_with_symbols(0.0, 0.0, &st), 10.0);
}
