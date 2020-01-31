pub fn run() {
    let args: Vec<String> = std::env::args().collect();
    let command = args[1].clone();

    println!("Command: {:?}", command);
}
