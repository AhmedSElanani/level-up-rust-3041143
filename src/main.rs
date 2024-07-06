extern crate linked_hash_set;

use linked_hash_set::LinkedHashSet;
use std::cmp::Eq;

fn unique<T>(input_list: &mut Vec<T>)
where
    T: Clone + Eq + std::hash::Hash,
{
    if input_list.is_empty() {
        return;
    }

    let mut set = LinkedHashSet::new();
    input_list.retain(|item| set.insert(item.clone()));
}

fn main() {
    let mut input = vec![1, 2, 2, 5];
    unique(&mut input);

    println!("The answer of unique(vec![1, 2, 2, 5]) = {:?}", input);
}

#[test]
fn empty_list() {
    let mut input: Vec<i32> = vec![];
    unique(&mut input);

    assert_eq!(input, vec![]);
}

#[test]
fn sorted_list() {
    let mut input = vec![1, 2, 3, 3, 4, 5, 5];
    unique(&mut input);

    assert_eq!(input, vec![1, 2, 3, 4, 5]);
}

#[test]
fn unsorted_list() {
    let mut input = vec![61, 22, 33, 33, 54, 5, 54];
    unique(&mut input);

    assert_eq!(input, vec![61, 22, 33, 54, 5]);
}

#[test]
fn i64_list() {
    let mut input: Vec<i64> = vec![
        11345644356,
        454435453567426,
        32434356764756554,
        454435453567426,
    ];
    unique(&mut input);

    assert_eq!(
        input,
        vec![11345644356, 454435453567426, 32434356764756554,]
    );
}
