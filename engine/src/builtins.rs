//! Scalar math + waveform built-in dispatch.
//! Population: extend match arms; unknown name / wrong arity returns None (eval maps to NaN).
//! Signal transform builtins live in transforms.rs.

// Arity-checked scalar builtins for one or more f64 args.
// Returns None on wrong arity or undefined operation (ln of negative, etc.).
pub(crate) fn eval_scalar(name: &str, args: &[f64]) -> Option<f64> {
    match (name, args.len()) {
        ("sin", 1) => Some(args[0].sin()),
        ("cos", 1) => Some(args[0].cos()),
        ("tan", 1) => Some(args[0].tan()),
        ("sqrt", 1) => if args[0] < 0.0 { None } else { Some(args[0].sqrt()) },
        _ => None,
    }
}
