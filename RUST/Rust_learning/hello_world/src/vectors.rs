//vectors are resizeable arrays

use std::mem;

pub fn run(){

    let mut numbers: Vec<i32> = vec![1,2,3,4,5];
    println!("{:?}", numbers);

    //Reassigning values to arrays
    numbers[2] = 45;
    println!("Reassigned value at index 2: {:?}", numbers);

    //getting single element 
    println!("Single value: {}", numbers[0]);

    //get vecotr length and amount of memory they takes (vectors are stacked allocated)
    //std is standard libarary and mem is a class of it
    println!("Vecotr length: {}", numbers.len());

    // println!("Vecotr occupies {} bytes", std::mem::size_of_val(&numbers));
    println!("Vecotr occupies {} bytes", mem::size_of_val(&numbers));

    //get slice of the arrray
    let slice: &[i32] = &numbers;
    println!("Slice: {:?}", slice);

    //selective slicing 
    let slice: &[i32] = &numbers[0..2];
    println!("Selective Slice: {:?}", slice);

    //add on to vecotrs
    numbers.push(10);
    println!("{:?}", numbers);

    //pop the last value from vectors
    numbers.pop();
    println!("{:?}", numbers);

    //loop through vectors values
    for x in numbers.iter() {
        println!("Numbers: {}", x);
    }

    //loop through and mutate each value
    for x in numbers.iter_mut() {
        *x *= 2;
    }

    println!("mutated vector: {:?}", numbers);
    
}

