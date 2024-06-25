use custom_help::Counter;

// given input strings only consist of lowercase english letters

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
fn is_anagram_s2(original: &str, test_str: &str) -> bool{
    if test_str.len() != original.len() {
        return false;
    }

    let mut original_map = Counter::new(original.chars());

    for (letter, count) in &mut original_map {
        
    }

    true

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
