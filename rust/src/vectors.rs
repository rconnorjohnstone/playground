use std::mem;

pub fn run() {
    let mut numbers: Vec<i32> = vec![1,2,3,4,5];

    println!("Our numbers are {:?}", numbers);

    // Get singleton
    println!("Our first number is {}", numbers[0]);

    // Stack stuff
    println!("This vector occupies {} bytes", mem::size_of_val(&numbers));

    // Add to vector
    numbers.push(9);
    
    // Remove
    numbers.pop();

    // Loop
    for x in numbers.iter() {
        println!("{}", x+1);
    }
    
    // Loop
    for x in numbers.iter_mut() {
        *x *= 2;
    }

    // Get slice
    let slice: &[i32] = &numbers[2..];
    println!("Slice: {:?}", slice);
}
