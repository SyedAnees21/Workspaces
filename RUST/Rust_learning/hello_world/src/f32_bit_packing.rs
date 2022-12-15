const F32_PRECISION_BITS: u32 = 13;

const F32_R_SHIFT_POS: u32 = 32-F32_PRECISION_BITS;
const F32_L_SHIFT_POS: u32 = 32-F32_PRECISION_BITS;
const F32_PRECISION_BITS_OFFSET: u64 = F32_PRECISION_BITS as u64;
const F32_INDEX_BITS_OFFSET: u64 = 3*F32_PRECISION_BITS as u64;
const F32_UPPER_REM_BITS: u32 = 32-2*F32_PRECISION_BITS;
const F32_UPPER_REM_POS: u32 = F32_PRECISION_BITS+F32_UPPER_REM_BITS;
const F32_LOWER_REM_POS: u32 = 32-(F32_PRECISION_BITS-F32_UPPER_REM_BITS);
const F32_INDEX_POS: u32 = 3*F32_PRECISION_BITS -32;

pub fn compress_f32 () {
    let mut quaternion: Vec<f32>= Vec::new();

    quaternion.push(0.5);
    quaternion.push(0.4);
    quaternion.push(0.43);
    quaternion.push(0.63);

    let mut largest = 0.;
    let mut index = 0;
    
    for value in quaternion.clone().into_iter().enumerate() {        
        if value.1 > largest {
            largest = value.1;
            index = value.0;
        }
    }
    quaternion.remove(index);

    let mut double: u64 = 0;
    let mut i: u64 = 0;
    
    for component in quaternion.iter() {
        let mut temp: u32 = 0;
        let quat_comp_bytes = (*component).to_be_bytes();        
        temp = u32::from_be_bytes(quat_comp_bytes);
        temp = temp >> F32_R_SHIFT_POS;
        double |= (temp as u64) << i*F32_PRECISION_BITS_OFFSET;
        i += 1;
    }
    double |= (index as u64) << F32_INDEX_BITS_OFFSET;

    println!("{:b}", double);

    let quat = uncompress_quat(double);
    println!("\nreceived quaternion: {:?}", quat.iter()); 
}

fn uncompress_quat(double: u64) -> Vec<f32> {
    
    let mut upper_half: u32 = 0;
    let mut lower_half: u32 = 0;
    let mut quat_comp1: u32 = 0;
    let mut quat_comp2: u32 = 0;
    let mut quat_comp3: u32 = 0;
    let mut index: u32 = 0;

    let mut quat = Vec::new();

    upper_half |= (double as u32 ) >> 0;

    println!("\nUpper Half: ");

    let half_word = upper_half.to_be_bytes();
    for byte in half_word.iter() {
        print!("{:b}", byte);    
    }
    print!("\n");

    
    println!("\nLower Half: ");
    
    lower_half |= (double  >> 32) as u32;
    
    let lower_half_bytes = lower_half.to_be_bytes();
    for byte in lower_half_bytes.iter() {
        print!("{:b}", byte);    
    }
    print!("\n");
    
    quat_comp1 |= upper_half << F32_L_SHIFT_POS;

    upper_half &= !(quat_comp1 >> F32_R_SHIFT_POS );
    upper_half = upper_half >> F32_PRECISION_BITS_OFFSET;
    
    quat_comp2 |= upper_half << F32_L_SHIFT_POS;
    
    upper_half &= !(quat_comp1 >> F32_R_SHIFT_POS);
    upper_half = upper_half >> F32_PRECISION_BITS_OFFSET;
    println!("{:b}", upper_half);

    quat_comp3 |= lower_half << F32_LOWER_REM_POS | upper_half << F32_UPPER_REM_POS;
    
    lower_half = lower_half >> F32_INDEX_POS;

    index |= lower_half;
    println!("{index}");

    if index == 0 {
        let mut component_bytes = quat_comp1.to_be_bytes();
        let y = f32::from_be_bytes(component_bytes);

        component_bytes = quat_comp2.to_be_bytes();
        let z = f32::from_be_bytes(component_bytes);

        component_bytes = quat_comp3.to_be_bytes();
        let w = f32::from_be_bytes(component_bytes);

        let x;

        if y < 0. && z < 0. && w < 0. {
            let num = 1. - (y*y + z*z + w*w);
            x = -(num.sqrt());
        }else {
            let num = 1. - (w*w + y*y + z*z);
            x = num.sqrt();
        }
        
    
        quat.push(x);
        quat.push(y);
        quat.push(z);
        quat.push(w);
    }

    if index == 1 {
        let mut component_bytes = quat_comp1.to_be_bytes();
        let x = f32::from_be_bytes(component_bytes);

        component_bytes = quat_comp2.to_be_bytes();
        let z = f32::from_be_bytes(component_bytes);

        component_bytes = quat_comp3.to_be_bytes();
        let w = f32::from_be_bytes(component_bytes);

        let y;

        if x < 0. && z < 0. && w < 0. {
            let num = 1. - (x*x + z*z + w*w);
            y = -(num.sqrt());
        }else {
            let num = 1. - (x*x + z*z + w*w);
            y = num.sqrt();
        }
    
        quat.push(x);
        quat.push(y);
        quat.push(z);
        quat.push(w);
    }

    if index == 2 {
        let mut component_bytes = quat_comp1.to_be_bytes();
        let x = f32::from_be_bytes(component_bytes);

        component_bytes = quat_comp2.to_be_bytes();
        let y = f32::from_be_bytes(component_bytes);

        component_bytes = quat_comp3.to_be_bytes();
        let w = f32::from_be_bytes(component_bytes);

        let z;

        if x < 0. && y < 0. && w < 0. {
            let num = 1. - (x*x + y*y + w*w);
            z = -(num.sqrt());
        }else {
            let num = 1. - (x*x + y*y + w*w);
            z = num.sqrt();
        }
    
        quat.push(x);
        quat.push(y);
        quat.push(z);
        quat.push(w);
    }

    if index == 3 {
        let mut component_bytes = quat_comp1.to_be_bytes();
        let x = f32::from_be_bytes(component_bytes);

        component_bytes = quat_comp2.to_be_bytes();
        let y = f32::from_be_bytes(component_bytes);

        component_bytes = quat_comp3.to_be_bytes();
        let z = f32::from_be_bytes(component_bytes);
        
        let w;

        if x < 0. && z < 0. && y < 0. {
            let num = 1. - (x*x + z*z + y*y);
            w = -(num.sqrt());
        }else {
            let num = 1. - (x*x + z*z + y*y);
            w = num.sqrt();
        }
    
        quat.push(x);
        quat.push(y);
        quat.push(z);
        quat.push(w);
    }

    quat
}