use rand::Rng;

fn gen_points<R: Rng>(rng: &mut R, n: i64) -> Vec<(f64, f64)> {
    let mut result = Vec::new();
    
    for _i in 0..n {
        let x = rng.gen_range(-1.0..=1.0);
        let y = rng.gen_range(-1.0..=1.0);
        result.push((x, y));
    }
    result
}

#[test]
fn test_gen_points() {
    let mut rng = rand::thread_rng();

    let n1 = 0;
    let result1 = gen_points(&mut rng, n1);
    assert_eq!(result1.len(), 0);

    let n2 = 5;
    let result2 = gen_points(&mut rng, n2);
    assert_eq!(result2.len(), n2 as usize);
    for &(x, y) in &result2 {
        assert!(x >= -1.0 && x <= 1.0);
        assert!(y >= -1.0 && y <= 1.0);
    }

    let n3 = 10;
    let result3 = gen_points(&mut rng, n3);
    assert_eq!(result3.len(), n3 as usize);
    for &(x, y) in &result3 {
        assert!(x >= -1.0 && x <= 1.0);
        assert!(y >= -1.0 && y <= 1.0);
    }
}