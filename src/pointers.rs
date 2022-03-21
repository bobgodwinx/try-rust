// Reference pointers baisce points an object to a resource in memory
pub fn run() {
    println!("Weclome to pointers ");
    // Primitive Array 
    let arr_1 = [1, 2, 3];
    // refernce
    let arr_2 = arr_1;
    println!("arr_2 points to arr_1: {:?}", arr_2);

    // With non-premitives, if you assign another variable to a piece of data,
    // then the previous variable wioll no longer hold that value. It will loose
    // the value. You'll need to use a reference (&) to point to the resource 

    let vec_1 = vec!["Hello"];
    println!("Vector 1: {:?}", vec_1);
    //let vec_2 = vec_1;
    //error value borrowed after move 
    //println!("Vector 1 will print nothing: {:?}", vec_1); 
    // we have to point it with a reference.
    // what this does is that it creates a "REFERENCE" to the
    // same resource that `vec_1` is pointing to and makes
    // `vec_2` points to the same resource. 
    let vec_2 = &vec_1;
    println!("Vector 2: {:?}", vec_2);

}