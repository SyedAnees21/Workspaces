use serde::{Deserialize,Serialize};
use serde_json;
use std::str;

#[derive(Serialize,Deserialize,Debug)]
struct SeqData {
    x: (u8, u8),
    y: (u8, u8),
    z: (u8, u8),
}

#[derive(Deserialize,Debug)]
struct Data {
    x: (u8, u8),
    z: (u8, u8),
}

pub fn run() {
    let data = SeqData {
        x: (22, 1),
        y: (34, 2),
        z: (45, 3),
    };

    let json_buf = serde_json::to_string(&data).unwrap();

    let json_buf2:Data = serde_json::from_str(json_buf.as_str()).expect("could not parse");
    println!("recieved data: {:#?}", json_buf2);

    let seq_bytes_buf = assigning_seq(json_buf);
    println!("sequence: {:?}", seq_bytes_buf);

    let transmission_buf = serde_json::to_string(&seq_bytes_buf).unwrap();
    println!("final transmission buffer {}", transmission_buf);

    let string_from_bytes = unassigning_seq(seq_bytes_buf);
    println!("String recieved: {:?}", string_from_bytes);

    let response= validate_bytes(seq_bytes_buf);
    println!("Response recieved : {}",response);

}

fn assigning_seq(buf:String) -> [(u8,usize);25] {
    let buf_bytes = buf.as_bytes();
    let mut bytes_seq:[(u8,usize); 25]= Default::default();

    for i in 0..23 {
        bytes_seq[i] = (buf_bytes[i], i);  
    }
    bytes_seq
}

fn unassigning_seq(buf:[(u8, usize); 25]) -> String {

    let mut bytes_unseq:[u8; 25]= Default::default();

    for i in 0..23 {
        bytes_unseq[i] = buf[i].0;  
    }
    let recv_string: String = (str::from_utf8(&bytes_unseq).expect("msg: &str")).to_string();
    recv_string
}

fn validate_bytes(buf:[(u8, usize); 25]) -> String {
    let mut response= String::new();
    
    for i in 0..buf.len() {
        if (buf[i+1].1 - buf[i].1)== 1 {
            response= String::from("Recieved bytes are intact!");
        }else {
            response= String::from("byte number") + &i.to_string() + &String::from("is lost");
        }
    }
    response
}

