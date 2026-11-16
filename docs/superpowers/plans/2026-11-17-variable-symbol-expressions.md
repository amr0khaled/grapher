# Variable and Symbol Expression Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Implement a variable/symbol expression system with dependent evaluation, single-character case-sensitive variables, functions with expression arguments, and per-line syntax error reporting.

**Architecture:** Extend `engine/src/eval.rs` with a `SymbolTable` that stores expressions (not just values) and supports recursive/topological resolution. Extend `engine/src/parser.rs` to support assignment (`a=...`) and function call syntax.

**Tech Stack:** Rust (WASM math core)

**Spec:** Variable/symbol system: assignments, single-character (case-sensitive) variables/symbols, dependent evaluation, function calls with expression arguments, syntax error detection per-line.

## Global Constraints
- Rust code must compile without errors under `engine/`.
- No new dependencies.
- Case-sensitive single-character symbols/variables.
- Dependent expression evaluation (evaluating symbol `b` should evaluate `a` if `b=a+1`).

## Review Focus
1. Circular dependencies in expression evaluation (`a=b+1`, `b=a+1`).
2. Invalid symbol names or syntax in assignments.
3. Errors in function argument evaluation.

---

### Task 1: Symbol Table & Dependent Evaluation Support

**Files:**
- Modify: `engine/src/eval.rs`
- Test: `engine/tests/symbol_evaluation.rs`

**Interfaces:**
- Consumes: N/A
- Produces: `SymbolTable` struct with `set_expression(char, Expr)`, `get_value(char)`, and cycle detection.

- [ ] **Step 1: Write failing test in `engine/tests/symbol_evaluation.rs` for dependency resolution**
- [ ] **Step 2: Run test to verify failure**
- [ ] **Step 3: Implement `SymbolTable` with dependency tracking in `engine/src/eval.rs`**
- [ ] **Step 4: Run test to verify pass**
- [ ] **Step 5: Commit changes**

### Task 2: Single-Character Variables & Assignment Parsing

**Files:**
- Modify: `engine/src/parser.rs`
- Modify: `engine/src/eval.rs`
- Test: `engine/tests/parsing.rs`

**Interfaces:**
- Consumes: `SymbolTable` from Task 1.
- Produces: Updated parser to support assignments (`a=...`) and variable usage in expressions.

- [ ] **Step 1: Write failing test for assignment parsing and variable usage**
- [ ] **Step 2: Run test to verify failure**
- [ ] **Step 3: Implement assignment parsing in `engine/src/parser.rs`**
- [ ] **Step 4: Run test to verify pass**
- [ ] **Step 5: Commit changes**

### Task 3: Function Definitions & Argument Evaluation

**Files:**
- Modify: `engine/src/eval.rs`
- Modify: `engine/src/parser.rs`
- Test: `engine/tests/functions.rs`

**Interfaces:**
- Consumes: `SymbolTable` and `Parser` from Task 1 & 2.
- Produces: Support for `f(x, y)` where arguments are expressions.

- [ ] **Step 1: Write failing test for function with expression arguments**
- [ ] **Step 2: Run test to verify failure**
- [ ] **Step 3: Implement function expression evaluation in `engine/src/eval.rs`**
- [ ] **Step 4: Run test to verify pass**
- [ ] **Step 5: Commit changes**

### Task 4: Syntax Error Detection & Reporting

**Files:**
- Modify: `engine/src/parser.rs`
- Test: `engine/tests/errors.rs`

**Interfaces:**
- Consumes: Updated `Parser`.
- Produces: Per-line error messages for invalid syntax.

- [ ] **Step 1: Write failing test for syntax error detection**
- [ ] **Step 2: Run test to verify failure**
- [ ] **Step 3: Implement syntax error tracking in `engine/src/parser.rs`**
- [ ] **Step 4: Run test to verify pass**
- [ ] **Step 5: Commit changes**
