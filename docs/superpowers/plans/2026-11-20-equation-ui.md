# Equation Sampling and UI Improvements Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Optimize sampling by identifying drawable expressions, improve UI consistency, enhance variable evaluation, and provide dynamic error feedback.

**Architecture:** Update `ExpressionLine` classification in Rust/WASM, update Svelte components to handle UI state based on classification, and enhance symbol evaluation in `math.ts`.

**Tech Stack:** Svelte 5, Rust/WASM.

**Spec:** Project instructions and user request.

## Global Constraints

- No drawing for definitions (`x = 5`, `f(x) = ...`).
- Drawing for functions/equations (`y=...`, `f(x)=...`, `g(x)=...`).
- Color picker space preserved when not drawing.
- Evaluation badge only for determinable variables.
- Dynamic error messages.

## Review Focus

1. **Definition detection:** Does `x=5` draw? (Expect: No).
2. **Function detection:** Does `g(x)=x^2` draw? (Expect: Yes).
3. **Evaluation badge:** Does `y = x+1` show a badge? (Expect: No, `x` indeterminate).
4. **Error message:** Does undefined symbol `a` in `y=a+1` show specific "a undefined" error? (Expect: Yes).
5. **UI Consistency:** Does removing color picker from definition shift input field? (Expect: No).

---

### Task 1: Classify Expressions (Rust/WASM)

**Files:**
- Modify: `engine/src/lib.rs`
- Modify: `src/types.ts`
- Modify: `src/utils/math.ts`

**Interfaces:**
- Produces: `isDrawable(expr: string) -> boolean`

- [ ] **Step 1: Write failing test in `engine/src/lib.rs`**
- [ ] **Step 2: Run test, verify fail**
- [ ] **Step 3: Implement `isDrawable` logic**
- [ ] **Step 4: Run test, verify pass**
- [ ] **Step 5: Commit**

### Task 2: Update UI State (`EquationInput.svelte`)

**Files:**
- Modify: `src/components/EquationInput.svelte`

**Interfaces:**
- Consumes: `isDrawable` classification

- [ ] **Step 1: Implement conditional color picker visibility**
- [ ] **Step 2: Add interactivity (long press/double click color picker)**
- [ ] **Step 3: Commit**

### Task 3: Enhance Evaluation Badge (`math.ts` + `App.svelte`)

**Files:**
- Modify: `src/utils/math.ts`
- Modify: `src/components/EquationInput.svelte`

**Interfaces:**
- Produces: `getEvaluatedValue(expr: string, symbols: Record) -> number | null`

- [ ] **Step 1: Write test for determining if RHS is determinable**
- [ ] **Step 2: Implement badge logic**
- [ ] **Step 3: Commit**

### Task 4: Dynamic Error Messages

**Files:**
- Modify: `engine/src/lib.rs` (error propagation)
- Modify: `src/components/EquationInput.svelte`

- [ ] **Step 1: Update WASM error types**
- [ ] **Step 2: Update UI to display dynamic error**
- [ ] **Step 3: Commit**
