/* -------- Two Sum : Check if a pair with given sum exists in Array -------- */

// Assume that each input would have exactly one solution,
// and you may not use the same element twice.

use std::collections::HashMap;

// time - O(n*(n/2))
// {avg time of inner loop: n/2; from sequence of times: n-1,n-2,...0 }
// space - O(1)
fn brute_soln(arr: &[i32], target: i32) -> (usize, usize) {
    for (i, val) in arr.iter().enumerate() {
        for (j, val2) in arr.iter().enumerate().skip(i) {
            if val + val2 == target {
                return (i, j);
            }
        }
    }
    (0, 0)
}

// map: elements -> indexes
// then go over elements and find their other one
// time - O(n)
// space- O(n)
fn better_soln_v1(arr: &[i32], target: i32) -> (usize, usize) {
    let mut map = HashMap::new();

    for (i, val) in arr.iter().enumerate() {
        map.insert(val, i);
    }

    for (i, val) in arr.iter().enumerate() {
        let other_val = target - val;
        if map.contains_key(&other_val) {
            let j = map.get(&other_val).unwrap().to_owned();
            return (i, j);
        }
    }
    (0, 0)
}

// has a different perspective from above soln
// combines two iteration into one
// time - O(n)
// space- O(n)
fn better_soln_v2(arr: &[i32], target: i32) -> (usize, usize) {
    let mut map: HashMap<&i32, usize> = HashMap::new();

    for (i, val) in arr.iter().enumerate() {
        let other_val = target - val;
        if map.contains_key(&other_val) {
            let j = map.get(&other_val).unwrap().to_owned();
            return (i, j);
        }
        map.insert(val, i);
    }
    (0, 0)
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_better_soln1() {
        let arr = [2, 6, 5, 8, 11];
        let target = 7;
        assert_eq!(better_soln_v1(&arr, target), (0, 2))
    }
    
    #[test]
    fn test_better_sol2() {
        let arr = [2, 6, 5, 8, 11];
        let target = 7;
        assert_eq!(better_soln_v2(&arr, target), (2, 0))
    }
    
    #[test]
    fn test_brute_soln() {
        let arr = [2, 6, 5, 8, 11];
        let target = 7;
        assert_eq!(brute_soln(&arr, target), (0, 2));
    }
}
