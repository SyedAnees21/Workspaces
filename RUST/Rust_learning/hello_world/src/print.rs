pub fn run(){
    //print to console 
    println!("hello from the print rs file");

    //Basic formatting
    println!("{} is from {}", "Brad", "Mass");
    println!("number: {}", 1);

    //positional arguments
    println!("{0} is from {1} and {0} likes to {2}", "Brad", "mass", "code");

    //Named arguments
    println!("{name} likes to play {activity}", name="Brad", activity="football");
    
    //Placeholder traits
    println!("Binary: {:b} Hex: {:x} and octal: {:o}", 10, 10, 10);

    //Placeholder for debug traits
    println!("{:?}", (10, true, "Anees"));

    //Basic maths 
    println!("10 + 10 = {}", 10+10);
}