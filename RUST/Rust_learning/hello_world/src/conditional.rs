pub fn run(){

    let age: u8 = 22;
    let check_id: bool = false; 
    let knows_person_age = true;

    if age >= 21 && check_id || knows_person_age{
        println!("Bartender: What would you like to drink?");
    }else if age < 21 && check_id{
        println!("Sorry you must leave");
    }else {
        println!("Bartender: I need to see your ID");
    }

    //short hand if condition
    let is_of_age = if age >=21 {true} else {false};
    println!("Is of age {}", is_of_age);
}