fn main() {
    // println!("Hello, world!");
    // calculate the mean and mode of a list of integers
    // used to practice with vectors and hash maps
    // Rust book Chapter 8

    use rand::Rng;
    use std::collections::HashMap;

    let mut rand_num = rand::thread_rng();

    // generate random list of integers
    let mut int_list: Vec<i32> = Vec::new();
    for _i in 0..100 {
        int_list.push(rand_num.gen_range(0..10))
    }

    // sort list 
    int_list.sort();
    // print list
    for i in &int_list {
        print!("{i}\n");
    }
    // println!("\n----");
    // calculate median - middle value
    let list_len = int_list.len();
    let even_odd = list_len % 2;
    if even_odd == 0 {
        let first_num = int_list[list_len/2 - 1];
        let second_num = int_list[list_len/2];
        let median = (first_num as f32 + second_num as f32)/2.0;
        println!("Median is {median}");
    } else {
        let median = int_list[list_len/2];
        println!("Median is {median}");
    }

    // calculate mode - value occurring most often
    // insert values into hash-map
    let mut int_map = HashMap::new();

    for num in int_list {
        let count = int_map.entry(num).or_insert(0);
        *count += 1;
    }

    println!("-----\n{int_map:?}");
    let mut high_count = 0;
    let mut high_key: Vec<i32> = Vec::new();
    for (key, value) in &int_map {
        if *value > high_count {
            high_count = *value;
            if high_key.len() > 0 {
                high_key = vec![*key];
            } else {
                high_key.push(*key);
            }
        } else if *value == high_count {
            high_key.push(*key);
        }
    } 
    println!("----\nMode(s): {high_key:?}");
}
