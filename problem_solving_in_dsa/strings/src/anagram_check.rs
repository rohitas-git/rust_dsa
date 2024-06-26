use ps_helper::Counter;

// given input strings only consist of lowercase english letters

// main idea:
// - 1. rearrange both strings 
// - 2. check if rearrangements are the same

// Ways to do rearrangement:
// - HashMap : two map strategy
// - Sorting : sorting strategy

// insight: freq count of each aplhabet in anagrams should be same
// time: O(s + t) => O(s+s) => O(s)
// space: O(s + t) => O(s+s) => O(s) => O(26) => O(1)
fn is_anagram(original: &str, test_str: &str) -> bool {
    if test_str.len() != original.len() {
        return false;
    }

    let original_map = Counter::new(original.chars());
    let test_str_map = Counter::new(test_str.chars());

    for (letter, count) in &original_map {
        if test_str_map.get(letter).unwrap() != count {
            return false;
        }
    }

    true
}

// insight: two anagrams should perfectly cancel each other out
// comment: not as good as two map above solution due to mental clarity
fn is_anagram_s2(original: &str, test_str: &str) -> bool {
    if test_str.len() != original.len() {
        return false;
    }
    let mut original_map = Counter::new(original.chars());

    for letter in &mut test_str.chars() {
        if let Some(val) = original_map.get_mut(&letter) {
            *val -= 1;
        }
    }

    for count in original_map.values(){
        if *count != 0 {
            return  false
        }
    }
    true
}


// time - O(s*log(s)) if sorted by heap sort 
// space - O(s+t) => O(s) if strings are immutable | O(1) if string are mutable in place
fn is_anagram_s3(original: &str, test_str: &str) -> bool{
    if test_str.len() != original.len(){
        return false;
    }

    // I can not sort strings in-place by default: => O(s*log(s)), O(s)
    let mut sorted_s = original.chars().map(|c| c as u32).collect::<Vec<u32>>();
    let mut sorted_t = test_str.chars().map(|c| c as u32).collect::<Vec<u32>>();
    sorted_s.sort();
    sorted_t.sort();

    sorted_s == sorted_t
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_anagram_check() {
        let t = "anagram";
        let s = "nagrama";
        let s2 = "ngrm";

        assert!(is_anagram(t, s));
        assert!(!is_anagram(t, s2));
    }
}
