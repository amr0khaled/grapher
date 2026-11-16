# TODO — Grapher features

## Equation input & preview
- [ ] KaTeX live render fixes
  - [ ] `^` renders as superscript
  - [ ] implicit multiplication: juxtaposition = multiplication (no `*` required)
  - [ ] `*` renders as centered dot (·)
  - [ ] `/` renders as classic fraction (numerator over denominator)
  - [ ] `sqrt(x)` renders with radical shape
  - [ ] `infinity` renders ∞
  - [ ] `sum(...)` renders Σ with limits
  - [ ] `int(...)` / integral renders ∫
  - [ ] derivative notation renders d/dx
  - [ ] `[...]` / matrices render bracketed matrix
- [ ] Equation rows: more height (taller input rows)
- [ ] Equation rows: per-line evaluated value shown bottom-right when equation is unambiguous (e.g. `y = 5`)
- [ ] Symbol assignment: `m = 5` assigns value to `m`, usable everywhere
- [ ] Reactive dependent equations: `n = m/2` updates automatically when `m` changes
- [ ] Dependency ordering / cycle handling across lines
