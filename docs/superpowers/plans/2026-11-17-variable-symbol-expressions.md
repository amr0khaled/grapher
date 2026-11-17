# Variable and Symbol Expression Implementation Plan (Expanded)

**Goal:** Implement a variable/symbol expression system with dependent evaluation, single-character case-sensitive variables, user-defined functions (`f(x) = expr`) and builtin functions with expression arguments, cycle detection, and syntax error detection.

**Architecture:**
- Extend `engine/src/parser.rs` to parse assignments (`a = expr` and `f(x) = expr`) and function calls (`f(expr)`).
- Extend `engine/src/eval.rs` with `SymbolTable` supporting constants (`a = 5`), dependent expressions (`b = a + 1`), and function definitions (`f(x) = x^2 + 1`).
- Pass `SymbolTable` and visited set through recursive evaluation (`Node::eval(&self, x: f64, y: f64, symbols: &SymbolTable, visiting: &mut HashSet<char>) -> f64`).

---

### Task 1: Symbol Table & Dependent Evaluation Support

**Files:**
- Modify: `engine/src/eval.rs`
- Test: `engine/tests/symbol_evaluation.rs` (or unit tests in `engine/src/eval.rs`)

**Steps:**
- [ ] **Step 1: Write failing test in `engine/tests/symbol_evaluation.rs`**
  ```rust
  use grapher_engine::eval::SymbolTable;
  use grapher_engine::parser::parse_expr;
  use std::collections::HashMap;

  #[test]
  fn test_dependent_evaluation() {
      let mut st = SymbolTable::new();
      st.set_expr('a', parse_expr("5").unwrap());
      st.set_expr('b', parse_expr("a + 2").unwrap());
      assert_eq!(st.eval_symbol('b', 0.0, 0.0), 7.0);
  }

  #[test]
  fn test_cycle_detection() {
      let mut st = SymbolTable::new();
      st.set_expr('a', parse_expr("b").unwrap());
      st.set_expr('b', parse_expr("a").unwrap());
      assert!(st.eval_symbol('a', 0.0, 0.0).is_nan());
  }
  ```
- [ ] **Step 2: Run test to verify failure**
  `cargo test --test symbol_evaluation`
- [ ] **Step 3: Implement `SymbolTable` with cycle detection in `engine/src/eval.rs`**
- [ ] **Step 4: Run test to verify pass**
- [ ] **Step 5: Commit changes**

---

### Task 2: Single-Character Variables & Assignment Parsing

**Files:**
- Modify: `engine/src/parser.rs`
- Modify: `engine/src/eval.rs`
- Test: `engine/tests/parsing.rs`

**Steps:**
- [ ] **Step 1: Write failing test for assignment parsing (`a = 1 + 2`)**
  ```rust
  #[test]
  fn test_parse_assignment() {
      let (name, node) = grapher_engine::parser::parse_assignment("a = 3 + x").unwrap();
      assert_eq!(name, 'a');
      assert_eq!(node.eval(2.0, 0.0, &Default::default()), 5.0);
  }
  ```
- [ ] **Step 2: Run test to verify failure**
  `cargo test --test parsing`
- [ ] **Step 3: Implement assignment parsing in `engine/src/parser.rs`**
- [ ] **Step 4: Run test to verify pass**
- [ ] **Step 5: Commit changes**

---

### Task 3: User-Defined Functions & Expression Arguments

**Files:**
- Modify: `engine/src/eval.rs`
- Modify: `engine/src/parser.rs`
- Test: `engine/tests/functions.rs`

**Steps:**
- [ ] **Step 1: Write failing test for `f(x) = x^2 + 1` and `f(3 + 1)`**
  ```rust
  #[test]
  fn test_user_defined_function() {
      let mut st = SymbolTable::new();
      // f(x) = x * 2
      st.set_func('f', vec!['x'], parse_expr("x * 2").unwrap());
      // evaluate f(3 + 2) -> 10
      let call_node = parse_expr("f(3 + 2)").unwrap();
      assert_eq!(call_node.eval_with_symbols(0.0, 0.0, &st), 10.0);
  }
  ```
- [ ] **Step 2: Run test to verify failure**
  `cargo test --test functions`
- [ ] **Step 3: Implement function definitions and call evaluation**
- [ ] **Step 4: Run test to verify pass**
- [ ] **Step 5: Commit changes**

---

### Task 4: Syntax Error Detection & Reporting

**Files:**
- Modify: `engine/src/parser.rs`
- Test: `engine/tests/errors.rs`

**Steps:**
- [ ] **Step 1: Write failing test for syntax errors in definitions (`a = 1 + `, malformed assignments)**
  ```rust
  #[test]
  fn test_syntax_errors() {
      assert!(grapher_engine::parser::parse_assignment("a = 1 + ").is_err());
      assert!(grapher_engine::parser::parse_assignment("invalid_name = 5").is_err());
  }
  ```
- [ ] **Step 2: Run test to verify failure**
  `cargo test --test errors`
- [ ] **Step 3: Implement strict error reporting in `engine/src/parser.rs`**
- [ ] **Step 4: Run test to verify pass**
- [ ] **Step 5: Commit changes**
