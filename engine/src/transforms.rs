use crate::eval::SymbolTable;
use crate::parser::Node;

/// Evaluate a function or expression given by name or node, or fallback.
/// If `f_node` is provided or function exists in st, or we evaluate an expression.
pub(crate) fn eval_f_at(
    st: &SymbolTable,
    f_node: Option<&Node>,
    f_name: Option<&String>,
    x: f64,
) -> f64 {
    if let Some(node) = f_node {
        node.eval_with_symbols(x, 0.0, st)
    } else if let Some(name) = f_name {
        if let Some((args, body)) = st.lookup_func(&name) {
            if args.len() == 1 {
                let mut sub = st.clone();
                let f_arg = args[0].clone();
                if f_arg == "x" {
                    body.eval_with_symbols(x, 0.0, &sub)
                } else if f_arg == "y" {
                    body.eval_with_symbols(0.0, x, &sub)
                } else {
                    sub.set_expr(f_arg, Node::Num(x));
                    body.eval_with_symbols(0.0, 0.0, &sub)
                }
            } else {
                f64::NAN
            }
        } else {
            st.eval_symbol(name.clone(), x, 0.0)
        }
    } else {
        f64::NAN
    }
}

/// Helper to parse first argument as a function name or node.
fn parse_f_arg(arg: &Node) -> (Option<String>, Option<Node>) {
    match arg {
        Node::Var(c) => (Some(c.clone()), None),
        Node::Fn(name, subargs) if subargs.is_empty() => (Some(name.clone()), None),
        _ => (None, Some(arg.clone())),
    }
}

/// Continuous Fourier Transform Real part: ctftre(f, omega, T, N) or ctftre(f, omega)
pub(crate) fn ctftre(st: &SymbolTable, args: &[Node]) -> f64 {
    if args.len() < 2 {
        return f64::NAN;
    }
    let (f_name, f_node) = parse_f_arg(&args[0]);
    let omega = args[1].eval_with_symbols(0.0, 0.0, st);
    let t_bound = if args.len() > 2 {
        args[2].eval_with_symbols(0.0, 0.0, st)
    } else {
        10.0
    };
    let n = if args.len() > 3 {
        args[3].eval_with_symbols(0.0, 0.0, st) as usize
    } else {
        1000
    };

    // Simpson's rule integration over [-t_bound, t_bound] of f(t) * cos(omega * t)
    let n = if n % 2 == 1 { n + 1 } else { n }; // Simpson's rule requires even N
    let dt = (2.0 * t_bound) / (n as f64);
    let mut sum = 0.0;

    for i in 0..=n {
        let t = -t_bound + (i as f64) * dt;
        let val = eval_f_at(st, f_node.as_ref(), f_name.as_ref(), t);
        if !val.is_finite() {
            continue;
        }
        let weight = if i == 0 || i == n {
            1.0
        } else if i % 2 == 1 {
            4.0
        } else {
            2.0
        };
        sum += weight * val * (omega * t).cos();
    }
    (dt / 3.0) * sum
}

/// Continuous Fourier Transform Imaginary part: ctftim(f, omega, T, N)
pub(crate) fn ctftim(st: &SymbolTable, args: &[Node]) -> f64 {
    if args.len() < 2 {
        return f64::NAN;
    }
    let (f_name, f_node) = parse_f_arg(&args[0]);
    let omega = args[1].eval_with_symbols(0.0, 0.0, st);
    let t_bound = if args.len() > 2 {
        args[2].eval_with_symbols(0.0, 0.0, st)
    } else {
        10.0
    };
    let n = if args.len() > 3 {
        args[3].eval_with_symbols(0.0, 0.0, st) as usize
    } else {
        1000
    };

    let n = if n % 2 == 1 { n + 1 } else { n };
    let dt = (2.0 * t_bound) / (n as f64);
    let mut sum = 0.0;

    for i in 0..=n {
        let t = -t_bound + (i as f64) * dt;
        let val = eval_f_at(st, f_node.as_ref(), f_name.as_ref(), t);
        if !val.is_finite() {
            continue;
        }
        let weight = if i == 0 || i == n {
            1.0
        } else if i % 2 == 1 {
            4.0
        } else {
            2.0
        };
        sum += weight * val * (-omega * t).sin();
    }
    (dt / 3.0) * sum
}

/// Continuous Fourier Transform Amplitude (Magnitude): ctftag(f, omega, T, N)
pub(crate) fn ctftag(st: &SymbolTable, args: &[Node]) -> f64 {
    let re = ctftre(st, args);
    let im = ctftim(st, args);
    (re * re + im * im).sqrt()
}

/// Discrete Fourier Transform helpers: input is evaluated at sample points or vector/expression.
/// For dtftre(f, omega, N) or dtftre(f, omega)
pub(crate) fn dtftre(st: &SymbolTable, args: &[Node]) -> f64 {
    if args.len() < 2 {
        return f64::NAN;
    }
    let (f_name, f_node) = parse_f_arg(&args[0]);
    let omega = args[1].eval_with_symbols(0.0, 0.0, st);
    let n = if args.len() > 2 {
        args[2].eval_with_symbols(0.0, 0.0, st) as usize
    } else {
        64
    };

    let mut sum = 0.0;
    for k in 0..n {
        let t = k as f64;
        let val = eval_f_at(st, f_node.as_ref(), f_name.as_ref(), t);
        if val.is_finite() {
            sum += val * (omega * t).cos();
        }
    }
    sum
}

pub(crate) fn dtftim(st: &SymbolTable, args: &[Node]) -> f64 {
    if args.len() < 2 {
        return f64::NAN;
    }
    let (f_name, f_node) = parse_f_arg(&args[0]);
    let omega = args[1].eval_with_symbols(0.0, 0.0, st);
    let n = if args.len() > 2 {
        args[2].eval_with_symbols(0.0, 0.0, st) as usize
    } else {
        64
    };

    let mut sum = 0.0;
    for k in 0..n {
        let t = k as f64;
        let val = eval_f_at(st, f_node.as_ref(), f_name.as_ref(), t);
        if val.is_finite() {
            sum += val * (-omega * t).sin();
        }
    }
    sum
}

pub(crate) fn dtftmag(st: &SymbolTable, args: &[Node]) -> f64 {
    let re = dtftre(st, args);
    let im = dtftim(st, args);
    (re * re + im * im).sqrt()
}

/// Standard DFT (Discrete Fourier Transform at index m out of N points):
/// dftre(f, m, N), dftim(f, m, N), dftmag(f, m, N)
pub(crate) fn dftre(st: &SymbolTable, args: &[Node]) -> f64 {
    if args.len() < 3 {
        return f64::NAN;
    }
    let (f_name, f_node) = parse_f_arg(&args[0]);
    let m = args[1].eval_with_symbols(0.0, 0.0, st);
    let n = args[2].eval_with_symbols(0.0, 0.0, st) as usize;
    if n == 0 {
        return 0.0;
    }

    let mut sum = 0.0;
    for k in 0..n {
        let val = eval_f_at(st, f_node.as_ref(), f_name.as_ref(), k as f64);
        if val.is_finite() {
            let angle = -2.0 * std::f64::consts::PI * (m * (k as f64)) / (n as f64);
            sum += val * angle.cos();
        }
    }
    sum
}

pub(crate) fn dftim(st: &SymbolTable, args: &[Node]) -> f64 {
    if args.len() < 3 {
        return f64::NAN;
    }
    let (f_name, f_node) = parse_f_arg(&args[0]);
    let m = args[1].eval_with_symbols(0.0, 0.0, st);
    let n = args[2].eval_with_symbols(0.0, 0.0, st) as usize;
    if n == 0 {
        return 0.0;
    }

    let mut sum = 0.0;
    for k in 0..n {
        let val = eval_f_at(st, f_node.as_ref(), f_name.as_ref(), k as f64);
        if val.is_finite() {
            let angle = -2.0 * std::f64::consts::PI * (m * (k as f64)) / (n as f64);
            sum += val * angle.sin();
        }
    }
    sum
}

pub(crate) fn dftmag(st: &SymbolTable, args: &[Node]) -> f64 {
    let re = dftre(st, args);
    let im = dftim(st, args);
    (re * re + im * im).sqrt()
}

/// Continuous Convolution: conv(f, g, t)
pub(crate) fn conv(st: &SymbolTable, args: &[Node]) -> f64 {
    if args.len() < 3 {
        return f64::NAN;
    }
    let (f_name, f_node) = parse_f_arg(&args[0]);
    let (g_name, g_node) = parse_f_arg(&args[1]);
    let t = args[2].eval_with_symbols(0.0, 0.0, st);

    let t_bound = 16.0;
    let n = 256; // Simpson intervals (even)
    let dt = (2.0 * t_bound) / (n as f64);
    let mut sum = 0.0;

    for i in 0..=n {
        let tau = -t_bound + (i as f64) * dt;
        let f_val = eval_f_at(st, f_node.as_ref(), f_name.as_ref(), tau);
        let g_val = eval_f_at(st, g_node.as_ref(), g_name.as_ref(), t - tau);
        if !f_val.is_finite() || !g_val.is_finite() {
            continue;
        }
        let weight = if i == 0 || i == n {
            1.0
        } else if i % 2 == 1 {
            4.0
        } else {
            2.0
        };
        sum += weight * f_val * g_val;
    }
    (dt / 3.0) * sum
}

/// Discrete Convolution: dtconv(f, g, n)
pub(crate) fn dtconv(st: &SymbolTable, args: &[Node]) -> f64 {
    if args.len() < 3 {
        return f64::NAN;
    }
    let (f_name, f_node) = parse_f_arg(&args[0]);
    let (g_name, g_node) = parse_f_arg(&args[1]);
    let n_val = args[2].eval_with_symbols(0.0, 0.0, st) as i32;

    let mut sum = 0.0;
    for m in -32..=32 {
        let tau = m as f64;
        let f_val = eval_f_at(st, f_node.as_ref(), f_name.as_ref(), tau);
        let g_val = eval_f_at(st, g_node.as_ref(), g_name.as_ref(), (n_val - m) as f64);
        if f_val.is_finite() && g_val.is_finite() {
            sum += f_val * g_val;
        }
    }
    sum
}
