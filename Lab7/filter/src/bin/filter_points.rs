fn filter_points(v: &[(f64, f64)]) -> Vec<(f64, f64)> {
    let mut result = Vec::new();

    for (x, y) in v {
        if (((x - 0.).powi(2) + (y - 0.).powi(2)).sqrt()) <= 1.0 {
            result.push((*x, *y))
        }
    }
    result
}

#[test]
fn test_filter_points() {
    let input1: Vec<(f64, f64)> = vec![];
    let result1 = filter_points(&input1);
    assert_eq!(result1, vec![]);

    let input2 = vec![(2.0, 2.0), (-2.0, -2.0), (0.0, 2.0)];
    let result2 = filter_points(&input2);
    assert_eq!(result2, vec![]);

    let input3 = vec![(1.0, 0.0), (0.0, 1.0), (0.0, 0.0), (-1.0, 0.0), (0.0, -1.0)];
    let result3 = filter_points(&input3);
    assert_eq!(
        result3,
        vec![(1.0, 0.0), (0.0, 1.0), (0.0, 0.0), (-1.0, 0.0), (0.0, -1.0)]
    );

    assert_eq!(
        filter_points(&[
            (-2.0, -2.0),
            (-1.5, -1.5),
            (-1.0, -1.0),
            (-0.5, -0.5),
            (0.0, 0.0),
            (0.5, 0.5),
            (1.0, 1.0),
            (1.5, 1.5),
            (2.0, 2.0)
        ]),
        vec![(-0.5, -0.5), (0.0, 0.0), (0.5, 0.5)]
    );
}
