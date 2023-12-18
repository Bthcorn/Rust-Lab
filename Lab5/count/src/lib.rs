fn count_digits (text: &str) -> usize  {
    let mut count = 0;

    for c in text.chars() {
        if c.is_digit(10) {
            count += 1;
        }
    }
    count
}

#[test]
fn test_digits_count1() {
assert_eq!(count_digits(""), 0);
assert_eq!(count_digits("abcd"), 0);
assert_eq!(count_digits("ab12xy5 7x83y5z"), 7);

}

fn count_digits_r(text: &str) -> usize {
    if text.is_empty() {
        return 0;
    }

    let first_t = text.chars().next().unwrap();
    let rest = &text[1..];

    let mut count = if first_t.is_digit(10) {
        1
    } else {
        0
    };
    count += count_digits_r(&text[1..]);
    count
}

#[test]
fn test_digits_count_r() {
assert_eq!(count_digits_r(""), 0);
assert_eq!(count_digits_r("abcd"), 0);
assert_eq!(count_digits_r("ab12xy5 7x83y5z"), 7);
}

fn count_digits_v2(text: &str) -> Vec<(String, usize)> {
    let mut count = 0;
    let mut result = Vec::new();
    // let half = text.split_whitespace();
    for a in text.split_whitespace() {
        count = 0;
        for c in a.chars() {
            if c.is_digit(10) {
                count += 1;
            }
        }
        result.push((a.to_string(), count));
    } result
}

#[test]
fn test_digits_count2() {
    assert_eq!(count_digits_v2(""), []);
    assert_eq!(
        count_digits_v2("ab12xy5 7x83y5z"),
        [
            ("ab12xy5".to_string(), 3), // '1', '2', '5'
            ("7x83y5z".to_string(), 4)  // '7', '8', '3', '5'
        ]
    );
}
