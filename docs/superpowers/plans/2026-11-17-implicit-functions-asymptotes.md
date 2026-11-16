# Implicit Functions and Asymptotes Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Implement implicit function rendering ($f(x,y)=0$) via marching squares / sampling and asymptote/pole detection in the Rust math core.

**Architecture:** Extend Rust WASM engine (`engine/src/`) with 2D sampling grid evaluations for implicit equations and pole/asymptote discontinuity filtering.

**Tech Stack:** Rust, wasm-bindgen, Svelte + Vite frontend.

**Spec:** Roadmap items in `CLAUDE.md`.

## Global Constraints
- Rust code must compile without errors under `engine/`.
- No new dependencies.

## Review Focus
- Division by zero / tan poles handled gracefully without crashing sampling.
- Implicit contour line correctness on boundary crossings.

---

### Task 1: Implicit function evaluation support in Rust core

**Files:**
- Modify: `engine/src/eval.rs`
- Test: `engine/tests/implicit_functions.rs`

**Interfaces:**
- Produces: `sample()` handling 2-variable implicit expressions $f(x,y) = 0$.

- [ ] **Step 1: Write failing test for implicit function sampling**
- [ ] **Step 2: Run cargo test to verify failure**
- [ ] **Step 3: Implement minimal implicit sampling support in `engine/src/eval.rs`**
- [ ] **Step 4: Run cargo test to verify pass**
- [ ] **Step 5: Commit changes**
