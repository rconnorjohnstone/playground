pub fn run() {
    greeting("Hello", "Connor");

    let ten = add(3,7);
    println!("Ten is {}", ten);

    // Closure (has outer scope)
    let n3 = 4;
    let add_nums = |n1: i32, n2: i32| n1 + n2 + n3;
    println!("Closure sum: {}", add_nums(2,3))
}

fn greeting(greet: &str, name: &str) {
    println!("{}, {}", greet, name);
}

fn add(n1: i32, n2: i32) -> i32 {
    n1 + n2
}
