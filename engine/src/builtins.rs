//! Scalar math + waveform built-in dispatch.
//! Population: extend match arms; unknown name / wrong arity returns None (eval maps to NaN).
//! Signal transform builtins live in transforms.rs.

// Arity-checked scalar builtins for one or more f64 args.
// Returns None on wrong arity or undefined operation (ln of negative, etc.).
pub(crate) fn eval_scalar(name: &str, args: &[f64]) -> Option<f64> {
    match (name, args.len()) {
        // Trigonometric
        ("sin", 1) => Some(args[0].sin()),
        ("cos", 1) => Some(args[0].cos()),
        ("tan", 1) => Some(args[0].tan()),

        // Inverse Trigonometric
        ("asin", 1) => if args[0] < -1.0 || args[0] > 1.0 { None } else { Some(args[0].asin()) },
        ("acos", 1) => if args[0] < -1.0 || args[0] > 1.0 { None } else { Some(args[0].acos()) },
        ("atan", 1) => Some(args[0].atan()),
        ("atan2", 2) => Some(args[0].atan2(args[1])),

        // Hyperbolic
        ("sinh", 1) => Some(args[0].sinh()),
        ("cosh", 1) => Some(args[0].cosh()),
        ("tanh", 1) => Some(args[0].tanh()),
        ("asinh", 1) => Some(args[0].asinh()),
        ("acosh", 1) => if args[0] < 1.0 { None } else { Some(args[0].acosh()) },
        ("atanh", 1) => if args[0] <= -1.0 || args[0] >= 1.0 { None } else { Some(args[0].atanh()) },

        // Logarithmic & Exponential
        ("sqrt", 1) => if args[0] < 0.0 { None } else { Some(args[0].sqrt()) },
        ("cbrt", 1) => Some(args[0].cbrt()),
        ("ln", 1) => if args[0] <= 0.0 { None } else { Some(args[0].ln()) },
        ("log2", 1) => if args[0] <= 0.0 { None } else { Some(args[0].log2()) },
        ("log10", 1) => if args[0] <= 0.0 { None } else { Some(args[0].log10()) },
        ("log", 2) => if args[0] <= 0.0 || args[1] <= 0.0 || args[1] == 1.0 { None } else { Some(args[0].log(args[1])) },
        ("exp", 1) => Some(args[0].exp()),

        // Rounding, absolute, sign
        ("abs", 1) => Some(args[0].abs()),
        ("sign", 1) => Some(if args[0] > 0.0 { 1.0 } else if args[0] < 0.0 { -1.0 } else { 0.0 }),
        ("floor", 1) => Some(args[0].floor()),
        ("ceil", 1) => Some(args[0].ceil()),
        ("round", 1) => Some(args[0].round()),

        // Min, max, mod, hypot
        ("min", 2) => Some(args[0].min(args[1])),
        ("max", 2) => Some(args[0].max(args[1])),
        ("mod", 2) => if args[1] == 0.0 { None } else { Some(args[0].rem_euclid(args[1])) },
        ("hypot", 2) => Some(args[0].hypot(args[1])),

        // Signal waveform primitives
        ("heaviside", 1) => Some(if args[0] < 0.0 { 0.0 } else if args[0] > 0.0 { 1.0 } else { 0.5 }),
        ("step", 1) => Some(if args[0] < 0.0 { 0.0 } else { 1.0 }),
        ("rect", 1) => Some(if args[0].abs() <= 0.5 { 1.0 } else { 0.0 }),
        ("tri", 1) => {
            let a = 1.0 - args[0].abs();
            Some(if a < 0.0 { 0.0 } else { a })
        }
        ("sinc", 1) => Some(if args[0] == 0.0 { 1.0 } else { let pi_x = std::f64::consts::PI * args[0]; pi_x.sin() / pi_x }),
        ("imp", 1) => Some(if args[0] == 0.0 { 1.0 } else { 0.0 }),

        _ => None,
    }
}
