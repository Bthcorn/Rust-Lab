fn pack_number_tuples(list1: &[i32], list2: &[i32]) -> Vec<(i32, i32)> {
    let mut min_length = 0;
    let mut result = Vec::new();
    if list1.len() <= list2.len() {
        min_length = list1.len()
    } else if list2.len() < list1.len() {
        min_length = list2.len()
    }

    for i in 0..min_length {
        let list1_n = list1[i];
        let list2_n = list2[i];
        result.push((list1_n, list2_n));
    }

    for i in min_length..list1.len() {
        result.push((list1[i], 0));
    }

    for i in min_length..list2.len() {
        result.push((0, list2[i]));
    }

    result
}

// fn pack_number_tuples(n: &[i32], m: &[i32]) -> Vec<(i32, i32)> {
//     let nmax = n.len();
//     let mmax = m.len();
//     let mut max = 0;

//     if nmax > mmax {
//         max = nmax;
//     } else {
//         max = mmax;
//     }

//     if n.is_empty() && m.is_empty() {
//         return Vec::new();
//     }
//     else {
//         let mut result: Vec<(i32, i32)> = Vec::new();
//         for count in 0..max {
//             let n_value = if count < n.len() { n[count] } else { 0 };
//             let m_value = if count < m.len() { m[count] } else { 0 };
//             result.push((n_value, m_value));
//         }
//         result
//     }
// }

#[test]
fn test_pack_number_tuples() {
    assert_eq!(pack_number_tuples(&[], &[]), []);
    assert_eq!(pack_number_tuples(&[1], &[]), vec![(1, 0)]);
    assert_eq!(pack_number_tuples(&[], &[2, 3]), vec![(0, 2), (0, 3)]);
    assert_eq!(
        pack_number_tuples(&[5, 1, 4], &[2, 3]),
        vec![(5, 2), (1, 3), (4, 0)]
    ); assert_eq!(
        pack_number_tuples(&[5, 1, 4], &[2, 3, 7]),
        vec![(5, 2), (1, 3), (4, 7)]
    );
}

fn pack_number_tuples_s(num1: &[i32], num2: &[i32]) -> Vec<(i32, i32)> {
    let mut result = Vec::new();
    let mut a = num1.iter();
    let mut b = num2.iter();
    while let (Some(&a_val), Some(&b_val))  = (a.next(), b.next()) {
        result.push((a_val, b_val));
    }
    result
}

#[test]
fn test_pack_number_tuples_s() {
    assert_eq!(pack_number_tuples_s(&[], &[]), []);
    assert_eq!(pack_number_tuples_s(&[1], &[]), []);
    assert_eq!(pack_number_tuples_s(&[], &[2, 3]), []);
    assert_eq!(
    pack_number_tuples_s(&[5, 1, 4], &[2, 3]), [(5, 2), (1, 3)]
    );
}