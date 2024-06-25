// https://leetcode.com/problems/merge-sorted-array/description/?envType=study-plan-v2&envId=top-interview-150

/* --------------------------------- Problem -------------------------------- */
// You are given two integer arrays nums1 and nums2, sorted in non-decreasing order,
// and two integers m and n, representing the number of elements in nums1 and nums2 respectively.

// Merge nums1 and nums2 into a single array sorted in non-decreasing order.

// The final sorted array should not be returned by the function,
//  but instead be stored inside the array nums1.
// To accommodate this, nums1 has a length of m + n,
//  where the first m elements denote the elements that should be merged,
//  and the last n elements are set to 0 and should be ignored.
//  nums2 has a length of n.
/* ------------------------------------ x ----------------------------------- */
// try to come up with an algorithm that runs in O(m + n) time

use std::cmp::Ordering;

// m - length of arr1
// n - length of arr2
// Worst time: O(n + m*m) (custom_insert time: O(m)) 
pub fn merge_sorted_arrays_v2(arr1: &mut Vec<i32>, m: usize, arr2: &mut Vec<i32>, n: usize) {
    let mut zeros_start = m - n;
    let mut p1 = 0;
    arr2.iter().for_each(|to_insert| {
        for elem1 in arr1.iter_mut().skip(p1) {
            if *elem1 < *to_insert && p1 < zeros_start {
                p1 += 1;
            } else if p1 < zeros_start {
                custom_insert(arr1, *to_insert, p1, zeros_start);
                p1 += 1;
                zeros_start += 1;
                break;
            } else {
                *elem1 = *to_insert; // copy elems from arr2 to arr1's 0,0,0,..
                p1 += 1;
                zeros_start += 1;
                break;
            }
        }
    });
    println!("{arr1:?}");
}

// idea: use empty part of nums1 to store larger value conditionally based on comparison
// compare (nums1's last elemn , nums2's last elem) => store the larger elem at end of nums1
// m+n-1 keep track of where to store the larger elem 
fn merge_sorted_arrays(nums1: &mut Vec<i32>, m: usize, nums2: &mut Vec<i32>, n: usize) {
    let mut m = m - n;
    let mut n = n;

    while n > 0 {
        if m > 0 && nums1[m - 1] > nums2[n - 1] {
            nums1[m + n - 1] = nums1[m - 1];
            m -= 1;
        } else {
            nums1[m + n - 1] = nums2[n - 1];
            n -= 1;
        }
    }
}

fn custom_insert(arr1: &mut [i32], insert: i32, curr_pos: usize, zeros_start: usize) {
    // swap btw insert and curr
    let original = arr1[curr_pos];
    arr1[curr_pos] = insert;

    // swap btw arr1's curr and next elem
    let mut curr = original;
    let mut id = curr_pos + 1;
    while id <= zeros_start {
        std::mem::swap(&mut arr1[id], &mut curr);
        id += 1;
    }
}

#[test]
fn testing() {
    let mut arr1 = vec![-1, 0, 0, 3, 3, 3, 0, 0, 0];
    let mut arr2 = vec![1, 2, 2];
    let m = arr1.len();
    let n = arr2.len();

    println!("Before: {arr1:?} and {arr2:?}");
    merge_sorted_arrays(&mut arr1, m, &mut arr2, n);
    println!("After: {arr1:?}");
    assert_eq!(arr1, vec![-1, 0, 0, 1, 2, 2, 3, 3, 3]);

    let mut arr1 = vec![1, 2, 3, 0, 0, 0];
    let mut arr2 = vec![2, 5, 6];
    let m = arr1.len();
    let n = arr2.len();

    println!("Before: {arr1:?} and {arr2:?}");
    merge_sorted_arrays(&mut arr1, m, &mut arr2, n);
    println!("After: {arr1:?}");
    assert_eq!(arr1, vec![1, 2, 2, 3, 5, 6]);

    let mut arr1 = vec![0];
    let mut arr2 = vec![1];
    let m = arr1.len();
    let n = arr2.len();

    println!("Before: {arr1:?} and {arr2:?}");
    merge_sorted_arrays(&mut arr1, m, &mut arr2, n);
    println!("After: {arr1:?}");
    assert_eq!(arr1, vec![1]);

    let mut arr1 = vec![4, 0, 0, 0, 0, 0];
    let mut arr2 = vec![1, 2, 3, 5, 6];
    let m = arr1.len();
    let n = arr2.len();

    println!("Before: {arr1:?} and {arr2:?}");
    merge_sorted_arrays(&mut arr1, m, &mut arr2, n);
    println!("After: {arr1:?}");
    assert_eq!(arr1, vec![1, 2, 3, 4, 5, 6]);

    let mut arr1 = vec![0, 0, 3, 0, 0, 0, 0, 0, 0];
    let mut arr2 = vec![-1, 1, 1, 1, 2, 3];
    let m = arr1.len();
    let n = arr2.len();

    println!("Before: {arr1:?} and {arr2:?}");
    merge_sorted_arrays(&mut arr1, m, &mut arr2, n);
    println!("After: {arr1:?}");
    assert_eq!(arr1, vec![-1, 0, 0, 1, 1, 1, 2, 3, 3]);
}
