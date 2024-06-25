use std::io::{self, BufRead};

/*
 * Complete the 'miniMaxSum' function below.
 *
 * The function accepts INTEGER_ARRAY arr as parameter.
 */

fn miniMaxSum(arr: &mut [i32]) {
    let mut arr = arr.as_mut();
    arr.sort();
    let n = arr.len();
    let mut min = 0;
    &arr[0..(n - 1)].iter().for_each(|&val| min += val as i64);

    let mut max = 0;
    &arr[(n - 4)..n].iter().for_each(|&val| max += val as i64);
    println!("{min} {max}");
}

#[test]
fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut arr: Vec<i32> = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    miniMaxSum(&mut arr);
}
