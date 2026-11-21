#[test]
fn test_subscript_sampling() {
    let result = grapher_engine::sampling::sample(
        vec![
            "a_2 = 3".to_string(),
            "y = a_2 * x".to_string(),
        ],
        50.0,
        400.0,
        300.0,
        800,
        600,
    );
    assert_eq!(result.len(), 2);
    assert!(result[0].is_empty(), "assignment line yields no path");
    assert!(!result[1].is_empty(), "subscripted variable expression yielded no path points");
}
