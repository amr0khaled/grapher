// Shared preprocessing helpers.

// --- LaTeX-ish preprocessing ---
// Translates a small LaTeX subset into the token language the tokenizer already speaks.
// Runs once per parse, never on the sample() hot path.

pub(crate) fn find_braced_group(b: &[u8], i: usize) -> Option<(usize, usize)> {
    // b[i] == b'{'; returns (start, end_exclusive) of contents
    let mut depth = 0usize;
    for j in i..b.len() {
        match b[j] {
            b'{' => depth += 1,
            b'}' => {
                depth -= 1;
                if depth == 0 {
                    return Some((i + 1, j));
                }
            }
            _ => {}
        }
    }
    None
}

pub(crate) fn replace_command_args(
    s: &str,
    prefix: &str,
    n: usize,
    make: impl Fn(&[&str]) -> String,
) -> String {
    let mut out = String::with_capacity(s.len());
    let mut rest = s;
    while let Some(idx) = rest.find(prefix) {
        let mut args: Vec<&str> = Vec::with_capacity(n);
        let mut scan = idx + prefix.len();
        let mut ok = true;
        for _ in 0..n {
            while scan < rest.len() && rest.as_bytes()[scan].is_ascii_whitespace() {
                scan += 1;
            }
            if scan >= rest.len() || rest.as_bytes()[scan] != b'{' {
                ok = false;
                break;
            }
            match find_braced_group(rest.as_bytes(), scan) {
                Some((a, b2)) => {
                    args.push(&rest[a..b2]);
                    scan = b2 + 1;
                }
                None => {
                    ok = false;
                    break;
                }
            }
        }
        if !ok {
            break;
        }
        out.push_str(&rest[..idx]);
        out.push_str(&make(&args));
        rest = &rest[scan..];
    }
    out.push_str(rest);
    out
}

pub(crate) fn preprocess_latex(s: &str) -> String {
    let mut r = s.to_string();

    // \frac{a}{b} -> ((a)/(b))
    r = replace_command_args(&r, "\\frac", 2, |args| {
        format!("(({})/({}))", args[0], args[1])
    });
    // \sqrt{x} -> sqrt(x)
    r = replace_command_args(&r, "\\sqrt", 1, |args| format!("sqrt({})", args[0]));

    // Single-arg function commands -> name(x)
    for cmd in [
        "\\sin", "\\cos", "\\tan", "\\ln", "\\exp", "\\min", "\\max", "\\abs",
    ] {
        let name = &cmd[1..];
        r = replace_command_args(&r, cmd, 1, |args| format!("{}({})", name, args[0]));
    }
    // \log_{b}{x} -> log(x, b)
    r = replace_command_args(&r, "\\log", 2, |args| {
        format!("log({}, {})", args[1], args[0])
    });
    // \log{x} -> log(x)  (matches our base-10 convention)
    r = replace_command_args(&r, "\\log", 1, |args| format!("log({})", args[0]));
    // \floor{x}, \ceil{x} if present
    r = replace_command_args(&r, "\\floor", 1, |args| format!("floor({})", args[0]));
    r = replace_command_args(&r, "\\ceil", 1, |args| format!("ceil({})", args[0]));

    // Symbol replacements
    r = r
        .replace("\\pi", "pi")
        .replace("\\cdot", "*")
        .replace("\\times", "*")
        .replace("\\left(", "(")
        .replace("\\right(", ")")
        .replace("\\left[", "(")
        .replace("\\right]", ")")
        .replace("\\left|", "abs(")
        .replace("\\right|", ")")
        .replace("\\{", "(")
        .replace("\\}", ")");

    // Collapse stray whitespace around operators created by replacements
    r = r.replace('\u{00a0}', " ");
    r
}

// Implicit multiplication: 2x -> 2*x, 2( -> 2*(, )( -> )*(, )x -> )*x, x( -> x*(
// between adjacent tokens where left is Num|Var|RParen|Fn-name and right is Num|Var|LParen|Fn
pub(crate) fn insert_implicit_mul(tokens: Vec<crate::parser::Tok>) -> Vec<crate::parser::Tok> {
    use crate::parser::Tok;
    fn starts_expr(t: &Tok) -> bool {
        match t {
            Tok::Num(_) | Tok::VarX | Tok::VarY | Tok::LParen | Tok::Fn(_) => true,
            _ => false,
        }
    }
    fn ends_expr(t: &Tok) -> bool {
        match t {
            Tok::Num(_) | Tok::VarX | Tok::VarY | Tok::RParen => true,
            _ => false,
        }
    }
    let mut out: Vec<Tok> = Vec::with_capacity(tokens.len() * 2);
    for t in tokens {
        if let Some(prev) = out.last() {
            if ends_expr(prev) && starts_expr(&t) {
                out.push(Tok::Op('*'));
            }
        }
        out.push(t);
    }
    out
}

#[macro_export]
macro_rules! console_log {
    ($($t:tt)*) => {
        #[cfg(debug_assertions)]
        web_sys::console::log_1(&format!($($t)*).into());
    }
}

#[macro_export]
macro_rules! console_trace {
    ($($t:tt)*) => {
        #[cfg(debug_assertions)]
        web_sys::console::trace_1(&format!($($t)*).into());
    }
}
