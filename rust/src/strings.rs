pub fn run() {
    let hello = "hello";

    let mut better_hello = String::from("hello");
    better_hello.push(' ');
    better_hello.push('\u{1F600}');
    better_hello.push_str(" stranger!");

    println!("Capacity: {}", better_hello.capacity());
    println!("Contains 'stranger': {}", better_hello.contains("stranger"));
    for word in better_hello.split_whitespace() {
        println!("Word: {}", word);
    }
    assert_eq!(20, better_hello.len());
}
