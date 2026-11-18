use grapher_engine::eval::eval_expr;

#[test]
fn test_implicit_one_arg_fn() {
    assert_eq!(eval_expr("sin 0", 0.0, 0.0), 0.0);
    assert_eq!(eval_expr("sin x", std::f64::consts::FRAC_PI_2, 0.0), 1.0);
    assert_eq!(eval_expr("cos x + 1", 0.0, 0.0), 2.0); // cos(x)+1, not cos(x+1)
    assert_eq!(eval_expr("sin x^2", (std::f64::consts::FRAC_PI_2).sqrt(), 0.0), 1.0); // sin(x^2)
    assert_eq!(eval_expr("sqrt 9", 0.0, 0.0), 3.0);
    assert_eq!(eval_expr("exp 0", 0.0, 0.0), 1.0);
    assert_eq!(eval_expr("abs -5", 0.0, 0.0), 5.0); // abs binds unary minus

    // Parity: explicit parens vs implicit argument must match for all one-arg builtins
    for (explicit, implicit) in [
        ("sin(1.25)", "sin 1.25"),
        ("cos(1.25)", "cos 1.25"),
        ("tan(1.25)", "tan 1.25"),
        ("asin(0.5)", "asin 0.5"),
        ("acos(0.5)", "acos 0.5"),
        ("atan(1.25)", "atan 1.25"),
        ("sinh(1.25)", "sinh 1.25"),
        ("cosh(1.25)", "cosh 1.25"),
        ("tanh(1.25)", "tanh 1.25"),
        ("asinh(1.25)", "asinh 1.25"),
        ("acosh(2)", "acosh 2"),
        ("atanh(0.5)", "atanh 0.5"),
        ("sqrt(2)", "sqrt 2"),
        ("cbrt(2)", "cbrt 2"),
        ("ln(2)", "ln 2"),
        ("log2(8)", "log2 8"),
        ("log10(100)", "log10 100"),
        ("exp(1.25)", "exp 1.25"),
        ("abs(-1.25)", "abs -1.25"),
        ("sign(-1.25)", "sign -1.25"),
        ("floor(1.25)", "floor 1.25"),
        ("ceil(1.25)", "ceil 1.25"),
        ("round(1.25)", "round 1.25"),
        ("heaviside(-1)", "heaviside -1"),
        ("step(0)", "step 0"),
        ("rect(0.25)", "rect 0.25"),
        ("tri(0.25)", "tri 0.25"),
        ("sinc(1.25)", "sinc 1.25"),
        ("imp(0)", "imp 0"),
    ] {
        assert_eq!(
            eval_expr(explicit, 0.0, 0.0),
            eval_expr(implicit, 0.0, 0.0),
            "parity mismatch: {explicit} vs {implicit}"
        );
    }
}

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
    st.set_func('r'.to_string(), vec!['x'.to_string()], parse_expr("rect(x)").unwrap());
    st.set_func('d'.to_string(), vec!['x'.to_string()], parse_expr("imp(x)").unwrap());

    // (rect * rect)(0) should be ~1.0
    let val = parse_expr("conv(r, r, 0)").unwrap().eval_with_symbols(0.0, 0.0, &st);
    assert!((val - 1.0).abs() < 0.15, "Expected ~1.0, got {val}");

    // (imp * rect)(0.5) should be rect(0.5) = 1.0
    let val2 = parse_expr("dtconv(d, r, 0)").unwrap().eval_with_symbols(0.0, 0.0, &st);
    assert!((val2 - 1.0).abs() < 1e-2, "Expected ~1.0, got {val2}");
}

