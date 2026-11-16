mod tests {
    use grapher_engine::*;

    #[test]
    fn test_reciprocal_pole_breaks_path() {
        // 1/x has a pole at x = 0; at scale 5 (x range +-80) the sample
        // grid straddles the pole with both neighboring y values finite,
        // so the renderer must insert a NaN break instead of drawing
        // a straight segment through the asymptote.
        let result = sample("1/x", 5.0, 400.0, 300.0, 800, 600);
        assert!(
            result.chunks(2).any(|c| c[0].is_nan()),
            "pole at x=0 must produce a discontinuity break"
        );
    }
}
