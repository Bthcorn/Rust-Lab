use rand::Rng;

pub fn gen_numbers<R: Rng>(rng: &mut R, n: i64) -> Vec<f64> {
    let mut result = Vec::new();
    for _i in 0..n {
        result.push(rng.gen_range(-10.0..=10.0))
    }
    result
}

#[test]
fn test_gen_numbers() {
    let mut rng = rand::thread_rng();

    let n1 = 0;
    let result1 = gen_numbers(&mut rng, n1);
    assert_eq!(result1.len(), 0);

    let n2 = 5;
    let result2 = gen_numbers(&mut rng, n2);
    assert_eq!(result2.len(), n2 as usize);
    for &x in &result2 {
        assert!(x >= -10.0 && x <= 10.0);
    }

    let n3 = 10;
    let result3 = gen_numbers(&mut rng, n3);
    assert_eq!(result3.len(), n3 as usize);
    for &x in &result3 {
        assert!(x >= -10.0 && x <= 10.0);
    }
}