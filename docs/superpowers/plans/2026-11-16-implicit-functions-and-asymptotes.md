# Implicit Functions and Asymptotes Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add implicit function plotting (f(x,y)=0) and asymptote/hole detection to the graphing engine.

**Architecture:** 
1. Modify Rust WASM engine to support implicit function parsing and sampling via marching squares algorithm
2. Update GraphCanvas component to render implicit function paths
3. Add singularity detection for asymptotes and holes in the sampling pipeline

**Tech Stack:** 
- Rust WASM (engine/src/lib.rs)
- Svelte frontend (src/components/GraphCanvas.svelte)
- MathLive for equation parsing (existing)

**Spec:** 
- Implicit functions require new parsing approach and sampling algorithm
- Asymptotes need singularity detection in evaluation
- All changes must maintain backward compatibility with existing explicit functions

---
### Review Focus
- Implicit function sampling accuracy and performance
- Asymptote/hole detection and rendering correctness
- Backward compatibility with existing explicit function plotting
- Grid alignment and path continuity for implicit functions

---
### Task 1: Implicit Function Support in Rust Engine

**Files:**
- Modify: `engine/src/lib.rs` 
- Test: `engine/tests/implicit_functions.rs`

**Interfaces:**
- Consumes: `parse_expr()` function signature and node structure
- Produces: New parsing path for implicit equations, modified `sample()` function signature

- [ ] **Step 1: Write the failing test**

```rust
#[test]
fn test_implicit_function_sampling() {
    // This test will fail initially
    let result = sample("x^2 + y^2 - 4", 0.1, 0.0, 0.0, 800, 600);
    // Should contain valid (x,y) points forming a circle
    assert!(!result.is_empty());
}
```

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test --features test-implicit --test implicit_functions`
Expected: FAIL with "function not implemented"

- [ ] **Step 3: Implement implicit function parsing and sampling**

One line on approach: Use marching squares algorithm on grid-based sampling with sign change detection.

- [ ] **Step 4: Run test to verify it passes**

Run: `cargo test --features test-implicit --test implicit_functions`
Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add engine/src/lib.rs
git commit -m "feat: add implicit function sampling via marching squares"
```