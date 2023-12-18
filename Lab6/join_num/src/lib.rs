fn join_numbers (v: &[i32], c: &str) -> String {
    let mut result = String::new();
    let mut iter = v.iter();
    let mut count = 0;
    let n = v.len();
    while let Some(x) = iter.next() {
        if n < 2 {
            result.push_str(&format!("{}", x))
        } else if count == n-1 {
            result.push_str(&format!("{}", x))
        } else {
            result.push_str(&format!("{}{}", x, c));
            count += 1
        }
    }
    result
    // for (index, &num) in v.iter().enumerate() {
    //     if index > 0 {
    //         result.push_str(c);
    //     } 
    //     result.push_str(&num.to_string());
    // }
    // result.to_string()
}

#[test]
fn test_join_numbers() {
    assert_eq!(join_numbers(&[], ","), "");
    assert_eq!(join_numbers(&[25], ","), "25");
    let patterns = [5, 10, -1, 2];
    assert_eq!(join_numbers(&patterns, ", "), "5, 10, -1, 2");
    assert_eq!(join_numbers(&patterns, ";;"), "5;;10;;-1;;2");
}