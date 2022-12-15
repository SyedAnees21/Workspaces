// Reference pointers to point to some memory

pub fn run(){
    
    //Primitive array 
    let arr1 = [1,2,3];
    let arr2 = arr1;
    println!("Values: {:?}", (arr1, arr2));

    //With non-primitve values, Vectors are non-premitive, to do same thing with vectors as above
    //requires pointers to reference it to some other arrays
    let vec1 = vec![1,2,3];
    let vec2 = &vec1;
    println!("Vector Values: {:?}", (&vec1, vec2));


}