use crate::parser::parse_expr;

pub(crate) fn sample(
    expr: &str,
    scale: f64,
    offset_x: f64,
    offset_y: f64,
    width: u32,
    height: u32,
) -> Vec<f64> {
    let mut clean = expr;
    let mut is_x_eq = false;

    if let Some(idx) = clean.find('=') {
        let lhs = clean[..idx].trim().to_lowercase();
        let rhs = clean[idx + 1..].trim();
        if lhs == "x" {
            is_x_eq = true;
            clean = rhs;
        } else if lhs == "y" {
            clean = rhs;
        } else {
            // implicit or general expression like x^2 + y^2 = 4 -> treat as y = rhs - lhs or similar, or parse rhs
            clean = rhs;
        }
    }

    let tree = match parse_expr(clean) {
        Ok(t) => t,
        Err(_) => return Vec::new(),
    };

    let width_f = width as f64;
    let height_f = height as f64;
    let mut out: Vec<f64> = Vec::new();

    if is_x_eq {
        // Sample along py (screen y), computing x = f(y)
        let mut py = 0.0_f64;
        while py <= height_f {
            let y = (offset_y - py) / scale;
            let x = tree.eval(0.0, y);
            if x.is_finite() {
                let px = offset_x + x * scale;
                if px.is_finite() && px >= -width_f * 2.0 && px <= width_f * 3.0 {
                    out.push(px);
                    out.push(py);
                } else {
                    out.push(f64::NAN);
                    out.push(f64::NAN);
                }
            } else {
                out.push(f64::NAN);
                out.push(f64::NAN);
            }
            py += 1.0;
        }
    } else {
        // Sample along px (screen x), computing y = f(x)
        let mut px = 0.0_f64;
        // (x, y) of the previous finite sample, for pole detection
        let mut prev: Option<(f64, f64)> = None;
        while px <= width_f {
            let x = (px - offset_x) / scale;
            let y = tree.eval(x, 0.0);
            if y.is_finite() {
                // Vertical-asymptote heuristic: y flips sign between samples.
                // A zero crossing has a small midpoint; a pole blows up between them.
                let is_pole = match prev {
                    Some((xp, yp)) if yp * y < 0.0 => {
                        let ym = tree.eval((xp + x) * 0.5, 0.0);
                        !ym.is_finite() || ym.abs() >= yp.abs().max(y.abs())
                    }
                    _ => false,
                };
                if is_pole {
                    // Break the path: NaN pair to end the previous segment,
                    // NaN pair so the next finite point starts a new segment.
                    out.push(f64::NAN);
                    out.push(f64::NAN);
                    out.push(f64::NAN);
                    out.push(f64::NAN);
                    prev = Some((x, y));
                    px += 1.5;
                    continue;
                }
                let py = offset_y - y * scale;
                if py.is_finite() && py > -height_f * 4.0 && py < height_f * 5.0 {
                    out.push(px);
                    out.push(py);
                } else {
                    out.push(f64::NAN);
                    out.push(f64::NAN);
                }
                prev = Some((x, y));
            } else {
                out.push(f64::NAN);
                out.push(f64::NAN);
                prev = None;
            }
            px += 1.5;
        }
    }
    out
}
