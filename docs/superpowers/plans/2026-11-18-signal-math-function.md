# Signal Processing & Math Built-ins Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Extend the Rust math engine (`engine/`) with a complete set of math built-in functions and CT/DT Fourier Transform + convolution built-ins, usable from `y = f(x)` graphs.

**Architecture:** Replace single-arg `Node::Fn(String, Box<Node>)` with multi-arg `Node::Fn(String, Vec<Node>)`. All existing call sites (eval, tests, user-defined functions) migrate to `Vec<Node>`. Signal built-ins take an identifier as their first argument naming a user-defined function in the symbol table (e.g. `ctftmag(g, x)` where `g(x)=...`), because the parser has no closure literals. CTFT/convolution use numeric integration (Simpson, fixed bounds/steps); DTFT/DT-convolution use bounded finite sums. Outputs are split into `re`/`im`/`mag` variants — graphing can only plot real/imag/magnitude, not complex numbers.

**Tech Stack:** Rust, wasm-bindgen, existing recursive-descent parser (`engine/src/parser.rs`), `SymbolTable` (`engine/src/eval.rs`).

**Spec:** `SPEC.md` (roadmap: signal processing), current code in `engine/src/`, user decision (2026-11-18): both CT and DT, primitives + convolution + FT.

## Global Constraints

- Built-in names are case-insensitive-tolerant like existing code paths (match on exact lowercase name; tokenizer keeps case — callers write lowercase).
- Unknown built-in name or wrong arity → eval returns `f64::NAN` (existing convention), never a panic.
- `Node::Fn` first argument for transforms must be `Node::Var(c)` referencing `st.funcs`; anything else → `f64::NAN`.
- Integration bounds are fixed internal constants: CT `[-16, 16]`, Simpson with 512 intervals. DT sums `n = 0..63`.
- No serde/json; evaluation stays on the same `eval_with_symbols_rec` path used by batch sampling.
- `cargo test` in `engine/` must pass after every task.
- User-defined functions keep single-arg-only support (`parse_function_definition` unchanged).

## Review Focus

Untested inputs most likely to bite (most likely first):

1. `ctftmag(g, x)` where `g` is **not** defined or arg is an expression, not a `Var` → expect `NaN` path, no panic/crash. Test in Task 4.
2. Malformed multi-arg call: `min(,)`, `min(1,)`, `min()` → expect parse error, not panic. Test in Task 1.
3. CT `conv` cost: 401 px × 512 Simpson intervals × ~3 node evals ≈ manageable, but an expression inside the kernel re-parses nothing (already `Node`) — verify one implicit-sampling test (`y = conv(f, f, x)`) completes quickly. Task 6.
4. `imp(x)` (unit impulse) on the continuous CT grid almost never hits `x == 0` exactly → document as DT-only; Simpson CT tests must not rely on it. Task 3/6.
5. `min`/`max` gain multi-arg support — `min(x)` with one arg and `min(x, 0)` both valid; existing `preprocess_latex` emits `min(a)` single-arg — must keep working. Task 2.

---

### Task 1: Multi-arg function calls in parser + eval migration

**Files:**
- Modify: `engine/src/parser.rs:12,199,174-185`
- Modify: `engine/src/eval.rs:58-98`
- Test: `engine/tests/parsing.rs` (extend), `engine/tests/functions.rs` (update call sites)

**Interfaces:**
- Produces: `Node::Fn(String, Vec<Node>)` — every later task depends on this shape.
- Consumes: nothing new.

- [ ] **Step 1: Write failing test** in `engine/tests/parsing.rs`:

```rust
#[test]
fn test_parse_multi_arg_fn() {
    let n = parse_expr("min(x, 0)").unwrap();
    match n {
        Node::Fn(name, args) => {
            assert_eq!(name, "min");
            assert_eq!(args.len(), 2);
        }
        _ => panic!("expected Fn node"),
    }
    assert!(parse_expr("min()").is_err());
    assert!(parse_expr("min(1,)").is_err());
    assert!(parse_expr("min(,)").is_err());
    assert!(parse_expr("min(x, min(y, 0))").is_ok());
}
```
(add `use grapher_engine::parser::Node;`)

- [ ] **Step 2: Run** `cargo test --test parsing` → FAIL (Fn arity mismatch).

- [ ] **Step 3: Implement**
- `Tok::Fn(String)` stays. Change `Node` to `Fn(String, Vec<Node>)`.
- In `parse_primary`, after consuming the function name and `(`: parse zero-or-more args: first `parse_expr`, then while peek is `,` consume and `parse_expr`. Reject empty arg list with `"expected argument"`; require closing `)`; any other state (e.g. `)` first, trailing `,`) → `"expected )"`/`"expected argument"` error. `Tok` needs a `Comma` token: add `Comma` to `Tok`, emit in tokenizer for `,`.
- `insert_implicit_mul`: `Tok::Comma` never starts/ends expr — no change needed (it only inserts between `ends_expr` and `starts_expr`; comma matches neither).
- `eval.rs`: `Node::Fn(name, args)` → evaluate each `args[i]` recursively into `Vec<f64>` (NaN propagation automatic); pass slice to match.
- User-defined function branch: `args[0]` eval as before; require `args.len() == 1` for single-arg user funcs else NaN.
- Update `engine/tests/functions.rs` construction: `set_func('f', vec!['x'], parse_expr("x * 2").unwrap())` stays; call `Node::Fn` shape change is internal to parser output — only constructor sites (`parse_expr` results) need no edits, but `eval.rs` internal match does.

- [ ] **Step 4: Run** `cargo test` → all existing tests PASS (`sin(x)` etc. parse identically; `functions.rs` still green).

- [ ] **Step 5: Commit**

```bash
git add engine/src/parser.rs engine/src/eval.rs engine/tests/parsing.rs
git commit -m "feat(engine): support multi-argument function calls"
```

### Task 2: Complete math built-in set

**Files:**
- Modify: `engine/src/eval.rs` (builtin match arm)
- Modify: `engine/src/utils.rs:79` (add `\asin`,`\acos`,`\atan`,`\sinh` etc. single-arg latex)
- Test: `engine/tests/builtins.rs` (new)

**Interfaces:**
- Consumes: `Node::Fn(String, Vec<Node>)` from Task 1. Args evaluated to `Vec<f64>`.
- Produces: `asin acos atan sinh cosh tanh asinh acosh atanh abs floor ceil round sign exp ln log log2 min max atan2 hypot mod(d,m)` — Task 4-6 kernels rely on `abs`, `exp`, `min`, `max`.

Naming (locked): `ln` = natural, `log`/`log10` = base 10, `log2` = base 2. `mod(a,b)` returns `a - b*floor(a/b)`, `b == 0` → NaN. `sign(0)` = 0. `atan2(y,x)` 2-arg. `min`/`max` variadic (≥1 arg; wrong count → NaN).

- [ ] **Step 1: Failing tests** in new `engine/tests/builtins.rs`:

```rust
use grapher_engine::eval::eval_expr;
fn approx(a: f64, b: f64) -> bool { (a - b).abs() < 1e-9 }

#[test]
fn test_trig_and_inverse() {
    assert!(approx(eval_expr("asin(1)", 0.0, 0.0), std::f64::consts::FRAC_PI_2));
    assert!(approx(eval_expr("acos(1)", 0.0, 0.0), 0.0));
    assert!(approx(eval_expr("atan(1)", 0.0, 0.0), std::f64::consts::FRAC_PI_4));
    assert!(approx(eval_expr("atan2(1, 1)", 0.0, 0.0), std::f64::consts::FRAC_PI_4));
    assert!(approx(eval_expr("sinh(0)", 0.0, 0.0), 0.0));
    assert!(approx(eval_expr("hypot(3, 4)", 0.0, 0.0), 5.0));
}
#[test]
fn test_rounding_abs_sign() {
    assert_eq!(eval_expr("abs(-7)", 0.0, 0.0), 7.0);
    assert_eq!(eval_expr("floor(2.7)", 0.0, 0.0), 2.0);
    assert_eq!(eval_expr("ceil(2.1)", 0.0, 0.0), 3.0);
    assert_eq!(eval_expr("round(2.5)", 0.0, 0.0), 3.0);
    assert_eq!(eval_expr("sign(-3)", 0.0, 0.0), -1.0);
    assert_eq!(eval_expr("sign(0)", 0.0, 0.0), 0.0);
}
#[test]
fn test_exp_logs() {
    assert!(approx(eval_expr("exp(0)", 0.0, 0.0), 1.0));
    assert!(approx(eval_expr("ln(e)", 0.0, 0.0), 1.0));
    assert!(approx(eval_expr("log(100)", 0.0, 0.0), 2.0));
    assert!(approx(eval_expr("log2(8)", 0.0, 0.0), 3.0));
    assert_eq!(eval_expr("ln(-1)", 0.0, 0.0).is_nan(), true);
}
#[test]
fn test_min_max_variadic() {
    assert_eq!(eval_expr("min(3, 1, 2)", 0.0, 0.0), 1.0);
    assert_eq!(eval_expr("max(3, 1, 2)", 0.0, 0.0), 3.0);
    assert_eq!(eval_expr("min(x)", 5.0, 0.0), 5.0);
    assert_eq!(eval_expr("mod(7, 3)", 0.0, 0.0), 1.0);
    assert!(eval_expr("mod(1, 0)", 0.0, 0.0).is_nan());
}
```

- [ ] **Step 2: Run** `cargo test --test builtins` → FAIL.

- [ ] **Step 3: Implement** builtin match arm in `eval.rs`:
```
"asin" => v.asin(), ... "abs" => v.abs(), "floor" => v.floor(), "ceil" => v.ceil(),
"round" => v.round(), "sign" => v.signum() (0.0 → 0.0 already via signum),
"exp" => v.exp(), "ln" => if v <= 0.0 {NAN} else {v.ln()},
"log"|"log10" => base-10 same guard, "log2" => base-2 same guard,
"sqrt" (existing), "min"|"max" => fold over args, "atan2"|"hypot"|"mod" => 2-arg with arity check (else NaN).
```
Wrong arity on any builtin → NaN. Also add `\asin \acos \atan \abs \floor \ceil \round \sign \sinh \cosh \tanh \log2` to the single-arg latex list in `utils.rs`.

- [ ] **Step 4: Run** `cargo test` → PASS.

- [ ] **Step 5: Commit** `git commit -am "feat(engine): add inverse/hyperbolic/log/rounding/min/max builtins"`

### Task 3: Signal waveform primitives

**Files:** Modify `engine/src/eval.rs`; Test `engine/tests/builtins.rs` (append).

**Interfaces:** Consumes Task 2 `abs`,`min`,`max`. Produces `heaviside step rect tri sinc imp`:
`heaviside(x)` = 0 if x<0, 1 if x>=0. `rect(x)` = 1 if |x| <= 0.5 else 0. `tri(x)` = max(0, 1-|x|). `sinc(x)` = sin(πx)/(πx), sinc(0)=1 (guard |x|<1e-12). `imp(x)` = 1 if x == 0.0 else 0.0 (DT/integer-grid use only; document). Single-arg each.

- [ ] Step 1: Failing tests:
```rust
assert_eq!(eval_expr("heaviside(-1)", 0.0, 0.0), 0.0);
assert_eq!(eval_expr("heaviside(1)", 0.0, 0.0), 1.0);
assert_eq!(eval_expr("step(0)", 0.0, 0.0), 1.0);          // alias
assert_eq!(eval_expr("rect(0)", 0.0, 0.0), 1.0);
assert_eq!(eval_expr("rect(1)", 0.0, 0.0), 0.0);
assert_eq!(eval_expr("tri(0)", 0.0, 0.0), 1.0);
assert!(approx(eval_expr("sinc(0)", 0.0, 0.0), 1.0));
assert!(approx(eval_expr("sinc(1)", 0.0, 0.0), 0.0));
assert_eq!(eval_expr("imp(0)", 0.0, 0.0), 1.0);
assert_eq!(eval_expr("imp(0.5)", 0.0, 0.0), 0.0);
```
- [ ] Step 2: Run → FAIL.
- [ ] Step 3: Implement match arms (`step` = alias of `heaviside`). Single-arg arity checks.
- [ ] Step 4: `cargo test` → PASS.
- [ ] Step 5: Commit `"feat(engine): add signal waveform primitives"`.

### Task 4: CT Fourier Transform (`ctftre`, `ctftim`, `ctftmag`)

**Files:** Modify `engine/src/eval.rs` (add quadrature helper + 3 builtins); Test `engine/tests/builtins.rs` (append).

**Interfaces:** Consumes `Node::Fn` Vec args; `st.funcs` must expose `lookup_func(name: char) -> Option<(Vec<char>, Node)>` — add this accessor to `SymbolTable` (eval currently accesses `st.funcs` privately; the builtin needs the same access — implement as `pub(crate)` method). Signature: `ctftre(f, w) / ctftim(f, w) / ctftmag(f, w)` with `f = Node::Var(c)` referencing a single-arg user-defined function; `w` any expression. `w` is the angular frequency variable.

Algorithm (locked): trapezoid/Simpson over `[-16, 16]`, `N = 512` intervals, `g(t) = f(t) * cos(w*t)` (re) / `f(t) * sin(w*t)` (im). `mag = hypot(re, im)`. f undefined / arg-0 not `Var` / f not single-arg → NaN. Integration variable substituted by cloning `body` and rebuilding with `x -> t` is **rejected** — instead call body with `x = t` via `body.eval_with_symbols_rec(t, f64::NAN, st, visited)` since single-arg user funcs bind their arg to x (existing behavior binds args[0]=='x'). y = NaN (never used by 1-D signal funcs; document).

- [ ] Step 1: Failing tests (use `SymbolTable::set_func` + `eval_with_symbols` — builtins need symbol context, plain `eval_expr` won't see `g`):
```rust
fn with_fn(body: &str, call: &str, w: f64) -> f64 {
    let mut st = SymbolTable::new();
    st.set_func('g', vec!['x'], parse_expr(body).unwrap());
    parse_expr(call).unwrap().eval_with_symbols(0.0, 0.0, &st)
}
#[test]
fn test_ctft_of_exp_abs() {
    let re = with_fn("exp(-abs(x))", "ctftre(g, x)", 0.0);
    assert!(approx(re, 2.0));               // ∫ e^{-|t|} dt = 2
    let im = with_fn("exp(-abs(x))", "ctftim(g, x)", 0.0);
    assert!(im.abs() < 1e-9);
    let mag = with_fn("exp(-abs(x))", "ctftmag(g, x)", 0.0);
    assert!(approx(mag, 2.0));
    assert!(with_fn("exp(-abs(x))", "ctftmag(g, x)", 5.0) < 0.2); // 2/(1+w^2)
}
#[test]
fn test_ctft_arg_must_be_var() {
    assert!(with_fn("x", "ctftmag(1, x)", 0.0).is_nan());  // arg0 not Var
    assert!(with_fn("x", "ctftmag(h, x)", 0.0).is_nan());  // h undefined
}
```
(use `y`-as-w trick: pass w through first arg position by calling with x = w; simpler: call `eval_with_symbols(w, 0.0, &st)` and call expr `"ctftmag(g, x)"` with x = w. Adjust helper accordingly: `parse_expr(call).eval_with_symbols(w, 0.0, &st)`.)
- [ ] Step 2: Run → FAIL.
- [ ] Step 3: Implement `quad(t -> f64, n: usize) -> f64` Simpson helper in `eval.rs` (private fn), plus 3 match arms with arg validation (Var + arity 2 + func lookup).
- [ ] Step 4: `cargo test` → PASS.
- [ ] Step 5: Commit `"feat(engine): add continuous Fourier transform builtins"`.

### Task 5: DT Fourier Transform / finite DFT (`dtftre`, `dtftim`, `dtftmag`, `dftre`, `dftim`, `dftmag`)

**Files:** Modify `engine/src/eval.rs`; Test `engine/tests/builtins.rs` (append).

**Interfaces:** Consumes same func-lookup accessor as Task 4. `dtftre(f, w)`, `dtftim(f, w)`, `dtftmag(f, w)`: sum over `n = 0..63` of `f(n) · e^{-j w n}`, returns re/im/magnitude. `dftre(f, k, n)`, `dftim(f, k, n)`, `dftmag(f, k, n)`: DFT bin `k` (0..n-1) of finite sequence `f(m)`, `m = 0..n-1`; `n` clamped to `1..=256`. All arg0=Var-func, else NaN.

- [ ] Step 1: Failing tests:
```rust
#[test]
fn test_dtft_of_impulse() {
    // f(n) = 1 if n == 0 else 0  →  X(e^jw) = 1 for all w
    let mut st = SymbolTable::new();
    st.set_func('d', vec!['x'], parse_expr("imp(x)").unwrap());
    for w in [0.0, 1.0, 2.5, std::f64::consts::PI] {
        let v = parse_expr("dtftmag(d, x)").unwrap().eval_with_symbols(w, 0.0, &st);
        assert!(approx(v, 1.0));
    }
}
#[test]
fn test_dft_of_impulse_bin_count() {
    let mut st = SymbolTable::new();
    st.set_func('d', vec!['x'], parse_expr("imp(x)").unwrap());
    let v = parse_expr("dftmag(d, 3, 8)").unwrap().eval_with_symbols(0.0, 0.0, &st);
    assert!(approx(v, 1.0));
}
#[test]
fn test_dtft_undefined_fn() {
    let st = SymbolTable::new();
    assert!(parse_expr("dtftmag(d, x)").unwrap().eval_with_symbols(1.0, 0.0, &st).is_nan());
}
```
- [ ] Step 2: Run → FAIL.
- [ ] Step 3: Implement 6 match arms (sum loops; `dft*` arity 3, `dtft*` arity 2; arg0 Var-func lookup; n clamp).
- [ ] Step 4: `cargo test` → PASS.
- [ ] Step 5: Commit `"feat(engine): add discrete Fourier transform / DTFT builtins"`.

### Task 6: Convolution (`conv`, `dtconv`)

**Files:** Modify `engine/src/eval.rs`; Test `engine/tests/builtins.rs` (append).

**Interfaces:** Consumes Task 4 Simpson helper. `conv(f, g, t)` = CT convolution `∫_{-16}^{16} f(τ)·g(t-τ) dτ` (512 Simpson intervals). `dtconv(f, g, n)` = `Σ_{m=-32..32} f(m)·g(n-m)`. arg0/arg1 = `Var` identifiers of single-arg user funcs; else NaN. `t`/`n` need not be integer for `dtconv` (sum over integer lattice anyway — document).

- [ ] Step 1: Failing tests:
```rust
#[test]
fn test_ct_conv_rect_rect() {
    let mut st = SymbolTable::new();
    st.set_func('r', vec!['x'], parse_expr("rect(x)").unwrap());
    // (rect * rect)(0) = ∫ rect(τ)·rect(-τ) dτ = 1 (overlap width 1)
    let v = parse_expr("conv(r, r, x)").unwrap().eval_with_symbols(0.0, 0.0, &st);
    assert!(approx(v, 1.0));
    // (rect * rect)(2) = 0
    let v = parse_expr("conv(r, r, x)").unwrap().eval_with_symbols(2.0, 0.0, &st);
    assert!(v.abs() < 1e-9);
}
#[test]
fn test_dt_conv_with_imp() {
    let mut st = SymbolTable::new();
    st.set_func('d', vec!['x'], parse_expr("imp(x)").unwrap());
    st.set_func('g', vec!['x'], parse_expr("3 * x + 1").unwrap());
    let v = parse_expr("dtconv(d, g, x)").unwrap().eval_with_symbols(4.0, 0.0, &st);
    assert!(approx(v, 13.0));   // (δ * g)(n) = g(n)
}
#[test]
fn test_conv_bad_args() {
    let st = SymbolTable::new();
    assert!(parse_expr("conv(r, r, x)").unwrap().eval_with_symbols(0.0, 0.0, &st).is_nan());
}
```
- [ ] Step 2: Run → FAIL.
- [ ] Step 3: Implement 2 match arms; CT uses Simpson helper; DT uses plain loop.
- [ ] Step 4: `cargo test` → PASS (all suites, incl. `sampling.rs` implicit tests unchanged).
- [ ] Step 5: Commit `"feat(engine): add continuous and discrete convolution builtins"`.

### Task 7: End-to-end graph sampling smoke test

**Files:** Test `engine/tests/builtins.rs` (append). Modify nothing.

- [ ] Step 1: Add test proving builtins flow through WASM-facing `sampling::sample`:
```rust
#[test]
fn test_sample_graph_with_builtins() {
    // Line 1 defines g(x) = e^{-|x|}; line 2 graphs its |CTFT| vs frequency x.
    let out = crate::sampling::sample(
        vec![
            "g(x) = exp(-abs(x))".to_string(),
            "y = ctftmag(g, x)".to_string(),
        ],
        50.0, 400.0, 300.0, 800, 600,
    );
    assert_eq!(out.len(), 2);
    assert!(out[1].iter().any(|v| v.is_finite()), "FT graph produced no finite points");
}
```
NOTE: requires that graph lines like `g(x) = ...` be routed through function-definition parsing in `sampling::build_symbols` — verify how `g(x) = exp(-abs(x))` is currently classified there; if it is skipped entirely, instead set the function via `set_func` in a unit-test path (i.e. test the equivalent through `eval_with_symbols`) and note it. Do NOT restructure `sampling.rs` in this task; if the graph line can't reference user funcs, assert only that an unknown-builtin line yields an empty/NaN path without crashing, and file the finding in the plan as a follow-up. Reading `sampling.rs` first is required before writing this step.
- [ ] Step 2: Run `cargo test` → PASS.
- [ ] Step 3: Commit `"test(engine): builtin functions end-to-end sampling smoke test"`.
