use std::env;
use std::fmt::Binary;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'timeConversion' function below.
 *
 * The function is expected to return a STRING.
 * The function accepts STRING s as parameter.
 */

fn timeConversion(s: &str) -> String {
    let mut military_time = String::new();
    let n = s.len();

    if s.contains("PM") {
        let hr = &s[0..2].parse::<u32>().unwrap();
        if *hr == 12 {
            military_time.insert_str(0, &s[0..(n - 2)]);
        } else {
            let hr = hr + 12;
            let new_hr = format!("{hr}");
            military_time.insert_str(0, &new_hr);
            military_time.insert_str(2, &s[2..(n - 2)]);
        };
    } else {
        let hr = &s[0..2].parse::<u32>().unwrap();
        if *hr == 12 {
            military_time.insert_str(0, "00");
            military_time.insert_str(2, &s[2..(n - 2)]);
        } else {
            military_time.insert_str(0, &s[0..(n - 2)]);
        }
    }

    military_time
}

#[test]
fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create("./src/solutions/p3.txt").unwrap();

    let s = stdin_iterator.next().unwrap().unwrap();

    let result = timeConversion(&s);
    println!("time: {:?}", result);
    writeln!(&mut fptr, "{}", result).ok();
}
