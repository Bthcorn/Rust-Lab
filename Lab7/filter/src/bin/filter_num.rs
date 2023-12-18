fn filter_numbers(x: &[f32]) -> Vec<f32> {
    let mut result = Vec::new();
    let mut iter = x.iter();
    while let Some(&x1) = iter.next() {
        if x1 >= -1. && x1 <= 1. {
            result.push(x1);
        }
    } result
}

#[test]
fn test_filter_numbers() {
    assert_eq!(filter_numbers(&[]), []);
    assert_eq!(filter_numbers(&[-2., -3., 0., 2., 3.]), [0.]);
    assert_eq!(filter_numbers(&[-1., 0., 1.]), [-1., 0., 1.]);
}