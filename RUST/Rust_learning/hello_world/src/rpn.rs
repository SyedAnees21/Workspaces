fn pn_to_rpn(pn_str: &str) -> &str {
    let len = pn_str.len();
    let chr_iter = pn_str.chars();
    let mut temp ;

    let mut rpn_str = String::from("");

    for _ in 0..len {
        if let Some(chr) = chr_iter.next() {
            if chr=='+' || chr=='-'{
                temp = chr;
            }
        }
    }

    rpn_str
}

pub fn run() {
    let pn_str = "2+3 - 2";
    let rpn_str = pn_to_rpn(pn_str);

    // 23+2-

    println!("{}", rpn_str);
}