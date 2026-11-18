mod tests {
    use grapher_engine::sampling::sample;

    #[test]
    fn test_implicit_function_sampling() {
        // x^2 + y^2 - 4 = 0 (circle radius 2)
        // scale 50, offset (400, 300): center px=400, radius 100 px
        let result = grapher_engine::sampling::sample(vec!["x^2 + y^2 - 4 = 0".to_string()], 50.0, 400.0, 300.0, 800, 600);
        let result = &result[0];

        let mut saw_ring_point = false;
        for c in result.chunks(2) {
            if !c[0].is_finite() || !c[1].is_finite() {
                continue;
            }
            let dx = c[0] - 400.0;
            let dy = c[1] - 300.0;
            let dist = (dx * dx + dy * dy).sqrt();
            // Points must lie on the circle (within a few pixels).
            // A flat-line degenerate sample fails this.
            assert!(
                (dist - 100.0).abs() < 5.0,
                "finite sample at dist {dist} from center, expected ring ~100"
            );
            saw_ring_point = true;
        }
        assert!(saw_ring_point, "no finite contour points found");
    }

    #[test]
    fn test_batch_aligns_indices() {
        let result = grapher_engine::sampling::sample(
            vec![
                "a = 3".to_string(),
                "y = a * x".to_string(),
                "".to_string(),
            ],
            50.0,
            400.0,
            300.0,
            800,
            600,
        );
        assert_eq!(result.len(), 3);
        assert!(result[0].is_empty(), "assignment line yields no path");
        assert!(!result[1].is_empty(), "graph line yields path points");
        assert!(result[2].is_empty(), "empty line yields no path");
    }

    #[test]
    fn test_batch_symbol_dependency() {
        // y = a * x with a = 3 should plot y = 3*x at scale 1, offset 0:
        // at px=100, x=100, y=300 → py = 0 - 300*1 = -300? No:
        // offset_x=0, offset_y=0, scale=1: py = -y = -3*x = -300 at px=100
        let result = grapher_engine::sampling::sample(
            vec!["a = 3".to_string(), "y = a * x".to_string()],
            1.0,
            0.0,
            0.0,
            100,
            100,
        );
        let path = &result[1];
        assert!(!path.is_empty());
        // Find finite pair at px=10 → world x=10 → world y=30 → screen py=0-30=-30
        for c in path.chunks(2) {
            if c[0] == 10.0 && c[1].is_finite() {
                assert_eq!(c[1], -30.0);
                return;
            }
        }
        panic!("no finite point at px=10; got path {:?}", path);
    }

    #[test]
    fn test_user_defined_function_sampling() {
        let result = grapher_engine::sampling::sample(
            vec![
                "g(x) = x^2".to_string(),
                "y = g(x)".to_string(),
            ],
            1.0,
            0.0,
            0.0,
            100,
            100,
        );
        assert_eq!(result.len(), 2);
        assert!(!result[1].is_empty());
    }
}
