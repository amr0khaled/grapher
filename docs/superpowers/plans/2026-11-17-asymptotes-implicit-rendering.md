# Asymptotes and Implicit Function Rendering Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Implement robust asymptote handling and marching squares / interval arithmetic for implicit functions in the Rust WASM math engine and frontend.

**Architecture:** Extend Rust engine (`engine/src/`) with marching squares root finding / interval evaluation for implicit equations ($f(x, y) = 0$), pole/asymptote detection for rational/trig functions, and wire results into `GraphCanvas.svelte`.

**Tech Stack:** Rust, Wasm-bindgen, Svelte, SVG

**Spec:** Roadmap items from `CLAUDE.md` (Asymptotes, sin(1/x), tan poles, domain holes, implicit f(x,y)=0).

## Global Constraints
- Rust `wasm-bindgen` FFI must remain performant with no unnecessary heap allocations on hot paths.
- Svelte-check must pass with zero errors.

## Review Focus
1. Division by zero near poles ($\tan(x)$, $1/x$) producing infinite loop or lockup.
2. Sign-change ambiguity across cell boundaries in marching squares.
3. NaN propagation on invalid domain ranges ($\sqrt{x}$ for $x < 0$).

---

### Task 1: Asymptote & Pole Detection in Sampling

**Files:**
- Modify: `engine/src/sampling.rs`
- Test: `engine/tests/asymptotes.rs`

**Interfaces:**
- Produces: `sample()` handling discontinuity breaks on large derivative spikes or sign flips across poles.

- [ ] **Step 1: Write failing test in `engine/tests/asymptotes.rs`**
- [ ] **Step 2: Run cargo test to verify failure**
- [ ] **Step 3: Implement pole/asymptote discontinuity check in `engine/src/sampling.rs`**
- [ ] **Step 4: Run cargo test to verify pass**
- [ ] **Step 5: Commit changes**
