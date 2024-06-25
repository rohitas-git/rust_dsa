use std::{
    cmp::Ordering,
    io::{self, BufRead},
};

/*
 * Complete the 'plusMinus' function below.
 *
 * The function accepts INTEGER_ARRAY arr as parameter.
 */

fn plus_minus(arr: &[i32]) {
    let mut positive = 0;
    let mut negative = 0;
    let mut zeros = 0;
    let total = arr.len();

    arr.iter().for_each(|val| match val.cmp(&0) {
        Ordering::Equal => zeros += 1,
        Ordering::Greater => positive += 1,
        Ordering::Less => negative += 1,
    });
    let r1 = positive as f32 / total as f32;
    let r2 = negative as f32 / total as f32;
    let r3 = zeros as f32 / total as f32;
    println!("{:.6}",r1);
    println!("{:.6}",r2);
    println!("{:.6}",r3);
}

#[test]
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

    plus_minus(&arr);
}
