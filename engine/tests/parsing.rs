#[test]
fn test_parse_assignment() {
    let res = grapher_engine::parser::parse_assignment("a = 3 + x");
    assert!(res.is_ok());
    let (name, node) = res.unwrap();
    assert_eq!(name, 'a');
    assert_eq!(node.eval(2.0, 0.0), 5.0);
}

#[test]
fn test_parse_invalid_assignment() {
    assert!(grapher_engine::parser::parse_assignment("ab = 3").is_err());
    assert!(grapher_engine::parser::parse_assignment(" = 3").is_err());
}

#[test]
fn test_parse_assignment_ignores_rhs_symbol_resolution() {
    // Parsing must succeed for "y = a * x" even without knowing 'a'
    let res = grapher_engine::parser::parse_assignment("y = a * x");
    assert!(res.is_ok());
    let (name, _node) = res.unwrap();
    assert_eq!(name, 'y');
}
