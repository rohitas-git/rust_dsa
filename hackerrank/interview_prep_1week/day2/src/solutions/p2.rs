use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'diagonalDifference' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts 2D_INTEGER_ARRAY arr as parameter.
 */

fn diagonalDifference(arr: &[Vec<i32>]) -> i32 {
    let n = arr.len();
    let mut left_diagonal_sum = 0;
    let mut right_diagonal_sum = 0;

    // left diagonal: (0,0), (1,1)..(n-1,n-1)
    for (i, v) in arr.iter().enumerate() {
        left_diagonal_sum += v[i];
    }

    // right diagonal: (0,n-1), (1,n-2)..(n-1,0)
    for i in 0..n {
        let pos = (i, n - 1- i);
        right_diagonal_sum += arr[pos.0][pos.1];
    }

    (right_diagonal_sum - left_diagonal_sum).abs()
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let n = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .parse::<i32>()
        .unwrap();

    let mut arr: Vec<Vec<i32>> = Vec::with_capacity(n as usize);

    for i in 0..n as usize {
        arr.push(Vec::with_capacity(n as usize));

        arr[i] = stdin_iterator
            .next()
            .unwrap()
            .unwrap()
            .trim_end()
            .split(' ')
            .map(|s| s.to_string().parse::<i32>().unwrap())
            .collect();
    }

    let result = diagonalDifference(&arr);

    writeln!(&mut fptr, "{}", result).ok();
}
