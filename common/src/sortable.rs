use ethereum_types::U256;

fn sort(array: &[U256]) -> Vec<U256> {
    let mut array = array.to_vec();
    array.sort();
    array
}

#[test]
fn already_sorted() {
    let numbers = vec![U256::from(1i128), U256::from(2i128), U256::from(3i128)];
    let result = sort(&numbers);
    assert_eq!(result, numbers);
}

#[test]
fn unsorted() {
    let numbers = vec![U256::from(2i128), U256::from(1i128), U256::from(3i128)];
    let result = sort(&numbers);
    let expected = vec![U256::from(1i128), U256::from(2i128), U256::from(3i128)];
    assert_eq!(result, expected);
}
