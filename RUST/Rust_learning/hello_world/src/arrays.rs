use std::mem;

pub fn run(){

    let mut numbers: [i32; 5] = [1,2,3,4,5];
    println!("{:?}", numbers);

    //Reassigning values to arrays
    numbers[2] = 45;
    println!("Reassigned value at index 2: {:?}", numbers);

    //getting single element 
    println!("Single value: {}", numbers[0]);

    //get array length and amount of memory they takes (arrays are stacked allocated)
    //std is standard libarary and mem is a class of it
    println!("Array length: {}", numbers.len());

    // println!("Array occupies {} bytes", std::mem::size_of_val(&numbers));
    println!("Array occupies {} bytes", mem::size_of_val(&numbers));

    //get slice of the arrray
    let slice: &[i32] = &numbers;
    println!("Slice: {:?}", slice);

    //selective slicing 
    let slice: &[i32] = &numbers[0..2];
    println!("Selective Slice: {:?}", slice);


}