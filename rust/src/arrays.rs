use std::mem;

pub fn run() {
    let numbers: [i32; 5] = [1,2,3,4,5];

    println!("Our numbers are {:?}", numbers);

    // Get singleton
    println!("Our first number is {}", numbers[0]);

    // Stack stuff
    println!("This array occupies {} bytes", mem::size_of_val(&numbers));

    // Get slice
    let slice: &[i32] = &numbers[2..];
    println!("Slice: {:?}", slice);
}
