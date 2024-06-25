#![allow(dead_code)]
#![allow(unused_imports)]

use std::{
    collections::HashMap,
    io::{self, BufRead},
};

fn ans(arr: &[u32]) {
    let mut count_map: HashMap<u32, u32> = HashMap::new();
    let mut max = 1;
    let mut fav_count= 0;

    arr.iter().for_each(|val| {
        if let Some(value) = count_map.get_mut(val){
            *value+=1;
            if max < *value{
                max = *value;
            }
        }else{
            count_map.insert(*val, 1);
        }
    });

    for (_,freq) in count_map.iter(){
        if *freq == max {
            fav_count+=1;
        } 
    }
    println!("No. of favorite singers: {fav_count}");
}

#[test]
fn test_ans() {
    let stdin = io::stdin();
    let mut std_iterator = stdin.lock().lines();

    let num_entries = std_iterator
        .next()
        .expect("iterater is not finished")
        .expect("valid utf8 bytes")
        .parse::<u32>()
        .expect("able to parse u32");

    let singers_arr: Vec<u32> = std_iterator
        .next()
        .expect("iterater is not finished")
        .expect("valid utf8 bytes")
        .trim_end()
        .split(' ')
        .map(|s| {
            s.to_string()
                .parse::<u32>()
                .expect("string parsable into u32")
        })
        .collect();

    ans(&singers_arr);
}
