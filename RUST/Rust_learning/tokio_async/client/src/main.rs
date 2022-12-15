use std::{
    net::TcpStream,
    io::{Read,Write, self},
    str
};
use serde::{Deserialize, Serialize};
use serde_json;
// use tokio::stream;

const SERVER_ADDR:&str = "127.0.0.1:8000";

#[derive(Clone,Serialize, Deserialize, Debug, PartialEq)]
pub enum MsgType {
    INIT,
    CONNECT,
    DATA
}

#[derive(Clone,Serialize, Deserialize, Debug, PartialEq)]
pub struct Msg {
    pub msg_type: MsgType,
    pub init_params: Option<InitParam>,
    pub point:Option<Point>,
}

#[derive(Clone,Serialize, Deserialize, Debug, PartialEq)]
pub struct Point {
    x: f64,
    y: f64,
    packet_index: i16
}

#[derive(Clone,Serialize, Deserialize, Debug, PartialEq)]
pub struct InitParam {
    pub arena_width: i32,
    pub arena_height: i32,
}


fn main() {
    
    let mut buf= [0; 1500];

    let mut stream = TcpStream::connect(SERVER_ADDR).expect("Could not connect to the server");

    let init_req = Msg {
        msg_type: MsgType::INIT,
        point: None,
        init_params :None
    };

    let serialized = serde_json::to_string(&init_req).unwrap();

    let _ = stream.write(serialized.as_bytes());
    let _ = stream.flush();

    let len = stream.read(&mut buf).unwrap();
    let received  = str::from_utf8(&buf[..len]).unwrap();
    let init_res:Msg = serde_json::from_str(&received).unwrap();
    
    println!("Connection initialized at port: {} and recieved the initial response {:#?}", 
    stream.local_addr().unwrap().port(),
    init_res);

    loop{

        let mut input = String::new();

        println!("Press A to send the request");

        io::stdin().read_line(&mut input).unwrap();

        if input.trim() == String::from("a") {
            let init_req = Msg {
                msg_type: MsgType::DATA,
                point: None,
                init_params :None
            };
        
            let serialized = serde_json::to_string(&init_req).unwrap();
            let _ = stream.write(serialized.as_bytes());
            let _ = stream.flush();

            input.clear();
        }

        let len = stream.read(&mut buf).unwrap();
        let received  = str::from_utf8(&buf[..len]).unwrap();
        let rec_msg:Msg = serde_json::from_str(&received).unwrap();

        println!("{:#?}", rec_msg);
    }
}



