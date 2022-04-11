use crate::Int;

/// sort an array of Int
pub fn sort(array: &[Int]) -> Vec<Int> {
    let mut array = array.to_vec();
    array.sort();
    array
}

#[test]
fn already_sorted() {
    let numbers = vec![Int::from(1_u128), Int::from(2_u128), Int::from(3_u128)];
    let result = sort(&numbers);
    assert_eq!(result, numbers);
}

#[test]
fn unsorted() {
    let numbers = vec![Int::from(2_u128), Int::from(1_u128), Int::from(3_u128)];
    let result = sort(&numbers);
    let expected = vec![Int::from(1_u128), Int::from(2u128), Int::from(3_u128)];
    assert_eq!(result, expected);
}

#[test]
fn large_numbers() {
    let numbers = vec![
        Int::from(212837128371931812_u128),
        Int::from(u128::MAX),
        Int::from(51623219381273_u128),
    ];
    let result = sort(&numbers);
    let expected = vec![
        Int::from(51623219381273_u128),
        Int::from(212837128371931812_u128),
        Int::from(u128::MAX),
    ];
    assert_eq!(result, expected);
}
