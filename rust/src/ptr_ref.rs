pub fn run() {
    // Primitive Array
    let arr1 = [1,2,3];
    let arr2 = arr1;
    println!("Array 1 -> {:?}", arr1);
    println!("Array 2 -> {:?}", arr2);
    
    // Non-Primitive Vector
    let vec1 = vec![1,2,3];
    let vec2 = &vec1;
    println!("Vector 1 -> {:?}", vec1);
    println!("Vector 2 -> {:?}", vec2);
}
