enum movement {
    Up,
    Down,
    Left,
    Right
} 

fn move_avatar(m:movement){
    //Perform action depending upon info 
    match m {
        movement::Up => println!("UP"),
        movement::Down => println!("DOWN"),
        movement::Left => println!("left"),
        movement::Right => println!("RIGHT"),
    }
}

pub fn run() {

    let avatar1 = movement::Up;
    let avatar2 = movement::Down;
    let avatar3 = movement::Left;
    let avatar4 = movement::Right;
    
    move_avatar(avatar1);
    move_avatar(avatar2);
    move_avatar(avatar3);
    move_avatar(avatar4);
}