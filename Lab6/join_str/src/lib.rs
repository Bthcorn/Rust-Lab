fn join_strings(v: &[&str], c: &str) -> String {
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
    // for (index, &s) in v.iter().enumerate() {
    //     if index > 0 {
    //         result.push_str(c);
    //     }
    //     result.push_str(s);
    // }
    // result.to_string()
}

#[test]
fn test_join_strings() {
    assert_eq!(join_strings(&[], ","), "");
    assert_eq!(join_strings(&["C"], ","), "C");
    let patterns = ["C", "Rust", "C++", "Python"];
    assert_eq!(join_strings(&patterns, ", "), "C, Rust, C++, Python");
    assert_eq!(join_strings(&patterns, ";;"), "C;;Rust;;C++;;Python");
}
