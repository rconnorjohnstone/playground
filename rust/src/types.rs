pub fn run() {
    // i32
    let x = 1;
    
    // f64
    let y = 2.125;

    // explicit
    let z: i64 = 1203981093547203425;

    // bool
    let is_active = true;
    let might_be = 1 > 2;

    // char
    let a1 = 'a';
    let face = '\u{1F600}';

    println!("{:?}", (x,y,z,is_active, might_be, a1, face));

}
