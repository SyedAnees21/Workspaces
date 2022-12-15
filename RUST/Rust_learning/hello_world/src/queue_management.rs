use std::{time::{Duration, Instant}, collections::VecDeque};


pub fn run() {
    let mut data_queue:VecDeque<i32> = VecDeque::new();
    let mut queue_copy:VecDeque<i32> = VecDeque::new();

    let mut start = Instant::now();

        for i in 0..400 {
            data_queue.push_back(i);
            
            if queue_management(start,&mut data_queue) {
                println!("Queue Copy: {:?}", queue_copy.iter());
                queue_copy = data_queue.clone();
                start = Instant::now();
            }
            println!("Data Queue: {:?}", data_queue.iter());
        }  

        let res:usize = binary_search(397, &data_queue).unwrap();
        println!("{}", data_queue[res]);
        
}

fn queue_management(start:Instant, dataqueue:&mut VecDeque<i32>) -> bool {
    if dataqueue.len() >= 5 {
        dataqueue.pop_front();
    }

    if start.elapsed() >= Duration::from_millis(150) {
        return true;
    }else {
        return false;
    }
}

pub fn binary_search(k: i32, items: &VecDeque<i32>) -> Option<usize> {
    if items.is_empty() {
        return None;
    }
 
    let mut low: usize = 0;
    let mut high: usize = items.len() - 1;
 
    while low <= high {
        let middle = (high + low) / 2;
        if let Some(current) = items.get(middle) {
            if *current == k {
                return Some(middle);
            }
            if *current > k {
                if middle == 0 {
                    return None;
                }
                high = middle - 1
            }
            if *current < k {
                low = middle + 1
            }
        }
    }
    None
}