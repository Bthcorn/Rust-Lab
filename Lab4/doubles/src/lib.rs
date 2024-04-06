fn doubles(v: &mut [i64]) {
    // v.iter().map(|x| x * 2).collect()
    for n in 0..v.len() {
        v[n] *=2;
    }
}

#[test]
fn test_doubles() {
    let array: &mut [i64] = &mut [1, 2, -3, 4, -6, 7];
    doubles(array);
    assert_eq!(array, &[2, 4, -6, 8, -12, 14]);
}

fn doubles_re(v: &mut [i64]) -> Vec<i64> {
    if !v.is_empty() {
        v[0] *= 2;
        return doubles_re(&mut v[1..]);
    };
    v.to_vec()
}

#[test]
fn test_doubles_re() {
    let array: &mut [i64] = &mut [1, 2, -3, 4, -6, 7];
    doubles_re(array);
    assert_eq!(array, [2, 4, -6, 8, -12, 14]);
}