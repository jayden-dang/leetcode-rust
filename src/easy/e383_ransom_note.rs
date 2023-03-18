#![allow(dead_code)]
struct Solution;
use std::collections::HashMap;

impl Solution {
    fn can_construct(ransom_note: String, magazine: String) -> bool {
        let mut char_freq: HashMap<char, i32> = HashMap::new();

        // Count the frequency of each character in the magazine
        for c in magazine.chars() {
            *char_freq.entry(c).or_insert(0) += 1;
        }

        // Check if each character in the ransom note can be constructed
        for c in ransom_note.chars() {
            if let Some(freq) = char_freq.get_mut(&c) {
                if *freq > 0 {
                    *freq -= 1;
                } else {
                    return false;
                }
            } else {
                return false;
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chars_3() {
        assert!(Solution::can_construct("aa".to_string(), "aab".to_string()));
    }

    #[test]
    #[should_panic]
    fn chars_1() {
        assert!(Solution::can_construct("a".to_string(), "b".to_string()));
    }

    #[test]
    #[should_panic]
    fn chars_2() {
        assert!(Solution::can_construct("aa".to_string(), "ab".to_string()));
    }
}
