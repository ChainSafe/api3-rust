use ethereum_types::U256;

fn sort(array: &[U256]) -> Vec<U256> {
    let mut array = array.to_vec();
    array.sort();
    array
}

#[test]
fn already_sorted() {
    let numbers = vec![U256::from(1_i128), U256::from(2_i128), U256::from(3_i128)];
    let result = sort(&numbers);
    assert_eq!(result, numbers);
}

#[test]
fn unsorted() {
    let numbers = vec![U256::from(2_i128), U256::from(1_i128), U256::from(3_i128)];
    let result = sort(&numbers);
    let expected = vec![U256::from(1_i128), U256::from(2i128), U256::from(3_i128)];
    assert_eq!(result, expected);
}

#[test]
fn large_numbers() {
    let numbers = vec![
        U256::from(212837128371931812_i128),
        U256::from(81723812738912378812_i128),
        U256::from(51623219381273_i128),
    ];
    let result = sort(&numbers);
    let expected = vec![
        U256::from(51623219381273_i128),
        U256::from(212837128371931812_i128),
        U256::from(81723812738912378812_i128),
    ];
    assert_eq!(result, expected);
}
