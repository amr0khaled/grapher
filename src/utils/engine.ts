import init, { _eval, get_symbol_values, is_drawable, sample } from "../wasm/grapher_engine.js"


let fail = false
let ready = false
try {
  await init()
  ready = true
} catch (e) {
  console.error("WASM engine init failed", e);
  fail = true
}

export {
  _eval,
  get_symbol_values,
  is_drawable,
  sample,
  fail as failure,
  ready
}
