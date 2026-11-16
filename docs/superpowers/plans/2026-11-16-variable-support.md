# Variable Support & Expression Storage Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add symbol table storage for expressions/equations (supporting forward/backward references with dependency resolution, variable evaluation, and functions with arguments) using `Node::Var(char)` in the Rust WASM engine.

**Architecture:** 
1. Extend `Node` and evaluation context with variable assignment storage (`HashMap<char, Node>`).
2. Implement topological sort / dependency resolution for evaluating dependent variables (`o = m / 4`).
3. Support custom function definitions with arguments.

**Tech Stack:** 
- Rust WASM (`engine/src/`)

## Global Constraints
- Single-character variables (`Node::Var(char)`).
- Dependency resolution for multi-variable expressions defined in any order.

## Review Focus
- Cyclic dependency detection and error reporting.
- Correct order of evaluation for dependent variables.
- Function parameter substitution during evaluation.

---

### Task 1: Variable Storage and Dependency Evaluation

**Files:**
- Modify: `engine/src/eval.rs`
- Modify: `engine/src/parser.rs`
- Test: `engine/tests/variables.rs`

**Interfaces:**
- Consumes: `Node::Var(char)`
- Produces: Variable assignment evaluation and dependency resolution.

- [ ] **Step 1: Write the failing test**

```rust
#[test]
fn test_variable_dependencies() {
    // o depends on m, defined out of order
    let mut table = HashMap::new();
    // evaluate o = m / 4 where m = 8 -> should give 2.0
}
```

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test --test variables`
Expected: FAIL

- [ ] **Step 3: Implement variable table and dependency resolution in Rust engine**

- [ ] **Step 4: Run test to verify it passes**

Run: `cargo test --test variables`
Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add engine/src/eval.rs engine/tests/variables.rs
git commit -m "feat: add variable dependency evaluation and symbol storage"
```
