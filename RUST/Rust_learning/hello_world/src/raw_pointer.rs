use std::ptr;
struct RawPointer {
    data: *mut u8,
    index: u32
}
pub fn run() {
    let x: [u8;10] = [0;10] ;

    let mut a = RawPointer {
        data: ptr::null_mut::<u8>(),
        index: 0
    };
 
    // a.data = &mut x[0];
    // a.data.


    unsafe{
        let new_ptr = a.data.add(0) ;
        *new_ptr = 2;
        println!("Adress to the data {:?} and data {:?}", a.data, *a.data);

    }
    println!("{:?}",x);

    // unsafe{
    //     let new_ptr = a.data.add(8) ;
    //     *new_ptr = 3;
    //     println!("Adress to the data {:?} and data {:?}", a.data, *a.data);

    // }
    // println!("{:?}",x);
}