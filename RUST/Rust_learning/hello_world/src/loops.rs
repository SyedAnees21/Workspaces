pub fn run(){
    let mut count = 0;

    //infinite loop
    loop {
        count += 1;
        println!("Counting: {}", count);

        if count == 50 {
            break;
        }
    }

    //While_loop(FizzBuzz)
    while count <= 100 {

        if count % 3 == 0 {
            println!("Fizz");
        }else if count % 5 == 0 {
            println!("Buzz");
        }else{
            println!("{}", count);
        }

        count += 1;
    }

    //range for loop 
    for x in 0..100{
        if x % 3 == 0 {
            println!("Fizz");
        }else if x % 5 == 0 {
            println!("Buzz");
        }else{
            println!("{}", x);
        }
    }
    
}