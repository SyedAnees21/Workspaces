use serde::{Serialize,Deserialize};
use serde_json;

#[derive(Serialize,Deserialize,Debug)]
struct Data {
    d1: String,
    d2: u8,
    d3: bool,
}


pub fn run() {

    let d = Data {
        d1: String::from("Anees"),
        d2: 35,
        d3: true
    };

    let json_str = serde_json::to_string(&d).unwrap();
    println!("Serialized structure for udp: {}", json_str);

    let deserial:Data = serde_json::from_str(json_str.as_str()).unwrap();
    println!("Deserialized data: {:?\n}", deserial);

}