#![allow(unused_imports)]
use borsh::{from_slice, to_vec, BorshDeserialize, BorshSerialize};

#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug)]
struct A {
    x: u64,
    y: String,
}

#[test]
fn test_simple_struct() {
    let a = A {
        x: 3301,
        y: "liber primus".to_string(),
    };
    let encoded_a = to_vec(&a).unwrap();
    let decoded_a = from_slice::<A>(&encoded_a).unwrap();
    assert_eq!(a, decoded_a);
}
