#![allow(dead_code)]
#![allow(unused_imports)]

use std::{
    collections::HashMap,
    io::{self, BufRead},
};

fn find_min_steps(a: &[u32], b: &[u32]) {
    let mut a = a.to_owned();
    let b = b.to_owned();

    let n = a.len();
    assert_eq!(n, b.len());
    let min_elem = a.clone().into_iter().min().expect("iterator is not empty");
    let mut final_min = 0;

    for min in (0..=min_elem).rev() {
        let mut ok = true;
        let mut ans = 0;
        for (i, elem) in a.iter().enumerate() {
            // min == *elem: checks if min equal to the current element
            // b[i] > 0 && min % b[i] == a[i] % b[i]: 
            //      checks if min can be obtained from a[i] by 
            //      multiplying or adding multiples of b[i].
            ok = ok && min == *elem || b[i] > 0 && min % b[i] == a[i] % b[i];
            if b[i] > 0 {
                ans += (*elem - min) / b[i];
                final_min = min;
            }
        }
        // only when valid min is found
        if ok {
            println!("min: {final_min}");
            println!("{ans}");
            return;
        }
    }
    println!("-1");
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

    let a: Vec<u32> = std_iterator
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

    let b: Vec<u32> = std_iterator
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

    find_min_steps(&a, &b);
}
