#[allow(dead_code, unused_assignments)]
pub fn febonacci_finder_n(num: usize) -> i32{
    let mut current: i32= 1;
    let mut prev=0; 
    let mut next=0;

    if num == 0 {
        return 0;
    }else{
        for _ in 1..num {
            next = current + prev;
            prev = current;
            current = next;
        }
        current
    }
}