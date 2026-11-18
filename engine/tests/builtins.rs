use grapher_engine::eval::eval_expr;

#[test]
fn test_builtins_trig_and_inverse() {
    assert_eq!(eval_expr("sin(0)", 0.0, 0.0), 0.0);
    assert_eq!(eval_expr("cos(0)", 0.0, 0.0), 1.0);
    assert_eq!(eval_expr("tan(0)", 0.0, 0.0), 0.0);

    assert_eq!(eval_expr("asin(0)", 0.0, 0.0), 0.0);
    assert!(eval_expr("asin(2)", 0.0, 0.0).is_nan());
    assert_eq!(eval_expr("acos(1)", 0.0, 0.0), 0.0);
    assert!(eval_expr("acos(2)", 0.0, 0.0).is_nan());
    assert_eq!(eval_expr("atan(0)", 0.0, 0.0), 0.0);
    assert_eq!(eval_expr("atan2(0, 1)", 0.0, 0.0), 0.0);
}

#[test]
fn test_builtins_hyperbolic() {
    assert_eq!(eval_expr("sinh(0)", 0.0, 0.0), 0.0);
    assert_eq!(eval_expr("cosh(0)", 0.0, 0.0), 1.0);
    assert_eq!(eval_expr("tanh(0)", 0.0, 0.0), 0.0);
    assert_eq!(eval_expr("asinh(0)", 0.0, 0.0), 0.0);
    assert_eq!(eval_expr("acosh(1)", 0.0, 0.0), 0.0);
    assert!(eval_expr("acosh(0)", 0.0, 0.0).is_nan());
    assert_eq!(eval_expr("atanh(0)", 0.0, 0.0), 0.0);
    assert!(eval_expr("atanh(1)", 0.0, 0.0).is_nan());
}

#[test]
fn test_builtins_log_exp() {
    assert_eq!(eval_expr("sqrt(4)", 0.0, 0.0), 2.0);
    assert!(eval_expr("sqrt(-1)", 0.0, 0.0).is_nan());
    assert_eq!(eval_expr("cbrt(8)", 0.0, 0.0), 2.0);
    assert_eq!(eval_expr("ln(1)", 0.0, 0.0), 0.0);
    assert!(eval_expr("ln(0)", 0.0, 0.0).is_nan());
    assert_eq!(eval_expr("log2(8)", 0.0, 0.0), 3.0);
    assert_eq!(eval_expr("log10(100)", 0.0, 0.0), 2.0);
    assert_eq!(eval_expr("log(8, 2)", 0.0, 0.0), 3.0);
    assert_eq!(eval_expr("exp(0)", 0.0, 0.0), 1.0);
}

#[test]
fn test_builtins_rounding_abs_sign() {
    assert_eq!(eval_expr("abs(-5)", 0.0, 0.0), 5.0);
    assert_eq!(eval_expr("sign(-5)", 0.0, 0.0), -1.0);
    assert_eq!(eval_expr("sign(5)", 0.0, 0.0), 1.0);
    assert_eq!(eval_expr("sign(0)", 0.0, 0.0), 0.0);
    assert_eq!(eval_expr("floor(2.7)", 0.0, 0.0), 2.0);
    assert_eq!(eval_expr("ceil(2.3)", 0.0, 0.0), 3.0);
    assert_eq!(eval_expr("round(2.5)", 0.0, 0.0), 3.0);
}

#[test]
fn test_builtins_min_max_mod_hypot() {
    assert_eq!(eval_expr("min(3, 5)", 0.0, 0.0), 3.0);
    assert_eq!(eval_expr("max(3, 5)", 0.0, 0.0), 5.0);
    assert_eq!(eval_expr("mod(5, 3)", 0.0, 0.0), 2.0);
    assert!(eval_expr("mod(5, 0)", 0.0, 0.0).is_nan());
    assert_eq!(eval_expr("hypot(3, 4)", 0.0, 0.0), 5.0);
}

#[test]
fn test_builtins_waveforms() {
    assert_eq!(eval_expr("heaviside(-1)", 0.0, 0.0), 0.0);
    assert_eq!(eval_expr("heaviside(1)", 0.0, 0.0), 1.0);
    assert_eq!(eval_expr("heaviside(0)", 0.0, 0.0), 0.5);

    assert_eq!(eval_expr("step(-1)", 0.0, 0.0), 0.0);
    assert_eq!(eval_expr("step(0)", 0.0, 0.0), 1.0);
    assert_eq!(eval_expr("step(1)", 0.0, 0.0), 1.0);

    assert_eq!(eval_expr("rect(0)", 0.0, 0.0), 1.0);
    assert_eq!(eval_expr("rect(0.5)", 0.0, 0.0), 1.0);
    assert_eq!(eval_expr("rect(0.6)", 0.0, 0.0), 0.0);

    assert_eq!(eval_expr("tri(0)", 0.0, 0.0), 1.0);
    assert_eq!(eval_expr("tri(0.5)", 0.0, 0.0), 0.5);
    assert_eq!(eval_expr("tri(1.5)", 0.0, 0.0), 0.0);

    assert_eq!(eval_expr("sinc(0)", 0.0, 0.0), 1.0);
    assert_eq!(eval_expr("imp(0)", 0.0, 0.0), 1.0);
    assert_eq!(eval_expr("imp(1)", 0.0, 0.0), 0.0);
}

#[test]
fn test_convolution() {
    use grapher_engine::eval::SymbolTable;
    use grapher_engine::parser::parse_expr;

    let mut st = SymbolTable::new();
    st.set_func('r', vec!['x'], parse_expr("rect(x)").unwrap());
    st.set_func('d', vec!['x'], parse_expr("imp(x)").unwrap());

    // (rect * rect)(0) should be ~1.0
    let val = parse_expr("conv(r, r, 0)").unwrap().eval_with_symbols(0.0, 0.0, &st);
    assert!((val - 1.0).abs() < 0.15, "Expected ~1.0, got {val}");

    // (imp * rect)(0.5) should be rect(0.5) = 1.0
    let val2 = parse_expr("dtconv(d, r, 0)").unwrap().eval_with_symbols(0.0, 0.0, &st);
    assert!((val2 - 1.0).abs() < 1e-2, "Expected ~1.0, got {val2}");
}

