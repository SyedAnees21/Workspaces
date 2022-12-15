pub fn run() {
    //-------Ownership-Rules-------
    // 1. Eah value in Rust has its variable that is called its owner
    // 2. There can only  be one owner at a time
    // 3. When the owner goes out of scope the value gets dropped

    //take example
    /*
    s1 is the pointer to memory location in heap 
       which holds the "Hello" binary. So what actually 
       s1 variable is holding is the pointer to the location,
       length of the memory required for the value and the size of the memory
     */ 
    // let s1 = String::from("Hello"); // s1 is actually the point to the "Hello" in the heap memory
    // let s2 = s1;  //s2 now has a copy of pointer to "Hello" which is contained in s1 

    //println!("String: {}", s1); this will now gives the array as rust wont allow two pointers at the same time
    //so the best way to copy the value into another variable is

    let s1 = String::from("Hello");
    let _s2 = s1.clone();
    println!("String: {}", s1);

}