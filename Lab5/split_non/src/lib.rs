fn extract_non_negatives(v: &[f64]) -> Vec<f64> {
    let mut result: Vec<f64> = Vec::new();
    for n in 0..v.len() {
        if v[n] >= 0.0 {
            result.push(v[n])
        } 
    } result
}

#[test]
fn test_extract_non_negatives() {
assert_eq!(extract_non_negatives(&[]), []);
assert_eq!(
extract_non_negatives(&[0.8, -5.1, 1.6, -6.5, 10.5]),
    [0.8, 1.6, 10.5]
);
}

fn extract_non_negatives_r (v: &[f64]) -> Vec<f64> {
    let mut result: Vec<f64> = Vec::new();
    if !v.is_empty() {
        if v[0] >= 0.0 {
            result.push(v[0]);
        } 
    } else {
        return result
    }
    result.extend(extract_non_negatives_r(&v[1..]));
    result
} 

#[test]
fn test_extract_non_negatives2() {
assert_eq!(extract_non_negatives_r(&[]), []);
assert_eq!(
extract_non_negatives_r(&[0.8, -5.1, 1.6, -6.5, 10.5]),
    [0.8, 1.6, 10.5]
);
}

fn split_non_negatives (v: &[f64]) ->  (Vec<f64>, Vec<f64>) {
    let mut positives = Vec::new();
    let mut negatives = Vec::new();
    for n in 0..v.len() {
        if v[n] < 0.0 {
            negatives.push(v[n])
        } if v[n] >= 0.0 {
            positives.push(v[n])
        } if v.is_empty() {
            return (positives, negatives)
        }
    } return (positives, negatives)
}

#[test]
fn test_split_non_negatives() {
    assert_eq!(split_non_negatives(&[]), (vec![], vec![]));
    assert_eq!(
        split_non_negatives(&[0.8, -5.1, 1.6, -6.5, 10.5]),
        (
        vec![0.8, 1.6, 10.5],
        vec![-5.1, -6.5]
        )
    );
}