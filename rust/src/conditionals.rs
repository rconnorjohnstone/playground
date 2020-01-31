pub fn run() {
    let age: u8 = 20;
    let check_id = true;

    let age_check: bool;
    age_check = age >= 21;

    if age_check && check_id {
        println!("You can drink");
    } else if !age_check && check_id {
        println!("Get out of here!");
    } else {
        println!("Lemme see your id...");
    }
}
