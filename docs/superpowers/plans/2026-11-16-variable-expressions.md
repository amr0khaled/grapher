# Variable Expressions and Dependency Resolution Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add symbol table / variable store supporting expression assignment (`i = 5`, `o = m/4`), dependency ordering across arbitrary line order, and function definitions with arguments in the Rust WASM engine.

**Architecture:** 
1. Extend `parser.rs` and `eval.rs` to support variable assignments (`x = expr`), variable node evaluation via symbol resolution, and function definitions/calls with parameters.
2. Implement topological sort / dependency resolution for variables so expressions can reference variables defined anywhere in the script/list.
3. Expose multi-expression evaluation FFI to Rust/WASM.

**Tech Stack:** Rust WASM (engine/src/)

**Spec:** user prompt request for variable expression storage, dependency resolution, and function parameters.

## Global Constraints
- Single-character variables (`Node::Var(char)` / `Tok::Var(char)`).
- Handle arbitrary definition order via dependency evaluation.
- Support function definitions with arguments.

## Review Focus
- Cyclic dependency detection (e.g. `a = b`, `b = a`).
- Unresolved variable handling (returning NaN gracefully).
- Backward compatibility with explicit and implicit function plotting.

---

### Task 1: Variable Store & Dependency Resolution in Rust Engine

**Files:**
- Modify: `engine/src/eval.rs`, `engine/src/parser.rs`
- Test: `engine/tests/variables.rs`

**Interfaces:**
- Consumes: `Node::Var(char)`
- Produces: `eval_with_env` supporting symbol resolution and dependency sorting.

- [ ] **Step 1: Write failing test for variable dependencies**

```rust
#[test]
fn test_variable_dependencies() {
    // defined out of order: o depends on m, m depends on 5
    let exprs = vec!["o = m / 4", "m = 20"];
    // evaluating 'o' should resolve m first and return 5.0
}
```

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test --test variables`
Expected: FAIL

- [ ] **Step 3: Implement variable symbol table and dependency resolution in `eval.rs`**

- [ ] **Step 4: Run test to verify it passes**

Run: `cargo test --test variables`
Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add engine/src/eval.rs engine/tests/variables.rs
git commit -m "feat: add variable storage and topological dependency evaluation"
```
