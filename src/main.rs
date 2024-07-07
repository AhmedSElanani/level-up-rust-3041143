// 1) Non-generic, simpler way
// fn sort_usernames(users: &mut Vec<&str>) {

fn sort_usernames<T: AsRef<str>>(users: &mut Vec<T>) {
    if users.is_empty() {
        return;
    }

    // 1) Non-generic, simpler way
    // users.sort_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()));

    // 2) less efficient version
    // users.sort_by(|a, b| a.as_ref().to_lowercase().cmp(&b.as_ref().to_lowercase()));

    users.sort_by_cached_key(|a| a.as_ref().to_lowercase());
}

fn main() {
    let mut users = vec!["Todd", "amy"];
    println!("Users before sorting: {:#?}", users);

    sort_usernames(&mut users);
    println!("Users after sorting: {:#?}", users);
}

#[test]
fn empty_list() {
    let mut input: Vec<&str> = vec![];
    sort_usernames(&mut input);

    assert_eq!(input, Vec::<&str>::new());
}

#[test]
fn sorted_list() {
    let mut input = vec!["Ahmed", "Elanani"];
    sort_usernames(&mut input);

    assert_eq!(input, vec!["Ahmed", "Elanani"]);
}

#[test]
fn unsorted_case_sensitive_list() {
    let mut input = vec!["John", "Felix", "Anthony", "Cena"];
    sort_usernames(&mut input);

    assert_eq!(input, vec!["Anthony", "Cena", "Felix", "John"]);
}

#[test]
fn unsorted_case_insensitive_list() {
    let mut input = vec![
        "sun-Baby",
        "tinky-Winky",
        "Dipsy",
        "Po",
        "laa-Laa",
        "Narrator",
    ];
    sort_usernames(&mut input);

    assert_eq!(
        input,
        vec![
            "Dipsy",
            "laa-Laa",
            "Narrator",
            "Po",
            "sun-Baby",
            "tinky-Winky",
        ]
    );
}
