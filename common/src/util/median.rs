use super::sort;
use crate::Int;

/// get the median from an array of Int
pub fn median(array: &[Int]) -> Int {
    let len = array.len();
    let array = sort(array);
    let mid: usize = len / 2;
    if len % 2 == 1 {
        array[mid]
    } else {
        (array[mid - 1] + array[mid]) / 2
    }
}

#[test]
fn ideal_median() {
    let numbers = vec![Int::from(1_u128), Int::from(2_u128), Int::from(3_u128)];
    let result = median(&numbers);
    assert_eq!(result, Int::from(2_u128));
}

#[test]
fn even_length() {
    let numbers = vec![
        Int::from(2_u128),
        Int::from(3_u128),
        Int::from(5_u128),
        Int::from(9_u128),
    ];
    let result = median(&numbers);
    assert_eq!(result, Int::from(4_u128));
}
