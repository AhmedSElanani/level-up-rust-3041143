fn median(mut input_list: Vec<f32>) -> Option<f32> {
    if input_list.is_empty() {
        return None;
    }

    input_list.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let input_length = input_list.len();
    if (input_length % 2) != 0 {
        return Some(input_list[input_length / 2]);
    }

    let middle_elements = [
        input_list[(input_length - 1) / 2],
        input_list[input_length / 2],
    ];

    return Some(middle_elements.iter().sum::<f32>() / middle_elements.len() as f32);
}

fn main() {
    let answer: Option<f32> = median(vec![1.0, 2.0, 5.0]);

    println!("The answer of median([1.0, 2.0, 5.0]) = {:?}", answer);
}

