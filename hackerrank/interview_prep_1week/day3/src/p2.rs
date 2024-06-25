// https://www.hackerrank.com/challenges/one-week-preparation-kit-tower-breakers-1/problem?isFullScreen=true&h_l=interview&playlist_slugs%5B%5D=preparation-kits&playlist_slugs%5B%5D=one-week-preparation-kit&playlist_slugs%5B%5D=one-week-day-three

use std::io::{self, BufRead};

fn towerBreakers(num_towers: i32, height: i32) -> i32 {
    if height == 1 {
        // P1 loses
         2 
    } else if num_towers % 2 == 0 {
        // P2 always wins since he always matches P1 move.
         2 
    } else {
        // P1 will be first to remove the odd tower. 
        // P2's turn will have even towers and eventually P2 will lose
         1 
    }
}

#[test]
fn test_soln() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    // let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let t = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    for _ in 0..t {
        let first_multiple_input: Vec<String> = stdin_iterator.next().unwrap().unwrap()
            .split(' ')
            .map(|s| s.to_string())
            .collect();

        let n = first_multiple_input[0].trim().parse::<i32>().unwrap();

        let m = first_multiple_input[1].trim().parse::<i32>().unwrap();

        let result = towerBreakers(n, m);

    }
}