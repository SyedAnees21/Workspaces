use std::{
    // net::TcpListener,
    time::{Duration},
    str,
    // io::{Read, Write},s
    thread, sync::{Arc, RwLock}, collections::VecDeque
  };
use tokio::{net::TcpListener, io::{AsyncReadExt, AsyncWriteExt}, runtime::Runtime};

use serde::{Deserialize, Serialize};
use serde_json;


const ARENA_WIDTH:i32 = 600;
const ARENA_HEIGHT:i32 = 600;

#[derive(Clone,Serialize, Deserialize, Debug, PartialEq)]
pub enum MsgType {
    INIT,
    CONNECT,
    DATA
}

#[derive(Clone,Serialize, Deserialize, Debug, PartialEq)]
pub struct Msg {
    pub msg_type: MsgType,
    pub point:Option<Point>,
    pub init_params: Option<InitParam>,
}

#[derive(Clone,Serialize, Deserialize, Debug, PartialEq)]
pub struct InitParam {
    pub arena_width: i32,
    pub arena_height: i32,
}

#[derive(Clone,Serialize, Deserialize, Debug, PartialEq)]
pub struct Point {
    x: f64,
    y: f64,
    packet_index: i16
}

#[derive(Clone,Serialize, Deserialize, Debug, PartialEq)]
pub struct History {
    queue: VecDeque<Msg>,
}

fn main() {

    let connection_thread = thread::spawn(move || {
        let mut buffer = [1; 1000];
        let ref_point = Point { x: 1., y: 1., packet_index:1 };
        let origin_point = Point {x:0., y:0., packet_index:0};

        let mut history = History {
            queue: VecDeque::new(),
        };

        let init_value = Msg {
            msg_type: MsgType::DATA,
            point: Some(Point{ x: 0., y: 0., packet_index:0}),
            init_params: None
        };
        history.queue.push_back(init_value);

        let history_lock = Arc::new(RwLock::new(history));

        let ref_lock = Arc::new(RwLock::new(ref_point));
        let orig_lock = Arc::new(RwLock::new(origin_point));

        let tokio_runtime = Runtime::new().unwrap();

        tokio_runtime.block_on(async move {

            tokio::spawn(async move {

                let listener = TcpListener::bind("127.0.0.1:8000").await.expect("Could not bind");
                // let (tx, _) = broadcast::channel(10);
            
                loop{
                    // let tx = tx.clone();
                    // let mut rx = tx.subscribe();

                    let (stream, _) = listener.accept().await.unwrap();
                    let (mut tcp_reader, mut tcp_writer) = stream.into_split();
                
                    let len = tcp_reader.read(&mut buffer).await.unwrap();
                    let received = str::from_utf8(&buffer[..len]).unwrap();
                    let init_req:Msg = serde_json::from_str(received).unwrap();
        
                    if init_req.msg_type == MsgType::INIT {
        
                        println!("New conncction recieved with req: {:#?}", init_req);
        
                        let response_back = Msg {
                            msg_type: MsgType::CONNECT,
                            point: None,
                            init_params: Some(InitParam {
                                arena_width: ARENA_WIDTH,
                                arena_height: ARENA_HEIGHT
                            })
                        };
        
                        let serialized = serde_json::to_string(&response_back).unwrap();
                        let _ = tcp_writer.write(serialized.as_bytes()).await.unwrap();
                        // let _ = stream.flush();
        
                        thread::sleep(Duration::from_millis(300));
        
                        let c_ref_lock = Arc::clone(&ref_lock);
                        let c_orig_lock = Arc::clone(&orig_lock);
                        let c_history_lock = Arc::clone(&history_lock);
        
                        tokio::spawn( async move {
                            
                            let mut refer;
                            {
                                let r = c_ref_lock.read().unwrap();
                                refer =  r.clone();
                            }
        
                            let mut orig;
                            {
                                let r = c_orig_lock.read().unwrap();
                                orig = r.clone();
                            }
                            
                            loop{
                                let len = tcp_reader.read(&mut buffer).await.unwrap();
                                let received = str::from_utf8(&buffer[..len]).unwrap();
                                let request:Msg = serde_json::from_str(received).unwrap();

                                println!("{:?}", request);

                                match request.msg_type {
                                    MsgType::DATA => {
                                        let simulation = Msg {
                                            msg_type: MsgType::DATA,
                                            point:Some(update_movement(&mut orig, &mut refer)),
                                            init_params: None,
                                        };
                                        {
                                            let mut wr = c_history_lock.write().unwrap();
                                            let history = &mut  *wr;
                                            history.queue.push_back(simulation);
                                        }
                                    },
                                    _=> {}
                                };
        
                                let to_send;
                                {
                                    let r = c_history_lock.read().unwrap();
                                    let history = (*r).clone();

                                    let last = history.queue.len()-1;
                                    to_send = history.queue[last].clone();
                                }
                                println!("{:#?}",to_send);

                                let serialized = serde_json::to_string(&to_send).unwrap();
                                let _ = tcp_writer.write(serialized.as_bytes()).await.unwrap();
                                // let _ = stream.flush();
        
                                thread::sleep(Duration::from_millis(200));
                            }
                        });
                    }
                }
            });
            loop{}
        });
    });
    println!("thread handle {:?}", connection_thread);
    connection_thread.join().unwrap();
}

fn handle_connection(){

}

// fn main() {
//     let mut buffer = [1; 1000];
//     let ref_point = Point { x: 1., y: 1., packet_index:1 };
//     let origin_point = Point {x:0., y:0., packet_index:0};

//     let ref_lock = Arc::new(RwLock::new(ref_point));
//     let orig_lock = Arc::new(RwLock::new(origin_point));

//     let listener = TcpListener::bind("127.0.0.1:8000").expect("Could not bind");
    
//     loop{
//         let (mut stream, _) = listener.accept().unwrap();
    
//         let len = stream.read(&mut buffer).unwrap();
//         let received = str::from_utf8(&buffer[..len]).unwrap();
//         let init_req:Msg = serde_json::from_str(received).unwrap();

//         if init_req.msg_type == MsgType::INIT {

//             println!("New conncction recieved with req: {:#?}", init_req);

//             let response_back = Msg {
//                 msg_type: MsgType::CONNECT,
//                 point: None,
//                 init_params: Some(InitParam {
//                     arena_width: ARENA_WIDTH,
//                     arena_height: ARENA_HEIGHT
//                 })
//             };

//             let serialized = serde_json::to_string(&response_back).unwrap();
//             let _ = stream.write(serialized.as_bytes()).unwrap();
//             let _ = stream.flush();

//             thread::sleep(Duration::from_millis(300));

//             let c_ref_lock = Arc::clone(&ref_lock);
//             let c_orig_lock = Arc::clone(&orig_lock);

//             thread::spawn( move || {
                
//                 let mut refer;
//                 {
//                     let r = c_ref_lock.read().unwrap();
//                     refer =  r.clone();
//                 }

//                 let mut orig;
//                 {
//                     let r = c_orig_lock.read().unwrap();
//                     orig = r.clone();
//                 }
                
//                 loop{

//                     let simulation = Msg {
//                         msg_type: MsgType::DATA,
//                         point:Some(update_movement(&mut orig, &mut refer)),
//                         init_params: None,
//                     };

//                     let serialized = serde_json::to_string(&simulation).unwrap();
//                     let _ = stream.write(serialized.as_bytes()).unwrap();
//                     let _ = stream.flush();

//                     thread::sleep(Duration::from_millis(200));
//                 }
//             });

//             // loop{
                
//             //     let simulation = Msg {
//             //         msg_type: MsgType::DATA,
//             //         point:Some(update_movement(&mut origin_point, &mut ref_point)),
//             //         init_params: None,
//             //     };

//             //     let serialized = serde_json::to_string(&simulation).unwrap();
//             //     let _ = stream.write(serialized.as_bytes()).unwrap();
//             //     let _ = stream.flush();

//             //     thread::sleep(Duration::from_millis(200));
//             // }
//         }
//     }    
// }
// #[tokio::main]
// async fn main() {
//     let mut buffer = [1; 1000];
//     let ref_point = Point { x: 1., y: 1., packet_index:1 };
//     let origin_point = Point {x:0., y:0., packet_index:0};

//     let ref_lock = Arc::new(RwLock::new(ref_point));
//     let orig_lock = Arc::new(RwLock::new(origin_point));

//     let listener = TcpListener::bind("127.0.0.1:8000").await.expect("Could not bind");
    
//     loop{
//         let (mut stream, _) = listener.accept().await.unwrap();
//         // let (tcp_reader, tcp_writer) = stream.split();
    
//         let len = stream.read(&mut buffer).await.unwrap();
//         let received = str::from_utf8(&buffer[..len]).unwrap();
//         let init_req:Msg = serde_json::from_str(received).unwrap();

//         if init_req.msg_type == MsgType::INIT {

//             println!("New conncction recieved with req: {:#?}", init_req);

//             let response_back = Msg {
//                 msg_type: MsgType::CONNECT,
//                 point: None,
//                 init_params: Some(InitParam {
//                     arena_width: ARENA_WIDTH,
//                     arena_height: ARENA_HEIGHT
//                 })
//             };

//             let serialized = serde_json::to_string(&response_back).unwrap();
//             let _ = stream.write(serialized.as_bytes()).await.unwrap();
//             let _ = stream.flush();

//             thread::sleep(Duration::from_millis(300));

//             let c_ref_lock = Arc::clone(&ref_lock);
//             let c_orig_lock = Arc::clone(&orig_lock);

//             tokio::spawn( async move {
                
//                 let mut refer;
//                 {
//                     let r = c_ref_lock.read().unwrap();
//                     refer =  r.clone();
//                 }

//                 let mut orig;
//                 {
//                     let r = c_orig_lock.read().unwrap();
//                     orig = r.clone();
//                 }
                
//                 loop{

//                     let simulation = Msg {
//                         msg_type: MsgType::DATA,
//                         point:Some(update_movement(&mut orig, &mut refer)),
//                         init_params: None,
//                     };

//                     let serialized = serde_json::to_string(&simulation).unwrap();
//                     let _ = stream.write(serialized.as_bytes()).await.unwrap();
//                     let _ = stream.flush();

//                     thread::sleep(Duration::from_millis(200));
//                 }
//             });
//         }
//     }    
// }




fn update_movement( point: &mut Point,  v:&mut Point) -> Point {

    let win_size_w = 400.;
    let win_size_h = 400.;

    let ball_radius = 20.;
    let col_padding = 20.;

    point.x += v.x;
    point.y += v.y;
    point.packet_index += v.packet_index;

    let win_w_half = win_size_w / 2.;
    let win_h_half = win_size_h / 2.;

    if point.x <= (-win_w_half + ball_radius) {
        v.x *= -1.;
        point.x += col_padding;
    } else if point.x >= (win_w_half - ball_radius) {
        v.x *= -1.;
        point.x -= col_padding;
    }

    if point.y <= (-win_h_half + ball_radius) {
        v.y *= -1.;
        point.y += col_padding;
    } else if point.y >= (win_h_half - ball_radius) {
        v.y *= -1.;
        point.y -= col_padding;
    }
        
        
    let  w_point = Point {
        x: point.x,
        y: point.y,
        packet_index: point.packet_index
    };
    return w_point;    
}

// async fn main() {
//     let mut buffer = [1; 1000];
//     let ref_point = Point { x: 1., y: 1., packet_index:1 };
//     let origin_point = Point {x:0., y:0., packet_index:0};

//     let ref_lock = Arc::new(RwLock::new(ref_point));
//     let orig_lock = Arc::new(RwLock::new(origin_point));

//     let listener = TcpListener::bind("127.0.0.1:8000").await.expect("Could not bind");
    
//     loop{
//         let (mut stream, _) = listener.accept().await.unwrap();
//         // let (tcp_reader, tcp_writer) = stream.split();
    
//         let len = stream.read(&mut buffer).await.unwrap();
//         let received = str::from_utf8(&buffer[..len]).unwrap();
//         let init_req:Msg = serde_json::from_str(received).unwrap();

//         if init_req.msg_type == MsgType::INIT {

//             println!("New conncction recieved with req: {:#?}", init_req);

//             let response_back = Msg {
//                 msg_type: MsgType::CONNECT,
//                 point: None,
//                 init_params: Some(InitParam {
//                     arena_width: ARENA_WIDTH,
//                     arena_height: ARENA_HEIGHT
//                 })
//             };

//             let serialized = serde_json::to_string(&response_back).unwrap();
//             let _ = stream.write(serialized.as_bytes()).await.unwrap();
//             let _ = stream.flush();

//             thread::sleep(Duration::from_millis(300));

//             let c_ref_lock = Arc::clone(&ref_lock);
//             let c_orig_lock = Arc::clone(&orig_lock);

//             tokio::spawn( async move {
                
//                 let mut refer;
//                 {
//                     let r = c_ref_lock.read().unwrap();
//                     refer =  r.clone();
//                 }

//                 let mut orig;
//                 {
//                     let r = c_orig_lock.read().unwrap();
//                     orig = r.clone();
//                 }
                
//                 loop{

//                     let simulation = Msg {
//                         msg_type: MsgType::DATA,
//                         point:Some(update_movement(&mut orig, &mut refer)),
//                         init_params: None,
//                     };

//                     let serialized = serde_json::to_string(&simulation).unwrap();
//                     let _ = stream.write(serialized.as_bytes()).await.unwrap();
//                     let _ = stream.flush();

//                     thread::sleep(Duration::from_millis(200));
//                 }
//             });
//         }
//     }    
// }