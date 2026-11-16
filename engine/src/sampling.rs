use crate::parser::parse_expr;

pub(crate) fn sample(
    expr: &str,
    scale: f64,
    offset_x: f64,
    offset_y: f64,
    width: u32,
    height: u32,
) -> Vec<f64> {
    if let Some(idx) = expr.find('=') {
        let lhs = expr[..idx].trim().to_lowercase();
        let rhs = expr[idx + 1..].trim();
        let rhs_tree = match parse_expr(rhs) {
            Ok(t) => t,
            Err(_) => return Vec::new(),
        };
        let lhs_tree = match parse_expr(&lhs) {
            Ok(t) => t,
            Err(_) => return Vec::new(),
        };
        let f = |x: f64, y: f64| rhs_tree.eval(x, y) - lhs_tree.eval(x, y);

        // Marching squares / zero-crossing grid sampling for implicit f(x,y)=0
        let step = 4;
        let mut out: Vec<f64> = Vec::new();
        for py in (0..height as usize).step_by(step) {
            for px in (0..width as usize).step_by(step) {
                let x = (px as f64 - offset_x) / scale;
                let y = (offset_y - py as f64) / scale;
                let v = f(x, y);
                if v.is_finite() && v.abs() < 0.2 {
                    out.push(px as f64);
                    out.push(py as f64);
                } else {
                    out.push(f64::NAN);
                    out.push(f64::NAN);
                }
            }
        }
        return out;
    }

    let mut clean = expr;
    let mut is_x_eq = false;

    if clean.starts_with("x=") || clean.starts_with("X=") {
        is_x_eq = true;
        clean = &clean[2..];
    } else if clean.starts_with("y=") || clean.starts_with("Y=") {
        clean = &clean[2..];
    }

    let tree = match parse_expr(clean) {
        Ok(t) => t,
        Err(_) => return Vec::new(),
    };

    let width_f = width as f64;
    let height_f = height as f64;
    let mut out: Vec<f64> = Vec::new();

    if is_x_eq {
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
        let mut px = 0.0_f64;
        let mut prev: Option<(f64, f64)> = None;
        while px <= width_f {
            let x = (px - offset_x) / scale;
            let y = tree.eval(x, 0.0);
            if y.is_finite() {
                let is_pole = match prev {
                    Some((xp, yp)) if yp * y < 0.0 => {
                        let ym = tree.eval((xp + x) * 0.5, 0.0);
                        !ym.is_finite() || ym.abs() >= yp.abs().max(y.abs())
                    }
                    _ => false,
                };
                if is_pole {
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
