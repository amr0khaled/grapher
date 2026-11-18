use grapher_engine::parser::{parse_expr, parse_assignment};

#[test]
fn test_syntax_errors() {
    assert!(parse_expr("1 + * 2").is_err());
    assert!(parse_expr("(1 + 2").is_err());
    assert!(parse_assignment("a = 1 + ").is_err());
    assert!(parse_assignment("invalid name! = 5").is_err());
}
