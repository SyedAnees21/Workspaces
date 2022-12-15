pub fn run(){
    let hello = String::from("Hello");  //or simply let hello = "hello"

    println!("{}", hello);

    //get string length
    println!("{}", hello.len());

    //mutable string (strings which can be updated)
    let mut abc = String::from("aha");
    abc.push('a');
    println!("{}", abc);
    abc.push_str("huh!");
    println!("{}", abc);

    //checking how much of bytes a string certain string can store (in bytes)
    println!("capacity {}", abc.capacity());
    println!("isEmpty {}", abc.is_empty());
    println!("contains a?  {}", abc.contains('a'));
    println!("contains d?  {}", abc.contains('d'));
    println!("Replace a with d{}", abc.replace("a", "d"));

}