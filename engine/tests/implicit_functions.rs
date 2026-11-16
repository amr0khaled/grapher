mod tests {
    use grapher_engine::*;

    #[test]
    fn test_implicit_function_sampling() {
        // x^2 + y^2 - 4 = 0 (circle radius 2)
        // scale 50, offset (400, 300): center px=400, radius 100 px
        let result = sample("x^2 + y^2 - 4 = 0", 50.0, 400.0, 300.0, 800, 600);

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
}
