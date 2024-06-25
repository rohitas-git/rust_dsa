/* ----------------------------- Remove elements ---------------------------- */

// Given an integer array nums and an integer val,
// remove all occurrences of val in nums in-place.
// The order of the elements may be changed.
// Then return the number of elements in nums which are not equal to val.

/* ------------------------------------ x ----------------------------------- */

use core::num;
use std::mem::swap;

// swap remove soln
fn remove_elements(nums: &mut Vec<i32>, val: i32) -> i32 {
    if nums.is_empty(){
        return 0;
    }
    if nums.len() == 1 {
        if nums[0] == val{
            return 0;
        }else{
            return 1;
        }
    }
    let mut num_elems = 0; // count elems != val
    
    let mut front_ptr = 0;
    let mut back_ptr = nums.len() - 1;
    // let mut front_elem = nums[front_ptr];
    // let mut back_elem = nums[back_ptr];

    while front_ptr < back_ptr {
        // println!("nums: {nums:?}");
        // println!("1. f:{front_ptr},  b:{back_ptr}");

        // only swap when front_elem == val && back_elem != val
        if nums[front_ptr] == val && nums[back_ptr] != val{
            nums.swap(front_ptr, back_ptr);
            front_ptr+=1;
            back_ptr-=1;
            num_elems+=1; // back_elem != val
            // println!("2. after swap, nums: {nums:?}");
            // println!("2. f:{front_ptr},  b:{back_ptr}");
        }else if nums[front_ptr] != val{
            num_elems+=1; // front_elem != val
            front_ptr+=1;
        }else if nums[back_ptr] == val{
            back_ptr-=1;
        }
    }
    // println!("3. f:{front_ptr},  b:{back_ptr} non-vals:{num_elems}");
    // println!("3. after while loop, nums: {nums:?}");
    // println!("5. here");
    if nums[front_ptr] != val{
        num_elems + 1
    }else{
        num_elems
    }
}

fn remove_elements_v2(nums: &mut Vec<i32>, val: i32) -> i32 {
    let (mut b, mut f) = (nums.len(), 0);
    while f < b {
        if nums[f] == val {
            nums.swap(f, b - 1);
            b -= 1;
        } else {
            f += 1;
        }
    } 
    b as _
}



#[test]
fn test_sol1() {
    let mut nums = vec![3, 2, 2, 3];
    let val = 3;
    let k = remove_elements(&mut nums, val);
    // assert_eq!(k, 2);

    let mut nums = vec![0, 1, 2, 2, 3, 0, 4, 2];
    let val = 2;
    let k = remove_elements(&mut nums, val);
    assert_eq!(k, 5);

    let mut nums = vec![1, 1, 1, 1];
    let val = 1;
    let k = remove_elements(&mut nums, val);
    assert_eq!(k, 0);

    let mut nums = vec![];
    let val = 0;
    let k = remove_elements(&mut nums, val);
    assert_eq!(k, 0);

    let mut nums = vec![4,5];
    let val = 4;
    let k = remove_elements(&mut nums, val);
    assert_eq!(k, 1);

    let mut nums = vec![4,5];
    let val = 5;
    let k = remove_elements(&mut nums, val);
    assert_eq!(k, 1);

    let mut nums = vec![5];
    let val = 5;
    let k = remove_elements(&mut nums, val);
    assert_eq!(k, 0);

    let mut nums = vec![4];
    let val = 5;
    let k = remove_elements(&mut nums, val);
    assert_eq!(k, 1);
}
