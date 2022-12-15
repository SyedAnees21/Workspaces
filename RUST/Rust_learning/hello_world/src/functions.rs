pub fn run(){
    greeting("hello", "Anees");

    //Bind funtion values to variables
    let get_sum = add(5,5);
    println!("{}", get_sum);

    //Closure
    let n3: i32 = 10;
    let add_nums = |n1: i32, n2: i32| n1 + n2 + n3;
    println!("Sum {}", add_nums(3, 3));
}

//Basic function
fn greeting(greet: &str, name: &str){
    println!("{} {}, nice to meet you", greet, name);
}

//Return function
fn add(n1: i32, n2: i32)-> i32 {
    n1+n2
}