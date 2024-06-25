use std::cmp::Ordering;
use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'caesarCipher' function below.
 *
 * The function is expected to return a STRING.
 * The function accepts following parameters:
 *  1. STRING s
 *  2. INTEGER k
 */

fn caesarCipher(s: &str, k: i32) -> String {
    s.chars()
        .map(|c| {
            if c.is_ascii_alphabetic() {
                match k.cmp(&0) {
                    Ordering::Equal => c,
                    Ordering::Greater => shifted_char((k % 26) as u8, c),
                    Ordering::Less => shifted_char((26 - k.abs() % 26) as u8, c),
                }
            }
            else{
                c
            }
        })
        .collect()
}

fn shifted_char(shift: u8, c: char) -> char {
    let new = c as u8 + shift;
    let size = 26;

    if c.is_ascii_uppercase() {
        if new > 90 {
            (new - size) as char
        } else {
            new as char
        }
    } else if c.is_ascii_lowercase() && new > 122 {
        (new - size) as char
    } else {
        new as char
    }
}

#[test]
fn it_works() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let n = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .parse::<i32>()
        .unwrap();

    let s = stdin_iterator.next().unwrap().unwrap();

    let k = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim()1 +  8  +  23 =  
        .parse::<i32>()
        .unwrap();

    println!("Original text: {s}");
    let result = caesarCipher(&s, k);
    println!("Ceasar cipher : {result}");
}
