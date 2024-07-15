fn sum_with_missing(numbers: Vec<Option<i32>>) -> i32 {
    return numbers.iter().map(|x| x.unwrap_or_default()).sum();

    // My solution below
    // let mut sum: i32 = 0;

    // for num in numbers {
    //     sum += match num {
    //         Some(value) => value,
    //         None => 0,
    //     }
    // }

    // return sum;
}

fn main() {
    println!("");
}

// tests are from the course
#[test]
fn empty() {
    let nn = vec![];
    assert_eq!(sum_with_missing(nn), 0);
}

#[test]
fn no_missing() {
    let nn = vec![Some(1), Some(5), Some(4)];
    assert_eq!(sum_with_missing(nn), 10);
}

#[test]
fn some_missing() {
    let nn = vec![None, Some(1), Some(5), Some(4), None, None];
    assert_eq!(sum_with_missing(nn), 10);
}

#[test]
fn all_missing() {
    let nn = vec![None, None, None];
    assert_eq!(sum_with_missing(nn), 0);
}
