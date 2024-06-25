use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'lonelyinteger' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts INTEGER_ARRAY a as parameter.
 */

fn lonelyinteger(arr: &[i32]) -> i32 {
    let max = arr.iter().max().unwrap() + 1;
    let mut freq_arr = vec![0; max as usize];
    // println!("{freq_arr:?}");

    arr.iter().for_each(|&item| {
        let item = item as usize;
        if freq_arr[item] == 0 {
            freq_arr[item] = 1;
        } else {
            freq_arr[item] = 2;
        }
    });

    // println!("2nd {freq_arr:?}");
    for (i, val) in freq_arr.iter().enumerate() {
        if *val == 1 {
            return i as i32;
        }
    }
    0
}

#[test]
fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    // let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let n = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .parse::<i32>()
        .unwrap();

    let a: Vec<i32> = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let result = lonelyinteger(&a);
    println!("{result}");
    // writeln!(&mut fptr, "{}", result).ok();
}
