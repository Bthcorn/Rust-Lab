fn quote_list(v: &[&str], c: char) -> Vec<String> {
    let mut result = Vec::new();
    for n in 0..v.len() {
        result.push(format!("{}{}{}", c, v[n], c));
    }
    result
}

#[test]
fn test_quotes() {
    // assert_eq!(quote("abcd", '*'), "*abcd*");
    assert_eq!(quote_list(&[""; 0], '*'), &[""; 0]);
    assert_eq!(quote_list(&["abcd", "xyz"], '*'), ["*abcd*", "*xyz*"]);
}

fn quote_list_re(v: &mut [&str], c: char) -> Vec<String> {
    // v.iter().map(|s| format!("{}{}{}", c, s, c)).collect()
    let mut result = Vec::new();
    if !v.is_empty() {
        result.push(format!("{}{}{}", c, v[0], c));
        result.extend(quote_list_re(&mut v[1..], c));
    }
    result
}

#[test]
fn test_quotes_re() {
    // assert_eq!(quote("abcd", '*'), "*abcd*");
    assert_eq!(quote_list_re(&mut[""; 0], '*'), &[""; 0]);
    assert_eq!(quote_list_re(&mut["abcd", "xyz"], '*'), ["*abcd*", "*xyz*"]);
    assert_eq!(quote_list_re(&mut["abcd", "xyz"], '"'), ["\"abcd\"", "\"xyz\""]);
}