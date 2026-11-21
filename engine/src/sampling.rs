use crate::eval::SymbolTable;
use crate::parser::{parse_assignment, parse_expr};
use std::collections::HashSet;

fn strip_axis_prefix(s: &str) -> Option<(bool, &str)> {
    let lower = s.to_lowercase();
    let first = lower.chars().next()?;
    if first != 'x' && first != 'y' {
        return None;
    }
    let rest = lower[first.len_utf8()..].trim_start();
    if !rest.starts_with('=') {
        return None;
    }
    let eq_idx = s.find('=')?;
    Some((first == 'x', s[eq_idx + 1..].trim()))
}

pub fn build_symbols(lines: &[String]) -> SymbolTable {
    let mut symbols = SymbolTable::new();
    for line in lines {
        let trimmed = line.trim();
        if strip_axis_prefix(trimmed).is_some() {
            continue;
        }
        if !trimmed.contains('=') {
            continue;
        }
        if let Ok((name, node)) = parse_assignment(trimmed) {
            symbols.set_expr(name, node);
        }
    }
    symbols
}

pub fn sample(
    lines: Vec<String>,
    scale: f64,
    offset_x: f64,
    offset_y: f64,
    width: u32,
    height: u32,
) -> Vec<Vec<f64>> {
    let symbols = build_symbols(&lines);

    let mut all_paths = Vec::with_capacity(lines.len());

    for line in &lines {
        let trimmed = line.trim();
        let mut path_points: Vec<f64> = Vec::new();

        if trimmed.is_empty() {
            all_paths.push(path_points);
            continue;
        }

        let axis = strip_axis_prefix(trimmed);
        let is_assignment =
            axis.is_none() && trimmed.contains('=') && parse_assignment(trimmed).is_ok();
        if is_assignment {
            all_paths.push(path_points);
            continue;
        }

        if axis.is_none() && trimmed.contains('=') {
            let parts: Vec<&str> = trimmed.splitn(2, '=').collect();
            if parts.len() == 2 {
                if let (Ok(lhs), Ok(rhs)) =
                    (parse_expr(parts[0].trim()), parse_expr(parts[1].trim()))
                {
                    let step = 4_usize;
                    for py in (0..height as usize).step_by(step) {
                        for px in (0..width as usize).step_by(step) {
                            let x = (px as f64 - offset_x) / scale;
                            let y = (offset_y - py as f64) / scale;
                            let lhs_val =
                                lhs.eval_with_symbols_rec(x, y, &symbols, &mut HashSet::new());
                            let rhs_val =
                                rhs.eval_with_symbols_rec(x, y, &symbols, &mut HashSet::new());
                            let v = rhs_val - lhs_val;
                            if v.is_finite() && v.abs() < 0.2 {
                                path_points.extend([px as f64, py as f64]);
                            } else {
                                path_points.extend([f64::NAN, f64::NAN]);
                            }
                        }
                    }
                }
            }
            all_paths.push(path_points);
            continue;
        }

        let (is_x_eq, expr_to_parse) = match axis {
            Some((is_x, rhs)) => (is_x, rhs.to_string()),
            None => (false, trimmed.to_string()),
        };

        if let Ok(tree) = parse_expr(&expr_to_parse) {
            if is_x_eq {
                for py in 0..=height {
                    let y = (offset_y - py as f64) / scale;
                    let x = tree.eval_with_symbols_rec(f64::NAN, y, &symbols, &mut HashSet::new());
                    if x.is_finite() {
                        path_points.extend([offset_x + x * scale, py as f64]);
                    } else {
                        path_points.extend([f64::NAN, f64::NAN]);
                    }
                }
            } else {
                let mut prev: Option<f64> = None;
                for px in (0..=width).step_by(2) {
                    let x = (px as f64 - offset_x) / scale;
                    let y = tree.eval_with_symbols_rec(x, f64::NAN, &symbols, &mut HashSet::new());
                    if y.is_finite() {
                        if let Some(p) = prev {
                            if p.signum() != y.signum() && y.abs() > 1e6 {
                                path_points.extend([f64::NAN, f64::NAN, f64::NAN, f64::NAN]);
                            }
                        }
                        path_points.extend([px as f64, offset_y - y * scale]);
                        prev = Some(y);
                    } else {
                        path_points.extend([f64::NAN, f64::NAN]);
                        prev = None;
                    }
                }
            }
        }
        all_paths.push(path_points);
    }
    all_paths
}
