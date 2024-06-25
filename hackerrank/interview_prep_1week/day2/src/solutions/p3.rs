use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'countingSort' function below.
 *
 * The function is expected to return an INTEGER_ARRAY.
 * The function accepts INTEGER_ARRAY arr as parameter.
 */

fn countingSort(arr: &[i32]) -> Vec<i32> {
    // let max = arr.iter().max().unwrap()+1;
    let max = 100;
    let mut freq_arr = vec![0; max as usize];

    arr.iter().for_each(|&item| {
        let item = item as usize;
        if freq_arr[item] == 0 {
            freq_arr[item] = 1;
        } else {
            freq_arr[item] += 1;
        }
    });

    println!("max: {}", max);
    println!("len: {}", freq_arr.len());
    freq_arr
    // let mut result = Vec::new();

    // freq_arr.iter().enumerate().for_each(|(index,&count)| {
    //     for _i in 0..count{
    //         result.push(index as i32);
    //     }
    // });

    // result
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let n = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .parse::<i32>()
        .unwrap();

    let arr: Vec<i32> = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let result = countingSort(&arr);

    println!("sorted {:?}", result);
}
