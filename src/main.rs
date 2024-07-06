fn info<T: std::fmt::Debug + std::fmt::Display>(text: &T) {
    println!("Display formatting output: {}", text);
    println!("Debug formatting output:  {:?}", text);
    println!("Pretty-printed, debug formatting output: {:#?}", text);

    println!(); // just for a line break
}

fn main() {
    let a = "&str";
    let b = "string".to_string();

    info(&a);
    info(&b);
}
