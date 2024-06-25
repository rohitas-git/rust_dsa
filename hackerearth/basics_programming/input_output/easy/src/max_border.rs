/* ----------------------------- Maximum Border ----------------------------- */
/*
You are given a table with rows and columns.
Each cell is colored with white or black.
Considering the shapes created by black cells,

what is the maximum border of these shapes?

Border of a shape means the maximum number of consecutive black cells
in any row or column without any white cell in between.

A shape is a set of connected cells. Two cells are connected if they share an edge. Note that no shape has a hole in it.

Input format

- The first line contains denoting the number of test cases.
-The first line of each test case contains integers denoting the number of rows and columns of the matrix.
    Here, '#' represents a black cell and '.' represents a white cell.
- Each of the next n lines contains m integers.
*/
/* ------------------------------------ x ----------------------------------- */
use std::{
    cmp::max, collections::HashMap, io::{self, BufRead}
};

fn find_max_border(arr: &[Vec<u64>]) {
    let mut row_max_border = 0;
    let mut col_max_border = 0;

    let col_num = arr[0].len(); // number of columns
    let row_num = arr.len(); // number of rows

    for (i,row) in arr.iter().enumerate() {
        let mut row_border = 0;
        for (i, val) in row.iter().enumerate() {
            if val == &1 {
                row_border += 1;
            }
        }
        // println!("{i} row_border: {row_border}");
        if row_max_border < row_border {
            row_max_border = row_border;
        }
    }

    for i in 0..col_num {
        let mut col_border = 0;
        for j in 0..row_num {
            let v = arr[j][i];
            if v == 1 {
                col_border += 1;
            }
            if v == 0 {
                col_border = 0;
            }
        }
        // println!("{i} col_border: {col_border}");
        if col_max_border < col_border {
            col_max_border = col_border;
        }
    }

    let max_border = max(row_max_border, col_max_border);
    println!("{max_border}");
}

#[test]
fn test_ans() {
    let stdin = io::stdin();
    let mut std_iterator = stdin.lock().lines();

    let num_test_cases = std_iterator
        .next()
        .expect("iterater is not finished")
        .expect("valid utf8 bytes")
        .parse::<u32>()
        .expect("able to parse u32");

    for i in 0..num_test_cases {
        let row_col: Vec<u64> = std_iterator
            .next()
            .unwrap()
            .unwrap()
            .trim_end()
            .split(' ')
            .map(|s| s.to_string().parse::<u64>().unwrap())
            .collect();

        assert_eq!(row_col.len(), 2);

        let rows = row_col[0];
        let cols = row_col[1];
        let mut arr_2d = vec![];

        for row in 0..rows {
            let row_arr: Vec<u64> = std_iterator
                .next()
                .unwrap()
                .unwrap()
                .trim_end()
                .chars()
                .map(|c| {
                    if c == '.' {
                        0
                    } else if c == '#' {
                        1
                    } else {
                        2 // invalid
                    }
                })
                .collect();

            assert_eq!(row_arr.len(), cols as usize) ;
            // println!("{row}: {row_arr:?}");
            arr_2d.push(row_arr);
        }

        find_max_border(&arr_2d);
    }
}
