pub fn run(){
    //default is i32
    let x = 1;

    //default is f64
    let y = 2.5;

    //add explicit type 
    let z: i64 = 250000000000;

    //finding the max size
    println!("Max i32: {}", std::i32::MAX);
    println!("Max i64: {}", std::i64::MAX);

    //Boolean
    let is_active = true;
    println!("{:?}", (x,y,z,is_active));

    //char type
    let a1 = 'a';
    let face = '\u{1f600}';
    println!("{:?}", (x,y,z,is_active,a1,face));

}