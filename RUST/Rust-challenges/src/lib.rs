
pub fn common_factors_count(a: u32, b: u32) -> u32 {
    let mut count: u32 = 0;
    let smaller;

    if a > 0 && b > 0{
        if a < b {
            smaller = a;
        }else {
            smaller = b;
        }
        
        for i in 1..(smaller+1) {
            if (a % i == 0) && (b % i == 0) {
                count += 1;
            } 
        }
    }
    count
}

pub fn move_zeros (num_array: &mut [u8]) -> Vec<u8>{
    let len = num_array.len();
    let mut vec = Vec::from(num_array);

    if vec.contains(&0) && !vec.is_empty() {
            for i in 0..len {
            if let Some(num) = vec.get(i) {
                if *num == 0 {
                    let temp = vec.remove(i);
                    vec.push(temp);
                }
            }
        }
    }
    vec
}

pub fn longest_common_subsequence(seq_1: &str, seq_2: &str) {
    
    let char_vector_1: Vec<char> = seq_1.chars().collect();
    let char_vector_2: Vec<char> = seq_2.chars().collect();

    println!("{:?}",char_vector_1);
    println!("{:?}",char_vector_2);
}

pub fn max_mirror(int_array: Vec<u8>) -> (usize, Vec<u8>){
    let mut pattern = Vec::new();
    let mut min_pattern_len: usize = 2;
    let max_pattern_len = int_array.len()-1;

    let mut is_pattern = false;
    
    let mut mirror_pattern = Vec::new();
    let mut mirror_len = 0;

    // if int_array.len()%2 == 0 {
    //     max_pattern_len = int_array.len()/2;
    // }else {
    //     max_pattern_len = int_array.len()/2 +1;
    // }

    for _ in 0..int_array.len()-1 {
        if min_pattern_len <= max_pattern_len {
            for i in 0..min_pattern_len {
                if let Some(value) = int_array.get(i) {
                    pattern.push(*value);
                }
            }
        }
    
        let mut array_len = int_array.len()-1;

        for _ in 0..int_array.len()-1 {
            for (index, value) in pattern.iter().enumerate() {
                // if value == int_array.get(array_len-index).unwrap() {
                //     is_pattern = true;
                // }else {
                //     is_pattern = false;
                // }
            }
            if is_pattern {
                mirror_pattern = pattern.clone();
                mirror_len = pattern.len();
            }
            array_len -= 1;
            if array_len < min_pattern_len {
                break;
            }
        }
        min_pattern_len += 1;
        pattern.clear();
    }

    (mirror_len, mirror_pattern)
}

pub fn median_of_2_arrays(num1:Vec<u8>, num2: Vec<u8>) {
    let mut merged_array = num1.clone();

    for i in 0..num2.len() {
        for (index, value) in num1.iter().enumerate() {
            if num2[i] <= *value {
                merged_array.insert(index, num2[i]);
            }
        } 
    }
    println!("Merged: {:?}", merged_array);

    let median;
    if merged_array.len()%2 == 0 {
        let median_index1 = ((merged_array.len() / 2) as f32  + 0.5) as usize;
        let median_index2 = ((merged_array.len() / 2) as f32  - 0.5) as usize;

        median = (merged_array[median_index1] + merged_array[median_index2]) as f32 /2.;
    }else {
        let median_index = merged_array.len()/2;
        median = merged_array[median_index] as f32
    }

    println!("Median: {median}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn common_factors_test() {
        println!("Case 1: a= 10, b= 5, output= 2");
        let num_of_factors = common_factors_count(10, 5);
        assert_eq!(num_of_factors, 2);

        println!("Case 2: a= 60, b= 12, output= 6");
        let num_of_factors = common_factors_count(60, 12);
        assert_eq!(num_of_factors, 6);

        println!("Case 3: a= 10, b= 100, output= 4");
        let num_of_factors = common_factors_count(10, 100);
        assert_eq!(num_of_factors, 4);

        println!("Case 4: a= 5, b= 0, output= 0");
        let num_of_factors = common_factors_count(5, 0);
        assert_eq!(num_of_factors, 0);

        println!("Case 5: a= 2, b= 3, output= 1");
        let num_of_factors = common_factors_count(2, 3);
        assert_eq!(num_of_factors, 1);
    }

    #[test]
    fn move_zeros_test() {
        let mut arr:[u8;7] = [1,0,1,2,0,1,3];
        let modified_arr = move_zeros(&mut arr);
        println!("{:?}",modified_arr);
    }

    #[test]
    fn lcs_test() {
        let seq_1 = "ABCBDAB";
        let seq_2 = "BDCABA";
        
        longest_common_subsequence(seq_1, seq_2);
        
    }

    #[test]
    fn mirror_pattern_test() {
        let int_array1: Vec<u8> = Vec::from([1, 2, 3, 8, 9, 3, 2, 1]);
        let int_array2: Vec<u8> = Vec::from([1, 2, 1, 4]);
        let int_array3: Vec<u8> = Vec::from([7, 1, 2, 9, 7, 2, 1]);

        let (mirror_len, mirror_pattern) = max_mirror(int_array1);
        println!("{} {:?}", mirror_len, mirror_pattern);

        let (mirror_len, mirror_pattern) = max_mirror(int_array2);
        println!("{} {:?}", mirror_len, mirror_pattern);

        let (mirror_len, mirror_pattern) = max_mirror(int_array3);
        println!("{} {:?}", mirror_len, mirror_pattern);
    }

    #[test]
    fn median_of_2_test() {
        let int_array1: Vec<u8> = Vec::from([1,3]);
        let int_array2: Vec<u8> = Vec::from([2]);

        median_of_2_arrays(int_array1, int_array2);

        let int_array1: Vec<u8> = Vec::from([1,2]);
        let int_array2: Vec<u8> = Vec::from([3,4]);

        median_of_2_arrays(int_array1, int_array2);
    }
}
